use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint};
use std::{mem, ptr};
use std::ffi::c_void;

pub unsafe fn triangle_with_color_attributes() -> GLuint {
    let vertices: [f32; 18] = [
        //positions          //colors
        0.5, -0.5, 0.0,   1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 1.0, 0.0, // bottom left
        0.0,  0.5, 0.0,   0.0, 0.0, 1.0  // top
    ];

    let (mut vbo, mut vao) = (0, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::BindVertexArray(vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);
    gl::VertexAttribPointer(0,
                            3,
                            gl::FLOAT,
                            gl::FALSE,
                            6 * mem::size_of::<GLfloat>() as GLsizei,
                            ptr::null());
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(1,
                            3,
                            gl::FLOAT,
                            gl::FALSE,
                            6 * mem::size_of::<GLfloat>() as GLsizei,
                            (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);

    vao as GLuint
}