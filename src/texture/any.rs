use gl;
use GlObject;

use backend::Facade;
use version::Version;
use context::Context;
use ContextExt;
use TextureExt;
use version::Api;
use Rect;

use pixel_buffer::PixelBuffer;
use image_format::{self, TextureFormatRequest, ClientFormatAny};
use texture::Texture2dDataSink;
use texture::{MipmapsOption, TextureFormat};
use texture::{TextureCreationError, TextureMaybeSupportedCreationError};
use texture::{get_format, InternalFormat};

use buffer::BufferViewAny;
use BufferViewExt;

use libc;
use std::cmp;
use std::fmt;
use std::mem;
use std::ptr;
use std::borrow::Cow;
use std::cell::Cell;
use std::rc::Rc;

use ops;
use fbo;

/// A texture whose type isn't fixed at compile-time.
pub struct TextureAny {
    context: Rc<Context>,
    id: gl::types::GLuint,
    requested_format: TextureFormatRequest,

    /// Cache for the actual format of the texture. The outer Option is None if the format hasn't
    /// been checked yet. The inner Option is None if the format has been checkek but is unknown.
    actual_format: Cell<Option<Option<InternalFormat>>>,

    bind_point: gl::types::GLenum,
    ty: TextureType,
    width: u32,
    height: Option<u32>,
    depth: Option<u32>,
    array_size: Option<u32>,

    /// Number of mipmap levels (`1` means just the main texture, `0` is not valid)
    levels: u32,
    /// Is automatic mipmap generation allowed for this texture?
    generate_mipmaps: bool,
}

/// Represents a specific mipmap of a texture.
#[derive(Copy, Clone)]
pub struct TextureAnyMipmap<'a> {
    /// The texture.
    texture: &'a TextureAny,

    /// Layer for array textures, or 0 for other textures.
    layer: u32,

    /// Mipmap level.
    level: u32,

    /// Width of this mipmap level.
    width: u32,
    /// Height of this mipmap level.
    height: Option<u32>,
    /// Depth of this mipmap level.
    depth: Option<u32>,
}

/// Type of a texture.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]      // TODO: document and remove
pub enum TextureType {
    Texture1d,
    Texture1dArray,
    Texture2d,
    Texture2dArray,
    Texture2dMultisample,
    Texture2dMultisampleArray,
    Texture3d,
}

/// Builds a new texture.
pub fn new_texture<'a, F, P>(facade: &F, format: TextureFormatRequest,
                             data: Option<(ClientFormatAny, Cow<'a, [P]>)>,
                             mipmaps: MipmapsOption,
                             width: u32, height: Option<u32>, depth: Option<u32>,
                             array_size: Option<u32>, samples: Option<u32>)
                             -> Result<TextureAny, TextureMaybeSupportedCreationError>
                             where P: Send + Clone + 'a, F: Facade
{
    let (is_client_compressed, data_bufsize) = match data {
        Some((client_format, _)) => {
            (client_format.is_compressed(),
             client_format.get_buffer_size(width, height, depth, array_size))
        },
        None => (false, 0),
    };

    if let Some((_, ref data)) = data {
        if data.len() * mem::size_of::<P>() != data_bufsize
        {
            panic!("Texture data size mismatch");
        }
    }

    // checking non-power-of-two
    if facade.get_context().get_version() < &Version(Api::Gl, 2, 0) &&
        !facade.get_context().get_extensions().gl_arb_texture_non_power_of_two
    {
        if !width.is_power_of_two() || !height.unwrap_or(2).is_power_of_two() ||
            !depth.unwrap_or(2).is_power_of_two() || !array_size.unwrap_or(2).is_power_of_two()
        {
            let ce = TextureCreationError::DimensionsNotSupported;
            return Err(TextureMaybeSupportedCreationError::CreationError(ce));
        }
    }

    let (stored_ty, texture_type) = if height.is_none() && depth.is_none() {
        assert!(samples.is_none());
        if array_size.is_none() {
            (TextureType::Texture1d, gl::TEXTURE_1D)
        } else {
            (TextureType::Texture1dArray, gl::TEXTURE_1D_ARRAY)
        }

    } else if depth.is_none() {
        match (array_size.is_some(), samples.is_some()) {
            (false, false) => (TextureType::Texture2d, gl::TEXTURE_2D),
            (true, false) => (TextureType::Texture2dArray, gl::TEXTURE_2D_ARRAY),
            (false, true) => (TextureType::Texture2dMultisample, gl::TEXTURE_2D_MULTISAMPLE),
            (true, true) => (TextureType::Texture2dMultisampleArray, gl::TEXTURE_2D_MULTISAMPLE_ARRAY),
        }

    } else {
        assert!(samples.is_none());
        (TextureType::Texture3d, gl::TEXTURE_3D)
    };

    let generate_mipmaps = mipmaps.should_generate();
    let texture_levels = mipmaps.num_levels(width, height, depth) as gl::types::GLsizei;

    let (teximg_internal_format, storage_internal_format) =
        try!(image_format::format_request_to_glenum(facade.get_context(), data.as_ref().map(|&(c, _)| c), format));

    let (client_format, client_type) = match (&data, format) {
        (&Some((client_format, _)), f) => try!(image_format::client_format_to_glenum(facade.get_context(), client_format, f)),
        (&None, TextureFormatRequest::AnyDepth) => (gl::DEPTH_COMPONENT, gl::FLOAT),
        (&None, TextureFormatRequest::Specific(TextureFormat::DepthFormat(_))) => (gl::DEPTH_COMPONENT, gl::FLOAT),
        (&None, TextureFormatRequest::AnyDepthStencil) => (gl::DEPTH_STENCIL, gl::UNSIGNED_INT_24_8),
        (&None, TextureFormatRequest::Specific(TextureFormat::DepthStencilFormat(_))) => (gl::DEPTH_STENCIL, gl::UNSIGNED_INT_24_8),
        (&None, _) => (gl::RGBA, gl::UNSIGNED_BYTE),
    };

    let mut ctxt = facade.get_context().make_current();

    let id = unsafe {
        let has_mipmaps = texture_levels > 1;
        let data = data;
        let data_raw = if let Some((_, ref data)) = data {
            data.as_ptr() as *const libc::c_void
        } else {
            ptr::null()
        };

        if ctxt.state.pixel_store_unpack_alignment != 1 {
            ctxt.state.pixel_store_unpack_alignment = 1;
            ctxt.gl.PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        BufferViewAny::unbind_pixel_unpack(&mut ctxt);

        let id: gl::types::GLuint = mem::uninitialized();
        ctxt.gl.GenTextures(1, mem::transmute(&id));

        {
            ctxt.gl.BindTexture(texture_type, id);
            let act = ctxt.state.active_texture as usize;
            ctxt.state.texture_units[act].texture = id;
        }

        ctxt.gl.TexParameteri(texture_type, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        if height.is_some() || depth.is_some() || array_size.is_some() {
            ctxt.gl.TexParameteri(texture_type, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        }
        if depth.is_some() || array_size.is_some() {
            ctxt.gl.TexParameteri(texture_type, gl::TEXTURE_WRAP_R, gl::REPEAT as i32);
        }
        ctxt.gl.TexParameteri(texture_type, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        if has_mipmaps {
            ctxt.gl.TexParameteri(texture_type, gl::TEXTURE_MIN_FILTER,
                                  gl::LINEAR_MIPMAP_LINEAR as i32);
        } else {
            ctxt.gl.TexParameteri(texture_type, gl::TEXTURE_MIN_FILTER,
                                  gl::LINEAR as i32);
        }

        if !has_mipmaps && (ctxt.version >= &Version(Api::Gl, 1, 2) ||
                            ctxt.version >= &Version(Api::GlEs, 3, 0))
        {
            ctxt.gl.TexParameteri(texture_type, gl::TEXTURE_BASE_LEVEL, 0);
            ctxt.gl.TexParameteri(texture_type, gl::TEXTURE_MAX_LEVEL, 0);
        }

        if texture_type == gl::TEXTURE_3D || texture_type == gl::TEXTURE_2D_ARRAY {
            let mut data_raw = data_raw;

            let width = match width as gl::types::GLsizei {
                0 => { data_raw = ptr::null(); 1 },
                a => a
            };

            let height = match height.unwrap() as gl::types::GLsizei {
                0 => { data_raw = ptr::null(); 1 },
                a => a
            };

            let depth = match depth.or(array_size).unwrap() as gl::types::GLsizei {
                0 => { data_raw = ptr::null(); 1 },
                a => a
            };

            if storage_internal_format.is_some() && (ctxt.version >= &Version(Api::Gl, 4, 2) || ctxt.extensions.gl_arb_texture_storage) {
                ctxt.gl.TexStorage3D(texture_type, texture_levels,
                                     storage_internal_format.unwrap() as gl::types::GLenum,
                                     width, height, depth);

                if !data_raw.is_null() {
                    if is_client_compressed {
                        ctxt.gl.CompressedTexSubImage3D(texture_type, 0, 0, 0, 0, width, height, depth,
                                                         teximg_internal_format as u32,
                                                         data_bufsize as i32, data_raw);
                    } else {
                        ctxt.gl.TexSubImage3D(texture_type, 0, 0, 0, 0, width, height, depth,
                                              client_format, client_type, data_raw);
                    }
                }

            } else {
                if is_client_compressed && !data_raw.is_null() {
                    ctxt.gl.CompressedTexImage3D(texture_type, 0, teximg_internal_format as u32, 
                                       width, height, depth, 0, data_bufsize as i32, data_raw);
                } else {
                    ctxt.gl.TexImage3D(texture_type, 0, teximg_internal_format as i32, width,
                                       height, depth, 0, client_format as u32, client_type,
                                       data_raw);
                }
            }

        } else if texture_type == gl::TEXTURE_2D || texture_type == gl::TEXTURE_1D_ARRAY {
            let mut data_raw = data_raw;

            let width = match width as gl::types::GLsizei {
                0 => { data_raw = ptr::null(); 1 },
                a => a
            };

            let height = match height.or(array_size).unwrap() as gl::types::GLsizei {
                0 => { data_raw = ptr::null(); 1 },
                a => a
            };

            if storage_internal_format.is_some() && (ctxt.version >= &Version(Api::Gl, 4, 2) || ctxt.extensions.gl_arb_texture_storage) {
                ctxt.gl.TexStorage2D(texture_type, texture_levels,
                                     storage_internal_format.unwrap() as gl::types::GLenum,
                                     width, height);

                if !data_raw.is_null() {
                    if is_client_compressed {
                        ctxt.gl.CompressedTexSubImage2D(texture_type, 0, 0, 0, width, height,
                                                         teximg_internal_format as u32,
                                                         data_bufsize as i32, data_raw);
                    } else {
                        ctxt.gl.TexSubImage2D(texture_type, 0, 0, 0, width, height, client_format,
                                              client_type, data_raw);
                    }
                }

            } else {
                if is_client_compressed && !data_raw.is_null() {
                    ctxt.gl.CompressedTexImage2D(texture_type, 0, teximg_internal_format as u32, 
                                       width, height, 0, data_bufsize as i32, data_raw);
                } else {
                    ctxt.gl.TexImage2D(texture_type, 0, teximg_internal_format as i32, width,
                                       height, 0, client_format as u32, client_type, data_raw);
                }
            }

        } else if texture_type == gl::TEXTURE_2D_MULTISAMPLE {
            assert!(data_raw.is_null());

            let width = match width as gl::types::GLsizei {
                0 => 1,
                a => a
            };

            let height = match height.unwrap() as gl::types::GLsizei {
                0 => 1,
                a => a
            };

            if storage_internal_format.is_some() && (ctxt.version >= &Version(Api::Gl, 4, 2) || ctxt.extensions.gl_arb_texture_storage) {
                ctxt.gl.TexStorage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE,
                                                samples.unwrap() as gl::types::GLsizei,
                                                storage_internal_format.unwrap() as gl::types::GLenum,
                                                width, height, gl::TRUE);

            } else if ctxt.version >= &Version(Api::Gl, 3, 2) || ctxt.extensions.gl_arb_texture_multisample {
                ctxt.gl.TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE,
                                              samples.unwrap() as gl::types::GLsizei,
                                              teximg_internal_format as gl::types::GLenum,
                                              width, height, gl::TRUE);

            } else {
                unreachable!();
            }

        } else if texture_type == gl::TEXTURE_2D_MULTISAMPLE_ARRAY {
            assert!(data_raw.is_null());

            let width = match width as gl::types::GLsizei {
                0 => 1,
                a => a
            };

            let height = match height.unwrap() as gl::types::GLsizei {
                0 => 1,
                a => a
            };

            if storage_internal_format.is_some() && (ctxt.version >= &Version(Api::Gl, 4, 2) || ctxt.extensions.gl_arb_texture_storage) {
                ctxt.gl.TexStorage3DMultisample(gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
                                                samples.unwrap() as gl::types::GLsizei,
                                                storage_internal_format.unwrap() as gl::types::GLenum,
                                                width, height, array_size.unwrap() as gl::types::GLsizei,
                                                gl::TRUE);

            } else if ctxt.version >= &Version(Api::Gl, 3, 2) || ctxt.extensions.gl_arb_texture_multisample {
                ctxt.gl.TexImage3DMultisample(gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
                                              samples.unwrap() as gl::types::GLsizei,
                                              teximg_internal_format as gl::types::GLenum,
                                              width, height, array_size.unwrap() as gl::types::GLsizei,
                                              gl::TRUE);

            } else {
                unreachable!();
            }

        } else if texture_type == gl::TEXTURE_1D {
            let mut data_raw = data_raw;

            let width = match width as gl::types::GLsizei {
                0 => { data_raw = ptr::null(); 1 },
                a => a
            };

            if storage_internal_format.is_some() && (ctxt.version >= &Version(Api::Gl, 4, 2) || ctxt.extensions.gl_arb_texture_storage) {
                ctxt.gl.TexStorage1D(texture_type, texture_levels,
                                     storage_internal_format.unwrap() as gl::types::GLenum,
                                     width);

                if !data_raw.is_null() {
                    if is_client_compressed {
                        ctxt.gl.CompressedTexSubImage1D(texture_type, 0, 0, width,
                                                         teximg_internal_format as u32,
                                                         data_bufsize as i32, data_raw);
                    } else {
                        ctxt.gl.TexSubImage1D(texture_type, 0, 0, width, client_format,
                                              client_type, data_raw);
                    }
                }

            } else {
                if is_client_compressed && !data_raw.is_null() {
                    ctxt.gl.CompressedTexImage1D(texture_type, 0, teximg_internal_format as u32, 
                                       width, 0, data_bufsize as i32, data_raw);
                } else {
                    ctxt.gl.TexImage1D(texture_type, 0, teximg_internal_format as i32, width,
                                       0, client_format as u32, client_type, data_raw);
                }
            }

        } else {
            unreachable!();
        }

        // only generate mipmaps for color textures
        if generate_mipmaps {
            if ctxt.version >= &Version(Api::Gl, 3, 0) ||
               ctxt.version >= &Version(Api::GlEs, 2, 0)
            {
                ctxt.gl.GenerateMipmap(texture_type);
            } else if ctxt.extensions.gl_ext_framebuffer_object {
                ctxt.gl.GenerateMipmapEXT(texture_type);
            } else {
                unreachable!();
            }
        }

        id
    };

    Ok(TextureAny {
        context: facade.get_context().clone(),
        id: id,
        requested_format: format,
        actual_format: Cell::new(None),
        bind_point: texture_type,
        width: width,
        height: height,
        depth: depth,
        array_size: array_size,
        ty: stored_ty,
        levels: texture_levels as u32,
        generate_mipmaps: generate_mipmaps,
    })
}

impl<'a> TextureAnyMipmap<'a> {
    /// Returns the texture.
    pub fn get_texture(&self) -> &'a TextureAny {
        self.texture
    }

    /// Returns the level of the texture.
    pub fn get_level(&self) -> u32 {
        self.level
    }

    /// Returns the layer of the texture.
    pub fn get_layer(&self) -> u32 {
        self.layer
    }
}

/// Changes some parts of the texture.
pub fn upload_texture<'a, P>(mip: &TextureAnyMipmap, x_offset: u32, y_offset: u32, z_offset: u32,
                             (format, data): (ClientFormatAny, Cow<'a, [P]>), width: u32,
                             height: Option<u32>, depth: Option<u32>,
                             regen_mipmaps: bool)
                            -> Result<(), ()>   // TODO return a better Result!?
                             where P: Send + Copy + Clone + 'a
{
    let id = mip.texture.id;
    let bind_point = mip.texture.bind_point;
    let level = mip.level;

    let (is_client_compressed, data_bufsize) = (format.is_compressed(),
                                                format.get_buffer_size(width, height, depth, None));
    let regen_mipmaps = regen_mipmaps && mip.texture.levels >= 2 &&
                        mip.texture.generate_mipmaps && !is_client_compressed;

    assert!(!regen_mipmaps || level == 0);  // when regen_mipmaps is true, level must be 0!
    assert!(x_offset <= mip.width);
    assert!(y_offset <= mip.height.unwrap_or(1));
    assert!(z_offset <= mip.depth.unwrap_or(1));
    assert!(x_offset + width <= mip.width);
    assert!(y_offset + height.unwrap_or(1) <= mip.height.unwrap_or(1));
    assert!(z_offset + depth.unwrap_or(1) <= mip.depth.unwrap_or(1));

    if data.len() * mem::size_of::<P>() != data_bufsize
    {
        panic!("Texture data size mismatch");
    }

    let (client_format, client_type) = try!(image_format::client_format_to_glenum(&mip.texture.context,
                                                                                  format,
                                                                                  mip.texture.requested_format)
                                                                                  .map_err(|_| ()));

    let mut ctxt = mip.texture.context.make_current();

    unsafe {
        if ctxt.state.pixel_store_unpack_alignment != 1 {
            ctxt.state.pixel_store_unpack_alignment = 1;
            ctxt.gl.PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        BufferViewAny::unbind_pixel_unpack(&mut ctxt);

        {
            ctxt.gl.BindTexture(bind_point, id);
            let act = ctxt.state.active_texture as usize;
            ctxt.state.texture_units[act].texture = id;
        }

        if bind_point == gl::TEXTURE_3D || bind_point == gl::TEXTURE_2D_ARRAY {
            unimplemented!();

        } else if bind_point == gl::TEXTURE_2D || bind_point == gl::TEXTURE_1D_ARRAY {
            assert!(z_offset == 0);
            // FIXME should glTexImage be used here somewhere or glTexSubImage does it just fine?
            if is_client_compressed {
                ctxt.gl.CompressedTexSubImage2D(bind_point, level as gl::types::GLint,
                                                x_offset as gl::types::GLint,
                                                y_offset as gl::types::GLint,
                                                width as gl::types::GLsizei,
                                                height.unwrap_or(1) as gl::types::GLsizei,
                                                client_format,
                                                data_bufsize  as gl::types::GLsizei,
                                                data.as_ptr() as *const libc::c_void);
            } else {
                ctxt.gl.TexSubImage2D(bind_point, level as gl::types::GLint,
                                      x_offset as gl::types::GLint,
                                      y_offset as gl::types::GLint,
                                      width as gl::types::GLsizei,
                                      height.unwrap_or(1) as gl::types::GLsizei,
                                      client_format, client_type,
                                      data.as_ptr() as *const libc::c_void);
            }

        } else {
            assert!(z_offset == 0);
            assert!(y_offset == 0);

            unimplemented!();
        }

        // regenerate mipmaps if there are some
        if regen_mipmaps {
            if ctxt.version >= &Version(Api::Gl, 3, 0) {
                ctxt.gl.GenerateMipmap(bind_point);
            } else {
                ctxt.gl.GenerateMipmapEXT(bind_point);
            }
        }

        Ok(())
    }
}

pub fn download_compressed_data(mip: &TextureAnyMipmap) -> Option<(ClientFormatAny, Vec<u8>)> {
    let texture = mip.texture;
    let level = mip.level as i32;

    let mut ctxt = texture.context.make_current();

    unsafe {
        let bind_point = get_bind_point(texture);
        ctxt.gl.BindTexture(bind_point, texture.get_id());

        let mut is_compressed = mem::uninitialized();
        ctxt.gl.GetTexLevelParameteriv(bind_point, level, gl::TEXTURE_COMPRESSED, &mut is_compressed);
        if is_compressed != 0 {

            let mut buffer_size = mem::uninitialized();
            ctxt.gl.GetTexLevelParameteriv(bind_point, level, gl::TEXTURE_COMPRESSED_IMAGE_SIZE, &mut buffer_size);
            let mut internal_format = mem::uninitialized();
            ctxt.gl.GetTexLevelParameteriv(bind_point, level, gl::TEXTURE_INTERNAL_FORMAT, &mut internal_format);
            
            match ClientFormatAny::from_internal_compressed_format(internal_format as gl::types::GLenum) {
                Some(known_format) => {
                    let mut buf = Vec::with_capacity(buffer_size as usize);
                    buf.set_len(buffer_size as usize);

                    BufferViewAny::unbind_pixel_pack(&mut ctxt);
                    
                    // adjusting data alignement
                    let ptr = buf.as_ptr() as *const u8;
                    let ptr = ptr as usize;
                    if (ptr % 8) == 0 {
                    } else if (ptr % 4) == 0 && ctxt.state.pixel_store_pack_alignment != 4 {
                        ctxt.state.pixel_store_pack_alignment = 4;
                        ctxt.gl.PixelStorei(gl::PACK_ALIGNMENT, 4);
                    } else if (ptr % 2) == 0 && ctxt.state.pixel_store_pack_alignment > 2 {
                        ctxt.state.pixel_store_pack_alignment = 2;
                        ctxt.gl.PixelStorei(gl::PACK_ALIGNMENT, 2);
                    } else if ctxt.state.pixel_store_pack_alignment != 1 {
                        ctxt.state.pixel_store_pack_alignment = 1;
                        ctxt.gl.PixelStorei(gl::PACK_ALIGNMENT, 1);
                    }

                    ctxt.gl.GetCompressedTexImage(bind_point, level, buf.as_mut_ptr() as *mut _);
                    Some((known_format, buf))
                },
                None => None,
            }

        } else {
            None
        }
    }
}

/// Returns the `Context` associated with this texture.
pub fn get_context(tex: &TextureAny) -> &Rc<Context> {
    &tex.context
}

/// Returns the bind point of this texture.
///
/// The returned GLenum is guaranteed to be supported by the context.
pub fn get_bind_point(tex: &TextureAny) -> gl::types::GLenum {
    tex.bind_point
}

impl TextureAny {
    /// UNSTABLE. Reads the content of a mipmap level of the texture.
    // TODO: this function only works for level 0 right now
    //       width/height need adjustements
    pub fn read<T>(&self, level: u32) -> T
                   where T: Texture2dDataSink<(u8, u8, u8, u8)>
            // TODO: remove Clone for P
    {
        assert_eq!(level, 0);   // TODO:

        let attachment = fbo::Attachment::TextureLayer {
            texture: self,
            layer: 0,
            level: 0,
        };

        let rect = Rect {
            bottom: 0,
            left: 0,
            width: self.width,
            height: self.height.unwrap_or(1),
        };

        let mut ctxt = self.context.make_current();

        let mut data = Vec::with_capacity(0);
        ops::read(&mut ctxt, &attachment, &rect, &mut data);
        T::from_raw(Cow::Owned(data), self.width, self.height.unwrap_or(1))
    }

    /// UNSTABLE. Reads the content of a mipmap level of the texture to a pixel buffer.
    // TODO: this function only works for level 0 right now
    //       width/height need adjustements
    pub fn read_to_pixel_buffer(&self, level: u32) -> PixelBuffer<(u8, u8, u8, u8)> {
        assert_eq!(level, 0);   // TODO:

        let size = self.width as usize * self.height.unwrap_or(1) as usize * 4;

        let attachment = fbo::Attachment::TextureLayer {
            texture: self,
            layer: 0,
            level: 0,
        };

        let rect = Rect {
            bottom: 0,
            left: 0,
            width: self.width,
            height: self.height.unwrap_or(1),
        };

        let pb = PixelBuffer::new_empty(&self.context, size);

        let mut ctxt = self.context.make_current();
        ops::read(&mut ctxt, &attachment, &rect, &pb);
        pb
    }

    /// UNSTABLE. Returns the `Context` associated with this texture.
    pub fn get_context(&self) -> &Rc<Context> {
        &self.context
    }

    /// Returns the width of the texture.
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the texture.
    pub fn get_height(&self) -> Option<u32> {
        self.height.clone()
    }

    /// Returns the depth of the texture.
    pub fn get_depth(&self) -> Option<u32> {
        self.depth.clone()
    }

    /// Returns the array size of the texture.
    pub fn get_array_size(&self) -> Option<u32> {
        self.array_size.clone()
    }

    /// Returns the number of mipmap levels of the texture.
    pub fn get_mipmap_levels(&self) -> u32 {
        self.levels
    }

    /// Returns the type of the texture (1D, 2D, 3D, etc.).
    pub fn get_texture_type(&self) -> TextureType {
        self.ty
    }

    /// Determines the internal format of this texture.
    ///
    /// Returns `None` if the backend doesn't allow querying the actual format.
    pub fn get_internal_format_if_supported(&self) -> Option<InternalFormat> {
        if let Some(format) = self.actual_format.get() {
            format

        } else {
            let mut ctxt = self.context.make_current();
            let format = get_format::get_format_if_supported(&mut ctxt, self);
            self.actual_format.set(Some(format.clone()));
            format
        }
    }

    /// Returns a structure that represents a specific mipmap of the texture.
    ///
    /// Returns `None` if out of range.
    pub fn mipmap(&self, layer: u32, level: u32) -> Option<TextureAnyMipmap> {
        if layer >= self.array_size.unwrap_or(1) {
            return None;
        }

        if level >= self.levels {
            return None;
        }

        let pow = 2u32.pow(level);
        Some(TextureAnyMipmap {
            texture: self,
            level: level,
            layer: layer,
            width: cmp::max(1, self.width / pow),
            height: self.height.map(|height| cmp::max(1, height / pow)),
            depth: self.depth.map(|depth| cmp::max(1, depth / pow)),
        })
    }
}

impl TextureExt for TextureAny {
    fn get_bind_point(&self) -> gl::types::GLenum {
        self.bind_point
    }
}

impl GlObject for TextureAny {
    type Id = gl::types::GLuint;
    fn get_id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl fmt::Debug for TextureAny {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "Texture #{} (dimensions: {}x{}x{}x{})", self.id,
               self.width, self.height.unwrap_or(1), self.depth.unwrap_or(1),
               self.array_size.unwrap_or(1))
    }
}

impl Drop for TextureAny {
    fn drop(&mut self) {
        let mut ctxt = self.context.make_current();

        // removing FBOs which contain this texture
        fbo::FramebuffersContainer::purge_texture(&mut ctxt, self.id);

        // resetting the bindings
        for tex_unit in ctxt.state.texture_units.iter_mut() {
            if tex_unit.texture == self.id {
                tex_unit.texture = 0;
            }
        }

        unsafe { ctxt.gl.DeleteTextures(1, [ self.id ].as_ptr()); }
    }
}
