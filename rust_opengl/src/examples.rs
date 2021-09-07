use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint};
use std::{mem, ptr};
use std::ffi::c_void;
use std::path::Path;
use image::GenericImageView;
use image::io::Reader as ImageReader;
use crate::shader::ShaderProgram;
use crate::utils::gl_check_error;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use crate::texture::BasicTexture;

pub unsafe fn triangle_with_texture() -> (u32, u32, u32, u32) {
    let vertices: [f32; 32] = [
        // positions       // colors        // texture coords
        0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
    ];

    let indices = [
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];
    let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                   (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &indices[0] as *const i32 as *const c_void,
                   gl::STATIC_DRAW);

    let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
    // position attribute
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    // color attribute
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);
    // texture coord attribute
    gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(2);

    // load and create a texture
    // -------------------------
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
    let img = ImageReader::open("resources/textures/brickwally.jpg").unwrap().decode().unwrap();
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
    gl::GenerateMipmap(gl::TEXTURE_2D);

    (vbo, vao, ebo, texture)
}

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

#[allow(non_snake_case)]
pub unsafe fn learn_open_gl_coordinate_systems_example() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context =
        ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    gl::load_with(|address| windowed_context.get_proc_address(address) as *const _);


    // build and compile our shader program

    let shader_program = ShaderProgram::create_from_shader_paths("resources/shaders/6.1.coordinate_systems_vs.glsl",
                                                                 "resources/shaders/6.1.coordinate_systems_fs.glsl");

    // set up vertex data and configure attributes

    let vertices= [
        // positions             // texture coordinates
        0.5f32,  0.5f32, 0.0f32,  1.0f32, 1.0f32, // top right
        0.5f32, -0.5f32, 0.0f32,  1.0f32, 0.0f32, // bottom right
       -0.5f32, -0.5f32, 0.0f32,  0.0f32, 0.0f32, // bottom left
       -0.5f32,  0.5f32, 0.0f32,  0.0f32, 1.0f32  // top left
    ];

    let indices = [
        0, 1, 3, // first triangle
        1, 2, 3  // second triangle
    ];

    let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                   (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &indices[0] as *const i32 as *const c_void,
                   gl::STATIC_DRAW);

    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
    // position attribute
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    // texture coord attribute
    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(2);

    // load and create a texture
    // -------------------------
    let texture_1 = BasicTexture::new("resources/textures/wall.jpg");
    let texture_2 = BasicTexture::new("resources/textures/awesome-face.jpg");

    // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
    shader_program.use_program();
    shader_program.set_int("texture1", 0);
    shader_program.set_int("texture2", 1);

    el.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => {
                unsafe {
                    gl::DeleteVertexArrays(1, &vao);
                    // gl::DeleteBuffers(1, &vbo);
                    // gl::DeleteBuffers(1, &ebo);
                }
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size)
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            Event::RedrawRequested(_) => unsafe {
                // gl.draw_frame([1.0, 0.5, 0.7, 1.0]);

                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                // bind textures on corresponding texture units
                gl::ActiveTexture(gl::TEXTURE0);
                texture_1.bind();
                gl::ActiveTexture(gl::TEXTURE1);
                texture_2.bind();

                // activate shader
                shader_program.use_program();

                // create transformations
                let mut model = nalgebra_glm::identity::<f32, 4>();
                let mut view = nalgebra_glm::identity::<f32, 4>();
                let projection = nalgebra_glm::perspective(f32::to_radians(45f32), 800f32 / 600f32, 0.1f32, 100f32);

                model = nalgebra_glm::rotate(&model, f32::to_radians(-55f32), &nalgebra_glm::vec3(1f32, 0f32, 0f32));
                view = nalgebra_glm::translate(&view, &nalgebra_glm::vec3(0f32, 0f32, -3f32));

                // pass them to the shaders
                shader_program.set_mat4("model", &model);
                shader_program.set_mat4("view", &view);
                shader_program.set_mat4("projection", &projection);  // note: currently we set the projection matrix each frame, but since the projection matrix rarely changes it's often best practice to set it outside the main loop only once.

                // render container
                gl::BindVertexArray(vao);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}

pub unsafe fn transformation_matrices_needed_for_3d() {
    let mut model = nalgebra_glm::identity::<f32, 4>();
    model = nalgebra_glm::rotate(&model, f32::to_radians(-55f32), &nalgebra_glm::vec3(1f32, 0f32, 0f32));

    let mut view = nalgebra_glm::identity::<f32, 4>();
    view = nalgebra_glm::translate(&view, &nalgebra_glm::vec3(0f32, 0f32, -3f32));

    let mut projection = nalgebra_glm::identity::<f32, 4>();
    projection = nalgebra_glm::perspective(f32::to_radians(45f32), 800f32 / 600f32, 0.1f32, 100f32);
}