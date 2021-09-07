use gl::types::GLuint;
use crate::utils::gl_check_error;
use std::ffi::c_void;
use image::GenericImageView;
use image::io::Reader as ImageReader;

pub struct BasicTexture {
    texture_handle: GLuint
}

impl BasicTexture {
    pub unsafe fn new(path_to_texture_resource: &str) -> BasicTexture {
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        let img = ImageReader::open(path_to_texture_resource).unwrap().decode().unwrap();
        let image_data = img.to_bytes();
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGB as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0,
                       gl::RGB,
                       gl::UNSIGNED_BYTE,
                       &image_data[0] as *const u8 as *const c_void);

        gl_check_error(file!(), line!());

        BasicTexture {
            texture_handle: texture
        }
    }

    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
    }
}