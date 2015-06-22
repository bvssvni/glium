use std::io::Write;

#[derive(PartialEq, Eq)]
enum TextureType {
    Regular,
    Compressed,
    Srgb,
    CompressedSrgb,
    Integral,
    Unsigned,
    Depth,
    Stencil,
    DepthStencil,
}

#[derive(PartialEq, Eq)]
enum TextureDimensions {
    Texture1d,
    Texture2d,
    Texture2dMultisample,
    Texture3d,
    Texture1dArray,
    Texture2dArray,
    Texture2dMultisampleArray,
}

impl TextureDimensions {
    fn is_array(&self) -> bool {
        match self {
            &TextureDimensions::Texture1dArray => true,
            &TextureDimensions::Texture2dArray => true,
            _ => false
        }
    }

    fn is_multisample(&self) -> bool {
        match self {
            &TextureDimensions::Texture2dMultisample => true,
            &TextureDimensions::Texture2dMultisampleArray => true,
            _ => false
        }
    }
}

pub fn build_texture_file<W: Write>(mut dest: &mut W) {
    build_texture(dest, TextureType::Regular, TextureDimensions::Texture1d);
    build_texture(dest, TextureType::Compressed, TextureDimensions::Texture1d);
    build_texture(dest, TextureType::Srgb, TextureDimensions::Texture1d);
    build_texture(dest, TextureType::CompressedSrgb, TextureDimensions::Texture1d);
    build_texture(dest, TextureType::Integral, TextureDimensions::Texture1d);
    build_texture(dest, TextureType::Unsigned, TextureDimensions::Texture1d);
    build_texture(dest, TextureType::Depth, TextureDimensions::Texture1d);
    build_texture(dest, TextureType::Stencil, TextureDimensions::Texture1d);
    build_texture(dest, TextureType::DepthStencil, TextureDimensions::Texture1d);
    build_texture(dest, TextureType::Regular, TextureDimensions::Texture2d);
    build_texture(dest, TextureType::Compressed, TextureDimensions::Texture2d);
    build_texture(dest, TextureType::Srgb, TextureDimensions::Texture2d);
    build_texture(dest, TextureType::CompressedSrgb, TextureDimensions::Texture2d);
    build_texture(dest, TextureType::Integral, TextureDimensions::Texture2d);
    build_texture(dest, TextureType::Unsigned, TextureDimensions::Texture2d);
    build_texture(dest, TextureType::Depth, TextureDimensions::Texture2d);
    build_texture(dest, TextureType::Stencil, TextureDimensions::Texture2d);
    build_texture(dest, TextureType::DepthStencil, TextureDimensions::Texture2d);
    build_texture(dest, TextureType::Regular, TextureDimensions::Texture2dMultisample);
    build_texture(dest, TextureType::Integral, TextureDimensions::Texture2dMultisample);
    build_texture(dest, TextureType::Srgb, TextureDimensions::Texture2dMultisample);
    build_texture(dest, TextureType::Unsigned, TextureDimensions::Texture2dMultisample);
    build_texture(dest, TextureType::Depth, TextureDimensions::Texture2dMultisample);
    build_texture(dest, TextureType::Stencil, TextureDimensions::Texture2dMultisample);
    build_texture(dest, TextureType::DepthStencil, TextureDimensions::Texture2dMultisample);
    build_texture(dest, TextureType::Regular, TextureDimensions::Texture3d);
    build_texture(dest, TextureType::Compressed, TextureDimensions::Texture3d);
    build_texture(dest, TextureType::Srgb, TextureDimensions::Texture3d);
    build_texture(dest, TextureType::CompressedSrgb, TextureDimensions::Texture3d);
    build_texture(dest, TextureType::Integral, TextureDimensions::Texture3d);
    build_texture(dest, TextureType::Unsigned, TextureDimensions::Texture3d);
    build_texture(dest, TextureType::Depth, TextureDimensions::Texture3d);
    build_texture(dest, TextureType::Stencil, TextureDimensions::Texture3d);
    build_texture(dest, TextureType::DepthStencil, TextureDimensions::Texture3d);
    build_texture(dest, TextureType::Regular, TextureDimensions::Texture1dArray);
    build_texture(dest, TextureType::Compressed, TextureDimensions::Texture1dArray);
    build_texture(dest, TextureType::Srgb, TextureDimensions::Texture1dArray);
    build_texture(dest, TextureType::CompressedSrgb, TextureDimensions::Texture1dArray);
    build_texture(dest, TextureType::Integral, TextureDimensions::Texture1dArray);
    build_texture(dest, TextureType::Unsigned, TextureDimensions::Texture1dArray);
    build_texture(dest, TextureType::Depth, TextureDimensions::Texture1dArray);
    build_texture(dest, TextureType::Stencil, TextureDimensions::Texture1dArray);
    build_texture(dest, TextureType::DepthStencil, TextureDimensions::Texture1dArray);
    build_texture(dest, TextureType::Regular, TextureDimensions::Texture2dArray);
    build_texture(dest, TextureType::Compressed, TextureDimensions::Texture2dArray);
    build_texture(dest, TextureType::Srgb, TextureDimensions::Texture2dArray);
    build_texture(dest, TextureType::CompressedSrgb, TextureDimensions::Texture2dArray);
    build_texture(dest, TextureType::Integral, TextureDimensions::Texture2dArray);
    build_texture(dest, TextureType::Unsigned, TextureDimensions::Texture2dArray);
    build_texture(dest, TextureType::Depth, TextureDimensions::Texture2dArray);
    build_texture(dest, TextureType::Stencil, TextureDimensions::Texture2dArray);
    build_texture(dest, TextureType::DepthStencil, TextureDimensions::Texture2dArray);
    build_texture(dest, TextureType::Regular, TextureDimensions::Texture2dMultisampleArray);
    build_texture(dest, TextureType::Srgb, TextureDimensions::Texture2dMultisampleArray);
    build_texture(dest, TextureType::Integral, TextureDimensions::Texture2dMultisampleArray);
    build_texture(dest, TextureType::Unsigned, TextureDimensions::Texture2dMultisampleArray);
    build_texture(dest, TextureType::Depth, TextureDimensions::Texture2dMultisampleArray);
    build_texture(dest, TextureType::Stencil, TextureDimensions::Texture2dMultisampleArray);
    build_texture(dest, TextureType::DepthStencil, TextureDimensions::Texture2dMultisampleArray);
}

fn build_texture<W: Write>(mut dest: &mut W, ty: TextureType, dimensions: TextureDimensions) {
    // building the name of the texture type
    let name: String = {
        let prefix = match ty {
            TextureType::Regular => "",
            TextureType::Compressed => "Compressed",
            TextureType::Srgb => "Srgb",
            TextureType::CompressedSrgb => "CompressedSrgb",
            TextureType::Integral => "Integral",
            TextureType::Unsigned => "Unsigned",
            TextureType::Depth => "Depth",
            TextureType::Stencil => "Stencil",
            TextureType::DepthStencil => "DepthStencil",
        };

        let suffix = match dimensions {
            TextureDimensions::Texture1d => "Texture1d",
            TextureDimensions::Texture2d => "Texture2d",
            TextureDimensions::Texture2dMultisample => "Texture2dMultisample",
            TextureDimensions::Texture3d => "Texture3d",
            TextureDimensions::Texture1dArray => "Texture1dArray",
            TextureDimensions::Texture2dArray => "Texture2dArray",
            TextureDimensions::Texture2dMultisampleArray => "Texture2dMultisampleArray",
        };

        format!("{}{}", prefix, suffix)
    };

    // the trait corresponding to the data source
    let data_source_trait = match dimensions {
        TextureDimensions::Texture1d | TextureDimensions::Texture1dArray => "Texture1dDataSource",
        TextureDimensions::Texture2d | TextureDimensions::Texture2dArray => "Texture2dDataSource",
        TextureDimensions::Texture3d => "Texture3dDataSource",
        TextureDimensions::Texture2dMultisample | TextureDimensions::Texture2dMultisampleArray => {
            "unreachable"
        },
    };

    // the format enum corresponding to this texture
    let relevant_format = match ty {
        TextureType::Regular => "UncompressedFloatFormat",
        TextureType::Compressed => "CompressedFormat",
        TextureType::Srgb => "SrgbFormat",
        TextureType::CompressedSrgb => "CompressedSrgbFormat",
        TextureType::Integral => "UncompressedIntFormat",
        TextureType::Unsigned => "UncompressedUintFormat",
        TextureType::Depth => "DepthFormat",
        TextureType::Stencil => "StencilFormat",
        TextureType::DepthStencil => "DepthStencilFormat",
    };

    // the default format to use when none is specified
    let default_format = match ty {
        TextureType::Compressed => "TextureFormatRequest::AnyCompressed",
        TextureType::Regular => "TextureFormatRequest::AnyFloatingPoint",
        TextureType::CompressedSrgb => "TextureFormatRequest::AnyCompressedSrgb",
        TextureType::Srgb => "TextureFormatRequest::AnySrgb",
        TextureType::Integral => "TextureFormatRequest::AnyIntegral",
        TextureType::Unsigned => "TextureFormatRequest::AnyUnsigned",
        TextureType::Depth => "TextureFormatRequest::AnyDepth",
        TextureType::Stencil => "TextureFormatRequest::AnyStencil",
        TextureType::DepthStencil => "TextureFormatRequest::AnyDepthStencil",
    };

    // whether this is a internally compressed texture object
    let is_compressed = match ty {
        TextureType::Compressed |
        TextureType::CompressedSrgb => true,
        _ => false,
    };

    let client_format_any_ty = match ty {
        TextureType::Compressed => "ClientFormatAny::CompressedFormat",
        TextureType::CompressedSrgb => "ClientFormatAny::CompressedSrgbFormat",
        _ => "ClientFormatAny::ClientFormat",
    };

    let mipmaps_option_ty = match ty {
        TextureType::Compressed | TextureType::CompressedSrgb => "CompressedMipmapsOption",
        _ => "MipmapsOption",
    };

    let mipmap_default = match ty {
        TextureType::Compressed | TextureType::CompressedSrgb => "CompressedMipmapsOption::NoMipmap",
        _ => "MipmapsOption::AutoGeneratedMipmaps",
    };

    // the `#[cfg]` attribute for the related cargo feature
    let cfg_attribute = {
        let format = match ty {
            TextureType::Integral | TextureType::Unsigned => {
                "///
                /// # Features
                ///
                /// Only available if the 'gl_integral_textures' feature is enabled.
                #[cfg(feature = \"gl_integral_textures\")]"
            },
            TextureType::Depth | TextureType::DepthStencil => {
                "///
                /// # Features
                ///
                /// Only available if the 'gl_depth_textures' feature is enabled.
                #[cfg(feature = \"gl_depth_textures\")]"
            },
            TextureType::Stencil => "#[cfg(feature = \"gl_stencil_textures\")]",
            _ => ""
        };

        let dim = match dimensions {
            TextureDimensions::Texture1d => {
                "///
                /// # Features
                ///
                /// Only available if the 'gl_texture_1d' feature is enabled.
                #[cfg(feature = \"gl_texture_1d\")]"
            },
            TextureDimensions::Texture2dArray | TextureDimensions::Texture3d => {
                "///
                /// # Features
                /// 
                /// Only available if the 'gl_texture_3d' feature is enabled.
                #[cfg(feature = \"gl_texture_3d\")]"
            },
            TextureDimensions::Texture2dMultisample => {
                "///
                /// # Features
                /// 
                /// Only available if the 'gl_texture_multisample' feature is enabled.
                #[cfg(feature = \"gl_texture_multisample\")]"
            },
            TextureDimensions::Texture2dMultisampleArray => {
                "///
                /// # Features
                ///
                /// Only available if the 'gl_texture_multisample_array' feature is enabled.
                #[cfg(feature = \"gl_texture_multisample_array\")]"
            },
            _ => ""
        };

        format!("{}{}", format, dim)
    };

    // 
    let dimensions_parameters_input = match dimensions {
        TextureDimensions::Texture1d => "width: u32",
        TextureDimensions::Texture2d => "width: u32, height: u32",
        TextureDimensions::Texture2dMultisample => "width: u32, height: u32, samples: u32",
        TextureDimensions::Texture3d => "width: u32, height: u32, depth: u32",
        TextureDimensions::Texture1dArray => "width: u32, array_size: u32",
        TextureDimensions::Texture2dArray => "width: u32, height: u32, array_size: u32",
        TextureDimensions::Texture2dMultisampleArray => "width: u32, height: u32, array_size: u32, samples: u32",
    };

    let dimensions_parameters_passing = match dimensions {
        TextureDimensions::Texture1d => "width, None, None, None, None",
        TextureDimensions::Texture2d => "width, Some(height), None, None, None",
        TextureDimensions::Texture2dMultisample => "width, Some(height), None, None, Some(samples)",
        TextureDimensions::Texture3d => "width, Some(height), Some(depth), None, None",
        TextureDimensions::Texture1dArray => "width, None, None, Some(array_size), None",
        TextureDimensions::Texture2dArray => "width, Some(height), None, Some(array_size), None",
        TextureDimensions::Texture2dMultisampleArray => "width, Some(height), None, Some(array_size), Some(samples)",
    };

    let dimensions_parameters_passing_minimal = match dimensions {
        TextureDimensions::Texture1d => "width",
        TextureDimensions::Texture2d => "width, height",
        TextureDimensions::Texture2dMultisample => "width, height, samples",
        TextureDimensions::Texture3d => "width, height, depth",
        TextureDimensions::Texture1dArray => "width, array_size",
        TextureDimensions::Texture2dArray => "width, height, array_size",
        TextureDimensions::Texture2dMultisampleArray => "width, height, array_size, samples",
    };

    // writing the struct with doc-comment
    (write!(dest, "/// ")).unwrap();
    (write!(dest, "{}", match dimensions {
        TextureDimensions::Texture1d | TextureDimensions::Texture2d |
        TextureDimensions::Texture2dMultisample | TextureDimensions::Texture3d => "A ",
        TextureDimensions::Texture1dArray | TextureDimensions::Texture2dArray |
        TextureDimensions::Texture2dMultisampleArray => "An array of ",
    })).unwrap();
    if is_compressed {
        (write!(dest, "compressed ")).unwrap();
    }
    (write!(dest, "{}", match dimensions {
        TextureDimensions::Texture1d | TextureDimensions::Texture1dArray => "one-dimensional ",
        TextureDimensions::Texture2d | TextureDimensions::Texture2dArray |
        TextureDimensions::Texture2dMultisample | TextureDimensions::Texture2dMultisampleArray => {
            "two-dimensional "
        },
        TextureDimensions::Texture3d => "three-dimensional ",
    })).unwrap();
    (write!(dest, "{}", match dimensions {
        TextureDimensions::Texture1d | TextureDimensions::Texture2d |
        TextureDimensions::Texture2dMultisample | TextureDimensions::Texture3d => "texture ",
        TextureDimensions::Texture1dArray | TextureDimensions::Texture2dArray |
        TextureDimensions::Texture2dMultisampleArray => "textures ",
    })).unwrap();
    (write!(dest, "{}", match ty {
        TextureType::Regular | TextureType::Compressed => " containing floating-point data",
        TextureType::Srgb | TextureType::CompressedSrgb => " containing sRGB floating-point data",
        TextureType::Integral => " containing signed integral data",
        TextureType::Unsigned => " containing unsigned integral data",
        TextureType::Depth => " containing depth data",
        TextureType::Stencil => " containing stencil data",
        TextureType::DepthStencil => " containing both depth and stencil data",
    })).unwrap();
    (writeln!(dest, ".")).unwrap();
    (writeln!(dest, "pub struct {}(TextureAny);", name)).unwrap();

    // `Texture` trait impl
    (writeln!(dest, "
                impl Texture for {} {{
                    fn get_width(&self) -> u32 {{
                        self.0.get_width()
                    }}

                    fn get_height(&self) -> Option<u32> {{
                        self.0.get_height()
                    }}

                    fn get_depth(&self) -> Option<u32> {{
                        self.0.get_depth()
                    }}

                    fn get_array_size(&self) -> Option<u32> {{
                        self.0.get_array_size()
                    }}
                }}
            ", name)).unwrap();

    // `GlObject` trait impl
    (writeln!(dest, "
                impl GlObject for {} {{
                    type Id = gl::types::GLuint;
                    fn get_id(&self) -> gl::types::GLuint {{
                        self.0.get_id()
                    }}
                }}
            ", name)).unwrap();

    // `Debug` trait impl
    (writeln!(dest, "
                impl ::std::fmt::Debug for {} {{
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>
                    {{
                        self.0.fmt(f)
                    }}
                }}
            ", name)).unwrap();

    // 'Deref' impl to common type.
    (writeln!(dest, "
                impl ::std::ops::Deref for {} {{
                    type Target = TextureAny;
                    
                    fn deref<'a>(&'a self) -> &'a TextureAny {{
                        &self.0
                    }}
                }}
            ", name)).unwrap();

    // `UniformValue` trait impl
    {
        match ty {
            TextureType::Regular | TextureType::Compressed |
            TextureType::Srgb | TextureType::CompressedSrgb |
            TextureType::Integral | TextureType::Unsigned | TextureType::Depth => {
                (writeln!(dest, "
                            impl<'a> AsUniformValue for &'a {myname} {{
                                fn as_uniform_value(&self) -> UniformValue {{
                                    UniformValue::{myname}(*self, None)
                                }}

                                fn matches(_: &UniformType) -> bool {{
                                    false
                                }}
                            }}

                            impl<'a> AsUniformValue for Sampler<'a, {myname}> {{
                                fn as_uniform_value(&self) -> UniformValue {{
                                    UniformValue::{myname}(self.0, Some(self.1))
                                }}

                                fn matches(_: &UniformType) -> bool {{
                                    false
                                }}
                            }}

                            impl {myname} {{
                                /// Builds a `Sampler` marker object that allows you to indicate
                                /// how the texture should be sampled from inside a shader.
                                ///
                                /// # Example
                                ///
                                /// ```no_run
                                /// # #[macro_use] extern crate glium;
                                /// # fn main() {{
                                /// # let texture: glium::texture::Texture2d = unsafe {{
                                /// # ::std::mem::uninitialized() }};
                                /// let uniforms = uniform! {{
                                ///     color_texture: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                                /// }};
                                /// # }}
                                /// ```
                                pub fn sampled(&self) -> Sampler<{myname}> {{
                                    Sampler(self, Default::default())
                                }}
                            }}
                        ", myname = name)).unwrap();
            },
            _ => ()
        }
    }

    // `ToXXXAttachment` trait impl
    if dimensions == TextureDimensions::Texture2d || dimensions == TextureDimensions::Texture2dMultisample {
        match ty {
            TextureType::Regular => {
                (writeln!(dest, "
                        impl ::framebuffer::ToColorAttachment for {name} {{
                            fn to_color_attachment(&self) -> ::framebuffer::ColorAttachment {{
                                ::framebuffer::ColorAttachment::Texture(self.0.mipmap(0, 0).unwrap())
                            }}
                        }}
                    ", name = name)).unwrap();
            },
            TextureType::Srgb => {
                (writeln!(dest, "
                        impl ::framebuffer::ToColorAttachment for {name} {{
                            fn to_color_attachment(&self) -> ::framebuffer::ColorAttachment {{
                                ::framebuffer::ColorAttachment::Texture(self.0.mipmap(0, 0).unwrap())
                            }}
                        }}
                    ", name = name)).unwrap();
            },
            TextureType::Depth => {
                (writeln!(dest, "
                        impl ::framebuffer::ToDepthAttachment for {name} {{
                            fn to_depth_attachment(&self) -> ::framebuffer::DepthAttachment {{
                                ::framebuffer::DepthAttachment::Texture(self.0.mipmap(0, 0).unwrap())
                            }}
                        }}
                    ", name = name)).unwrap();
            },
            TextureType::Stencil => {
                (writeln!(dest, "
                        impl ::framebuffer::ToStencilAttachment for {name} {{
                            fn to_stencil_attachment(&self) -> ::framebuffer::StencilAttachment {{
                                ::framebuffer::StencilAttachment::Texture(self.0.mipmap(0, 0).unwrap())
                            }}
                        }}
                    ", name = name)).unwrap();
            },
            TextureType::DepthStencil => {
                (writeln!(dest, "
                        impl ::framebuffer::ToDepthStencilAttachment for {name} {{
                            fn to_depth_stencil_attachment(&self) -> ::framebuffer::DepthStencilAttachment {{
                                ::framebuffer::DepthStencilAttachment::Texture(self.0.mipmap(0, 0).unwrap())
                            }}
                        }}
                    ", name = name)).unwrap();
            },
            _ => ()
        }
    }

    // opening `impl Texture` block
    (writeln!(dest, "impl {} {{", name)).unwrap();

    // writing `get_internal_format_if_supported`
    writeln!(dest, "
            /// Determines the internal format of this texture.
            ///
            /// Returns `None` if the backend doesn't allow querying the actual format.
            pub fn get_internal_format_if_supported(&self) -> Option<InternalFormat> {{
                self.0.get_internal_format_if_supported()
            }}
        ").unwrap();

    // writing the `new` function
    if !dimensions.is_multisample() {
        let param = match dimensions {
            TextureDimensions::Texture1d | TextureDimensions::Texture2d |
            TextureDimensions::Texture3d => "T",

            TextureDimensions::Texture1dArray |
            TextureDimensions::Texture2dArray => "Vec<T>",

            _ => unreachable!()
        };

        let gen_doc = if is_compressed {
            "/// No mipmap level (except for the main level) will be allocator nor generated."
        } else {
            "/// This function will automatically generate all mipmaps of the texture."
        };

        (writeln!(dest, "
                /// Builds a new texture by uploading data.
                ///
                {gen_doc}
                {cfg_attr}
                pub fn new<'a, F, T>(facade: &F, data: {param})
                              -> {name} where T: {data_source_trait}<'a>, F: Facade
                {{
                    {name}::new_impl(facade, data, None, {mipmap_default}).unwrap()
                }}
            ", data_source_trait = data_source_trait, param = param, name = name,
               cfg_attr = cfg_attribute, mipmap_default = mipmap_default, gen_doc = gen_doc)).unwrap();
    }

    // writing the `new_if_supported` function
    if cfg_attribute.len() >= 1 && !dimensions.is_multisample() {
        let param = match dimensions {
            TextureDimensions::Texture1d | TextureDimensions::Texture2d |
            TextureDimensions::Texture3d => "T",

            TextureDimensions::Texture1dArray |
            TextureDimensions::Texture2dArray => "Vec<T>",

            _ => unreachable!()
        };

        let gen_doc = if is_compressed {
            "/// No mipmap level (except for the main level) will be allocator nor generated."
        } else {
            "/// This function will automatically generate all mipmaps of the texture."
        };

        (writeln!(dest, "
                /// Builds a new texture by uploading data.
                ///
                {gen_doc}
                pub fn new_if_supported<'a, F, T>(facade: &F, data: {param})
                                               -> Option<{name}> where T: {data_source_trait}<'a>, F: Facade
                {{
                    match {name}::new_impl(facade, data, None, {mipmap_default}) {{
                        Ok(t) => Some(t),
                        Err(TextureMaybeSupportedCreationError::NotSupported) => None,
                        Err(TextureMaybeSupportedCreationError::CreationError(_)) => unreachable!()
                    }}
                }}
            ", data_source_trait = data_source_trait, param = param, name = name,
               mipmap_default = mipmap_default, gen_doc = gen_doc)).unwrap();
    }

    // writing the `with_mipmaps` function
    if !dimensions.is_multisample() {
        let param = match dimensions {
            TextureDimensions::Texture1d | TextureDimensions::Texture2d |
            TextureDimensions::Texture3d => "T",

            TextureDimensions::Texture1dArray |
            TextureDimensions::Texture2dArray => "Vec<T>",

            _ => unreachable!()
        };

        (writeln!(dest, "
                /// Builds a new texture by uploading data.
                {cfg_attr}
                pub fn with_mipmaps<'a, F, T>(facade: &F, data: {param}, mipmaps: {mipmaps})
                                           -> {name} where T: {data_source_trait}<'a>, F: Facade
                {{
                    {name}::new_impl(facade, data, None, mipmaps).unwrap()
                }}
            ", data_source_trait = data_source_trait, param = param, name = name,
                cfg_attr = cfg_attribute, mipmaps = mipmaps_option_ty)).unwrap();
    }

    // writing the `with_mipmaps_if_supported` function
    if cfg_attribute.len() >= 1 && !dimensions.is_multisample() {
        let param = match dimensions {
            TextureDimensions::Texture1d | TextureDimensions::Texture2d |
            TextureDimensions::Texture3d => "T",

            TextureDimensions::Texture1dArray |
            TextureDimensions::Texture2dArray => "Vec<T>",

            _ => unreachable!()
        };

        (writeln!(dest, "
                /// Builds a new texture by uploading data.
                pub fn with_mipmaps_if_supported<'a, F, T>(facade: &F, data: {param},
                                                        mipmaps: {mipmaps}) -> Option<{name}>
                                                        where T: {data_source_trait}<'a>, F: Facade
                {{
                    match {name}::new_impl(facade, data, None, mipmaps) {{
                        Ok(t) => Some(t),
                        Err(TextureMaybeSupportedCreationError::NotSupported) => None,
                        Err(TextureMaybeSupportedCreationError::CreationError(_)) => unreachable!()
                    }}
                }}
            ", data_source_trait = data_source_trait, param = param, name = name,
               mipmaps = mipmaps_option_ty)).unwrap();
    }

    // writing the `with_compressed_data` / `with_compressed_data_if_supported` functions
    if is_compressed && !dimensions.is_multisample() {
        let param = match dimensions {
            TextureDimensions::Texture1d | TextureDimensions::Texture2d |
            TextureDimensions::Texture3d => "&[u8]",

            TextureDimensions::Texture1dArray |
            TextureDimensions::Texture2dArray => "Vec<&[u8]>",

            _ => unreachable!()
        };

        (writeln!(dest, "
                /// Builds a new texture with a specific format. The input data must also be of the
                /// specified compressed format.
                {cfg_attr}
                pub fn with_compressed_data<F>(facade: &F, data: {param}, {dim_params},
                                                      format: {format}, mipmaps: {mipmaps})
                                                      -> {name}
                                                       where F: Facade
                {{
                    {name}::with_compressed_data_if_supported(facade, data, {dim_params_passing}, format, mipmaps).unwrap()
                }}
            ", dim_params = dimensions_parameters_input, dim_params_passing = dimensions_parameters_passing_minimal,
               param = param, cfg_attr = cfg_attribute, name = name, format = relevant_format,
               mipmaps = mipmaps_option_ty).unwrap());

        (writeln!(dest, "
                /// Builds a new texture with a specific format. The input data must also be of the
                /// specified compressed format.
                {cfg_attr}
                pub fn with_compressed_data_if_supported<F>(facade: &F, data: {param}, {dim_params},
                                                      format: {format}, mipmaps: {mipmaps})
                                                      -> Result<{name}, TextureMaybeSupportedCreationError>
                                                       where F: Facade
                {{
                    let data = Cow::Borrowed(data.as_ref());
                    let client_format = {client_format_any}(format);
                    Ok({name}(try!(any::new_texture(facade, {default_format}, Some((client_format, data)),
                                                    mipmaps.to_regular(), {dim_params_passing}))))
                }}
            ", dim_params = dimensions_parameters_input, dim_params_passing = dimensions_parameters_passing,
               param = param, client_format_any = client_format_any_ty, cfg_attr = cfg_attribute, 
               name = name, format = relevant_format, default_format = default_format,
               mipmaps = mipmaps_option_ty).unwrap());
    }

    // writing the `with_format` function
    if !dimensions.is_multisample() {
        let param = match dimensions {
            TextureDimensions::Texture1d | TextureDimensions::Texture2d |
            TextureDimensions::Texture3d => "T",

            TextureDimensions::Texture1dArray |
            TextureDimensions::Texture2dArray => "Vec<T>",

            _ => unreachable!()
        };

        (writeln!(dest, "
                /// Builds a new texture with a specific format.
                {cfg_attr}
                pub fn with_format<'a, F, T>(facade: &F, data: {param},
                                          format: {format}, mipmaps: {mipmaps})
                                          -> Result<{name}, TextureCreationError>
                                          where T: {data_source_trait}<'a>, F: Facade
                {{
                    match {name}::new_impl(facade, data, Some(format), mipmaps) {{
                        Ok(t) => Ok(t),
                        Err(TextureMaybeSupportedCreationError::CreationError(e)) => Err(e),
                        Err(TextureMaybeSupportedCreationError::NotSupported) => unreachable!()
                    }}
                }}
            ", data_source_trait = data_source_trait, param = param, cfg_attr = cfg_attribute,
               format = relevant_format, name = name,
               mipmaps = mipmaps_option_ty)).unwrap();
    }

    // writing the `with_format_if_supported` function
    if cfg_attribute.len() >= 1 && !dimensions.is_multisample() {
        let param = match dimensions {
            TextureDimensions::Texture1d | TextureDimensions::Texture2d |
            TextureDimensions::Texture3d => "T",

            TextureDimensions::Texture1dArray |
            TextureDimensions::Texture2dArray => "Vec<T>",

            _ => unreachable!()
        };

        (writeln!(dest, "
                /// Builds a new texture with a specific format.
                pub fn with_format_if_supported<'a, F, T>(facade: &F, data: {param},
                                                       format: {format}, mipmaps: {mipmaps})
                                                       -> Result<{name}, TextureMaybeSupportedCreationError>
                                                       where T: {data_source_trait}<'a>, F: Facade
                {{
                    {name}::new_impl(facade, data, Some(format), mipmaps)
                }}
            ", data_source_trait = data_source_trait, param = param,
               format = relevant_format, name = name,
               mipmaps = mipmaps_option_ty)).unwrap();
    }

    // writing the `new_impl` function
    if !dimensions.is_multisample() {
        let param = match dimensions {
            TextureDimensions::Texture1d | TextureDimensions::Texture2d |
            TextureDimensions::Texture3d => "T",

            TextureDimensions::Texture1dArray |
            TextureDimensions::Texture2dArray => "Vec<T>",

            _ => unreachable!()
        };

        (writeln!(dest, "
                fn new_impl<'a, F, T>(facade: &F, data: {param},
                                   format: Option<{relevant_format}>, mipmaps: {mipmaps})
                                   -> Result<{name}, TextureMaybeSupportedCreationError>
                                   where T: {data_source_trait}<'a>, F: Facade
                {{
            ", data_source_trait = data_source_trait,
               param = param, name = name,
               relevant_format = relevant_format,
               mipmaps = mipmaps_option_ty)).unwrap();

        // writing the `let format = ...` line
        (write!(dest, "let format = format.map(|f| {{
                           TextureFormatRequest::Specific(f.to_texture_format())
                       }}).unwrap_or({});", default_format)).unwrap();

        match dimensions {
            TextureDimensions::Texture1d => (write!(dest, "
                    let RawImage1d {{ data, width, format: client_format }} = data.into_raw();
                ")).unwrap(),

            TextureDimensions::Texture2d => (write!(dest, "
                    let RawImage2d {{ data, width, height, format: client_format }} =
                                            data.into_raw();
                ")).unwrap(),

            TextureDimensions::Texture3d => (write!(dest, "
                    let RawImage3d {{ data, width, height, depth, format: client_format }} =
                                            data.into_raw();
                ")).unwrap(),

            TextureDimensions::Texture1dArray => (write!(dest, "
                    let vec_raw = data.into_iter().map(|e| e.into_raw()).collect();
                    let RawImage2d {{data, width, height: array_size, format: client_format }} = RawImage2d::from_vec_raw1d(&vec_raw);
                ")).unwrap(),   // TODO: panic if dimensions are inconsistent

            TextureDimensions::Texture2dArray => (write!(dest, "
                    let vec_raw = data.into_iter().map(|e| e.into_raw()).collect();
                    let RawImage3d {{data, width, height, depth: array_size, format: client_format }} = RawImage3d::from_vec_raw2d(&vec_raw);
                ")).unwrap(),   // TODO: panic if dimensions are inconsistent

            _ => unreachable!()
        }

        (write!(dest, "let client_format = ClientFormatAny::ClientFormat(client_format);")).unwrap();

        // writing the constructor
        (write!(dest, "Ok({}(try!(any::new_texture(facade, format, \
                       Some((client_format, data)), mipmaps.to_regular(), {}", name, dimensions_parameters_passing)).unwrap();
        (writeln!(dest, "))))")).unwrap();

        // end of "new" function block
        (writeln!(dest, "}}")).unwrap();
    }

    // writing the `new_empty` function
    if !is_compressed {
        // opening function
        (writeln!(dest, "
                /// DEPRECATED. Use `empty` instead.
                pub fn new_empty<F>(facade: &F, format: {format}, {dim_params}) -> {name} where F: Facade {{
                    let format = format.to_texture_format();
                    let format = TextureFormatRequest::Specific(format);
            ", format = relevant_format, dim_params = dimensions_parameters_input, name = name)).unwrap();

        // writing the constructor
        (write!(dest, "{}(any::new_texture::<_, u8>(facade, format, None, {mipmap_default}.to_regular(), {}).unwrap())",
                name, dimensions_parameters_passing, mipmap_default = mipmap_default)).unwrap();

        // closing function
        (writeln!(dest, "}}")).unwrap();
    }

    // writing the `empty` function
    if !is_compressed {
        // opening function
        (writeln!(dest, "
                /// Creates an empty texture.
                ///
                /// No mipmap level (except for the main level) will be allocated or generated.
                ///
                /// The texture will contain undefined data.
                {cfg_attr}
                pub fn empty<F>(facade: &F, {dim_params}) -> {name} where F: Facade {{
                    let format = {format};
            ", format = default_format, dim_params = dimensions_parameters_input, name = name,
                cfg_attr = cfg_attribute)).unwrap();

        // writing the constructor
        (write!(dest, "{}(any::new_texture::<_, u8>(facade, format, None, {mipmap}::NoMipmap.to_regular(), {}).unwrap())",
                name, dimensions_parameters_passing, mipmap = mipmaps_option_ty)).unwrap();

        // closing function
        (writeln!(dest, "}}")).unwrap();
    }

    // writing the `empty_if_supported` function
    if !is_compressed {
        // opening function
        (writeln!(dest, "
                /// Creates an empty texture.
                ///
                /// No mipmap level (except for the main level) will be allocated or generated.
                ///
                /// The texture will contain undefined data.
                pub fn empty_if_supported<F>(facade: &F, {dim_params}) -> Option<{name}> where F: Facade {{
                    let format = {format};
            ", format = default_format, dim_params = dimensions_parameters_input, name = name)).unwrap();

        // writing the constructor
        (write!(dest, "match any::new_texture::<_, u8>(facade, format, None, {mipmap}::NoMipmap.to_regular(), {})",
                dimensions_parameters_passing, mipmap = mipmaps_option_ty)).unwrap();
        (writeln!(dest, "
            {{
                Ok(t) => Some({}(t)),
                Err(TextureMaybeSupportedCreationError::NotSupported) => None,
                Err(TextureMaybeSupportedCreationError::CreationError(_)) => unreachable!()
            }}", name)).unwrap();

        // closing function
        (writeln!(dest, "}}")).unwrap();
    }

    // writing the `empty_with_format` function
    if true {
        // opening function
        (writeln!(dest, "
                /// Creates an empty texture with a specific format.
                ///
                /// The texture (and its mipmaps) will contain undefined data.
                {cfg_attr}
                pub fn empty_with_format<F>(facade: &F, format: {format}, mipmaps: {mipmaps}, {dim_params}) -> Result<{name}, TextureCreationError> where F: Facade {{
                    let format = format.to_texture_format();
                    let format = TextureFormatRequest::Specific(format);
            ", format = relevant_format, dim_params = dimensions_parameters_input, name = name,
                cfg_attr = cfg_attribute, mipmaps = mipmaps_option_ty)).unwrap();

        // writing the constructor
        (write!(dest, "let t = any::new_texture::<_, u8>(facade, format, None, mipmaps.to_regular(), {});", dimensions_parameters_passing)).unwrap();
        (writeln!(dest, "
            match t {{
                Ok(t) => Ok({}(t)),
                Err(TextureMaybeSupportedCreationError::CreationError(e)) => Err(e),
                Err(TextureMaybeSupportedCreationError::NotSupported) => unreachable!()
            }}", name)).unwrap();

        // closing function
        (writeln!(dest, "}}")).unwrap();
    }

    // writing the `empty_with_format_if_supported` function
    if true {
        // opening function
        (writeln!(dest, "
                /// Creates an empty texture with a specific format.
                ///
                /// Note that passing `true` for `mipmaps` does not mean that you will get mipmaps.
                /// Instead it indicates that mipmaps are *allowed* to be created if possible.
                ///
                /// The texture (and its mipmaps) will contain undefined data.
                pub fn empty_with_format_if_supported<F>(facade: &F, format: {format},
                                                      mipmaps: {mipmaps}, {dim_params})
                                                      -> Result<{name},
                                                                TextureMaybeSupportedCreationError> where F: Facade
                {{
                    let format = format.to_texture_format();
                    let format = TextureFormatRequest::Specific(format);
            ", format = relevant_format, dim_params = dimensions_parameters_input, name = name,
               mipmaps = mipmaps_option_ty)).unwrap();

        // writing the constructor
        (write!(dest, "any::new_texture::<_, u8>(facade, format, None, mipmaps.to_regular(), {})", dimensions_parameters_passing)).unwrap();
        (writeln!(dest, ".map(|t| {}(t))", name)).unwrap();

        // closing function
        (writeln!(dest, "}}")).unwrap();
    }

    // writing the `empty_with_mipmaps` function
    if !is_compressed {
        // opening function
        (writeln!(dest, "
                /// Creates an empty texture. Specifies whether is has mipmaps.
                ///
                /// The texture (and its mipmaps) will contain undefined data.
                {cfg_attr}
                pub fn empty_with_mipmaps<F>(facade: &F, mipmaps: {mipmaps}, {dim_params}) -> {name} where F: Facade {{
                    let format = {format};
            ", format = default_format, dim_params = dimensions_parameters_input, name = name,
                cfg_attr = cfg_attribute, mipmaps = mipmaps_option_ty)).unwrap();

        // writing the constructor
        (write!(dest, "{}(any::new_texture::<_, u8>(facade, format, None, mipmaps.to_regular(), {})", name, dimensions_parameters_passing)).unwrap();
        (writeln!(dest, ".unwrap())")).unwrap();

        // closing function
        (writeln!(dest, "}}")).unwrap();
    }

    // writing the `empty_with_mipmaps_if_supported` function
    if !is_compressed {
        // opening function
        (writeln!(dest, "
                /// Creates an empty texture. Specifies whether is has mipmaps.
                ///
                /// The texture (and its mipmaps) will contain undefined data.
                pub fn empty_with_mipmaps_if_supported<F>(facade: &F, mipmaps: {mipmaps},
                                                       {dim_params}) -> Option<{name}> where F: Facade {{
                    let format = {format};
            ", format = default_format, dim_params = dimensions_parameters_input, name = name,
               mipmaps = mipmaps_option_ty)).unwrap();

        // writing the constructor
        (write!(dest, "match any::new_texture::<_, u8>(facade, format, None, mipmaps.to_regular(), {})", dimensions_parameters_passing)).unwrap();
        (writeln!(dest, "
            {{
                Ok(t) => Some({}(t)),
                Err(TextureMaybeSupportedCreationError::NotSupported) => None,
                Err(TextureMaybeSupportedCreationError::CreationError(_)) => unreachable!()
            }}", name)).unwrap();

        // closing function
        (writeln!(dest, "}}")).unwrap();
    }


    // writing the `as_surface` function
    if (dimensions == TextureDimensions::Texture2d ||
        dimensions == TextureDimensions::Texture2dMultisample) && ty == TextureType::Regular
    {
        (write!(dest, "
                /// Starts drawing on the texture.
                ///
                /// All the function calls to the framebuffer will draw on the texture instead
                /// of the screen.
                ///
                /// ## Low-level information
                ///
                /// The first time that this function is called, a FrameBuffer Object will be
                /// created and cached. The following calls to `as_surface` will load the existing
                /// FBO and re-use it. When the texture is destroyed, the FBO is destroyed too.
                ///
                pub fn as_surface<'a>(&'a self) -> framebuffer::SimpleFrameBuffer<'a> {{
                    framebuffer::SimpleFrameBuffer::new(self.0.get_context(), self)
                }}
            ")).unwrap();
    }

    // writing the `get_mipmap_levels` function
    (write!(dest, "
            /// Returns the number of mipmap levels of the texture.
            ///
            /// The minimum value is 1, since there is always a main texture.
            pub fn get_mipmap_levels(&self) -> u32 {{
                self.0.get_mipmap_levels()
            }}
        ")).unwrap();

    // writing the `read` functions
    // TODO: implement for other types too
    if dimensions == TextureDimensions::Texture2d &&
       (ty == TextureType::Regular || is_compressed)
    {
        (write!(dest, r#"
                /// Reads the content of the texture to RAM.
                ///
                /// You should avoid doing this at all cost during performance-critical
                /// operations (for example, while you're drawing).
                /// Use `read_to_pixel_buffer` instead.
                pub fn read<T>(&self) -> T where T: Texture2dDataSink<(u8, u8, u8, u8)> {{
                    self.0.read(0)
                }}
            "#)).unwrap();

        (write!(dest, r#"
                /// Reads the content of the texture into a buffer in video memory.
                ///
                /// This operation copies the texture's data into a buffer in video memory
                /// (a pixel buffer). Contrary to the `read` function, this operation is
                /// done asynchronously and doesn't need a synchronization.
                pub fn read_to_pixel_buffer(&self) -> PixelBuffer<(u8, u8, u8, u8)> {{
                    self.0.read_to_pixel_buffer(0)
                }}
            "#)).unwrap();
    }

    // writing the `read_compressed_data` function
    if is_compressed && !dimensions.is_array() {
        (write!(dest, r#"
                /// Reads the content of the texture to RAM without decompressing it before.
                ///
                /// You should avoid doing this at all cost during performance-critical
                /// operations (for example, while you're drawing).
                ///
                /// Returns the compressed format of the texture and the compressed data, gives
                /// `None` when the internal compression format is generic or unknown.
                pub fn read_compressed_data(&self) -> Option<({format}, Vec<u8>)> {{
                    self.main_level().read_compressed_data()
                }}
            "#, format = relevant_format)).unwrap();
    }


    // writing the `write` function
    // TODO: implement for other types too
    if dimensions == TextureDimensions::Texture2d &&
            (ty == TextureType::Regular || is_compressed)
    {
        let compressed_restrictions = if is_compressed {
            r#" ///
                /// Calling this for compressed textures will result in a panic of type INVALID_OPERATION
                /// if `Rect::bottom` or `Rect::width` is not equal to 0 (border). In addition, the contents
                /// of any texel outside the region modified by such a call are undefined. These
                /// restrictions may be relaxed for specific compressed internal formats whose images
                /// are easily edited.
            "#
        } else {
            ""
        };

        (write!(dest, r#"
                /// Uploads some data in the texture.
                ///
                /// Note that this may cause a synchronization if you use the texture right before
                /// or right after this call. Prefer creating a whole new texture if you change a
                /// huge part of it.
                ///
                /// ## Panic
                ///
                /// Panics if the the dimensions of `data` don't match the `Rect`.
                {compressed_restrictions}
                pub fn write<'a, T>(&self, rect: Rect, data: T) where T: {data_source_trait}<'a> {{
                    self.main_level().write(rect, data)
                }}
            "#, data_source_trait = data_source_trait,
                compressed_restrictions = compressed_restrictions)).unwrap();
    }

    // writing the `write_compressed_data` function
    // TODO: implement for other types too
    if dimensions == TextureDimensions::Texture2d && is_compressed
    {
        (write!(dest, r#"
                /// Uploads some data in the texture by using a compressed format as input.
                ///
                /// Please see `write_compressed_data_if_supported` for details.
                pub fn write_compressed_data(&self, rect: Rect, data: &[u8],
                                             width: u32, height: u32, format: {format})
                {{

                    self.write_compressed_data_if_supported(rect, data, width, height, format).unwrap()
                }}

                /// Uploads some data in the texture by using a compressed format as input.
                ///
                /// Note that this may cause a synchronization if you use the texture right before
                /// or right after this call. Prefer creating a whole new texture if you change a
                /// huge part of it.
                ///
                /// ## Panic
                ///
                /// Panics if the the dimensions of `data` don't match the `Rect`.
                ///
                /// Calling this will result in a panic of type INVALID_OPERATION error if `Rect::width`
                /// or `Rect::height` is not equal to 0 (border), or if the written dimensions do not match
                /// the original texture dimensions. The contents of any texel outside the region modified
                /// by the call are undefined. These restrictions may be relaxed for specific compressed
                /// internal formats whose images are easily edited.
                pub fn write_compressed_data_if_supported(&self, rect: Rect, data: &[u8],
                                                          width: u32, height: u32, format: {format})
                                                          -> Result<(), ()>
                {{
                    // FIXME is having width and height as parameter redundant as rect kinda of
                    // already provides them? 
                    self.main_level().write_compressed_data_if_supported(rect, data, width, height, format)
                }}
            "#, format = relevant_format)).unwrap();
    }

    // writing the `layer()` function
    if dimensions.is_array() {
        (write!(dest, r#"
                /// Access a single layer of this texture.
                pub fn layer(&self, layer: u32) -> Option<{name}Layer> {{
                    if layer < self.0.get_array_size().unwrap_or(1) {{
                        Some({name}Layer {{
                            texture: self,
                            layer: layer,
                        }})
                    }} else {{
                        None
                    }}
                }}
            "#, name = name)).unwrap();
    }

    // writing the `mipmap()` and `main_level()` functions
    if !dimensions.is_array() {
        (write!(dest, r#"
                /// Access a single mipmap level of this texture.
                pub fn mipmap(&self, level: u32) -> Option<{name}Mipmap> {{
                    if level < self.0.get_mipmap_levels() {{
                        Some({name}Mipmap(any::new_mipmap_view(&self.0, level, 0), self))
                    }} else {{
                        None
                    }}
                }}
            "#, name = name)).unwrap();

        (write!(dest, r#"
                /// Access the main mipmap level of this texture.
                pub fn main_level(&self) -> {name}Mipmap {{
                    self.mipmap(0).unwrap()
                }}
            "#, name = name)).unwrap();
    }

    // closing `impl Texture` block
    (writeln!(dest, "}}")).unwrap();

    // the `Layer` struct
    if dimensions.is_array() {
        // writing the struct
        (write!(dest, r#"
                /// Represents a single layer of a `{name}`.
                ///
                /// Can be obtained by calling `{name}::layer()`.
                #[derive(Copy, Clone)]
                pub struct {name}Layer<'t> {{
                    texture: &'t {name},
                    layer: u32,
                }}
            "#, name = name)).unwrap();

        // opening `impl Layer` block
        (writeln!(dest, "impl<'t> {}Layer<'t> {{", name)).unwrap();

        // writing the `get_layer` and `get_texture` functions
        (write!(dest, "
                /// Returns the corresponding texture.
                pub fn get_texture(&self) -> &'t {name} {{
                    self.texture
                }}

                /// Returns the layer index.
                pub fn get_layer(&self) -> u32 {{
                    self.layer
                }}
            ", name = name)).unwrap();

        // writing the `get_mipmap_levels` function
        (write!(dest, "
                /// Returns the number of mipmap levels of the texture.
                ///
                /// The minimum value is 1, since there is always a main texture.
                pub fn get_mipmap_levels(&self) -> u32 {{
                    self.texture.get_mipmap_levels()
                }}
            ")).unwrap();

        // writing the `mipmap()` function
        (write!(dest, r#"
                /// Access a single mipmap level of this layer.
                pub fn mipmap(&self, level: u32) -> Option<{name}Mipmap> {{
                    if level < self.texture.get_mipmap_levels() {{
                        Some({name}Mipmap(any::new_mipmap_view(&self.texture.0, level, self.layer), &self.texture))
                    }} else {{
                        None
                    }}
                }}
            "#, name = name)).unwrap();

        // writing the `main_level()` function
        (write!(dest, r#"
                /// Access the main mipmap level of this layer.
                pub fn main_level(&self) -> {name}Mipmap {{
                    self.mipmap(0).unwrap()
                }}
            "#, name = name)).unwrap();

        // closing `impl Layer` block
        (writeln!(dest, "}}")).unwrap();
    }

    // the `Mipmap` struct
    {
        // writing the struct
        if dimensions.is_array() {
            (write!(dest, r#"
                    /// Represents a single mipmap level of a `{name}`.
                    ///
                    /// Can be obtained by calling `{name}Layer::mipmap()` or
                    /// `{name}Layer::main_level()`.
                    #[derive(Copy, Clone)]
                    pub struct {name}Mipmap<'t>(TextureAnyMipmap<'t>, &'t {name});
                "#, name = name)).unwrap();

        } else {
            (write!(dest, r#"
                    /// Represents a single mipmap level of a `{name}`.
                    ///
                    /// Can be obtained by calling `{name}::mipmap()` or `{name}::main_level()`.
                    #[derive(Copy, Clone)]
                    pub struct {name}Mipmap<'t>(TextureAnyMipmap<'t>, &'t {name});
                "#, name = name)).unwrap();
        }

        // opening `impl Mipmap` block
        (writeln!(dest, "impl<'t> {}Mipmap<'t> {{", name)).unwrap();

        // writing the `write` function for mipmaps.
        // TODO: implement for other types too
        if dimensions == TextureDimensions::Texture2d &&
                (ty == TextureType::Regular || is_compressed)
        {
            let compressed_restrictions = if is_compressed {
                r#" ///
                    /// Calling this for compressed textures will result in a panic of type INVALID_OPERATION
                    /// if `Rect::bottom` or `Rect::width` is not equal to 0 (border). In addition, the contents
                    /// of any texel outside the region modified by such a call are undefined. These
                    /// restrictions may be relaxed for specific compressed internal formats whose images
                    /// are easily edited.
                "#
            } else {
                ""
            };

            (write!(dest, r#"
                    /// Uploads some data in the texture level.
                    ///
                    /// Note that this may cause a synchronization if you use the texture right before
                    /// or right after this call.
                    ///
                    /// ## Panic
                    ///
                    /// Panics if the the dimensions of `data` don't match the `Rect`.
                    {compressed_restrictions}
                    pub fn write<'a, T>(&self, rect: Rect, data: T) where T: {data_source_trait}<'a> {{
                        let RawImage2d {{ data, width, height, format: client_format }} =
                                                data.into_raw();

                        assert_eq!(width, rect.width);
                        assert_eq!(height, rect.height);

                        let client_format = ClientFormatAny::ClientFormat(client_format);

                        any::upload_texture(&self.0, rect.left, rect.bottom, 0, (client_format, data), width,
                                            Some(height), None, true).unwrap()
                    }}
                "#, data_source_trait = data_source_trait,
                    compressed_restrictions = compressed_restrictions)).unwrap();
        }

        // writing the `write_compressed_data` function for mipmaps.
        // TODO: implement for other types too
        if dimensions == TextureDimensions::Texture2d && is_compressed
        {
            (write!(dest, r#"
                    /// Uploads some data in the texture level by using a compressed format as input.
                    ///
                    /// Please see `write_compressed_data_if_supported` for details.
                    pub fn write_compressed_data(&self, rect: Rect, data: &[u8],
                                                 width: u32, height: u32, format: {format})
                    {{

                        self.write_compressed_data_if_supported(rect, data, width, height, format).unwrap()
                    }}

                    /// Uploads some data in the texture level by using a compressed format as input.
                    ///
                    /// Note that this may cause a synchronization if you use the texture right before
                    /// or right after this call.
                    ///
                    /// ## Panic
                    ///
                    /// Panics if the the dimensions of `data` don't match the `Rect`.
                    ///
                    /// Calling this will result in a panic of type INVALID_OPERATION error if `Rect::width`
                    /// or `Rect::height` is not equal to 0 (border), or if the written dimensions do not match
                    /// the original texture dimensions. The contents of any texel outside the region modified
                    /// by the call are undefined. These restrictions may be relaxed for specific compressed
                    /// internal formats whose images are easily edited.
                    pub fn write_compressed_data_if_supported(&self, rect: Rect, data: &[u8],
                                                              width: u32, height: u32, format: {format})
                                                              -> Result<(), ()>
                    {{
                        // FIXME is having width and height as parameter redundant as rect kinda of
                        // already provides them? 

                        assert_eq!(width, rect.width);
                        assert_eq!(height, rect.height);

                        let data = Cow::Borrowed(data.as_ref());
                        let client_format = {client_format_any}(format);

                        any::upload_texture(&self.0, rect.left, rect.bottom, 0, (client_format, data),
                                            width, Some(height), None, false)
                    }}
                "#, format = relevant_format, client_format_any = client_format_any_ty)).unwrap();
        }


        // writing the `read_compressed_data` function for mipmaps
        if is_compressed && !dimensions.is_array() {
            (write!(dest, r#"
                    /// Reads the content of the texture level to RAM without decompressing it before.
                    ///
                    /// You should avoid doing this at all cost during performance-critical
                    /// operations (for example, while you're drawing).
                    ///
                    /// Returns the compressed format of the texture and the compressed data, gives
                    /// `None` when the internal compression format is generic or unknown.
                    pub fn read_compressed_data(&self) -> Option<({format}, Vec<u8>)> {{
                        match any::download_compressed_data(&self.0) {{
                            Some(({client_format_any}(format), buf)) => Some((format, buf)),
                            None => None,
                            _ => unreachable!(),
                        }}
                    }}
                "#, format = relevant_format, client_format_any = client_format_any_ty)).unwrap();
        }

        // writing the `get_level` and `get_texture` functions
        (write!(dest, "
                /// Returns the corresponding texture.
                pub fn get_texture(&self) -> &'t {name} {{
                    self.1
                }}

                /// Returns the texture level.
                pub fn get_level(&self) -> u32 {{
                    self.0.get_level()
                }}
            ", name = name)).unwrap();

        // writing the `get_layer` function
        if dimensions.is_array() {
            (write!(dest, "
                    /// Returns the layer index.
                    pub fn get_layer(&self) -> u32 {{
                        self.0.get_layer()
                    }}
                ")).unwrap();
        }

        // closing `impl Mipmap` block
        (writeln!(dest, "}}")).unwrap();
    }
}
