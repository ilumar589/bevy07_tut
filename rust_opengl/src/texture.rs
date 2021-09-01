use gl::types::GLuint;

struct Texture {
    texture_handle: GLuint
}

impl Texture {
    fn new(&self, path_to_texture_resource: &str) -> Texture {
        unimplemented!() // I still have to see how these textures are used in a real model so I can determine if the abstraction is worth it
    }
}