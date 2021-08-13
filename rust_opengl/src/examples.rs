use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint};
use std::{mem, ptr};
use std::ffi::c_void;
use std::path::Path;
use image::GenericImageView;
use image::io::Reader as ImageReader;
use crate::shader::ShaderProgram;

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


pub unsafe fn learn_open_gl_coordinate_systems_example() -> (ShaderProgram, GLuint, GLuint, GLuint){
    // build and compile our shader program

    let shader_program = ShaderProgram::create_from_shader_paths("resources/shaders/6.1.coordinate_systems_vs.glsl",
                                                                 "resources/shaders/6.1.coordinate_systems_fs.glsl");

    // set up vertex data and configure attributes

    let vertices: [f32; 20] = [
        // positions             // texture coordinates
        0.5f32,  0.5f32, 0.0f32,  1.0f32, 1.0f32, // top right
        0.5f32, -0.5f32, 0.0f32,  1.0f32, 0.0f32, // bottom right
       -0.5f32, -0.5f32, 0.0f32,  0.0f32, 0.0f32, // bottom left
       -0.5f32,  0.5f32, 0.0f32,  0.0f32, 1.0f32  // top left
    ];

    let indices: [u8; 6] = [
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
                   &indices[0] as *const u8 as *const c_void,
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

    let mut texture2 = 0;
    gl::GenTextures(1, &mut texture2);
    gl::BindTexture(gl::TEXTURE_2D, texture2); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
    // set the texture wrapping parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    // set texture filtering parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    // load image, create texture and generate mipmaps
    let img = ImageReader::open("resources/textures/smiley_santa.jpg").unwrap().decode().unwrap();
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

    // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
    shader_program.use_program();
    shader_program.set_int("texture1", 0);
    shader_program.set_int("texture2", 1);

    (shader_program, vao as GLuint, texture as GLuint, texture2 as GLuint)

    // while (!glfwWindowShouldClose(window)) // example of while loop because this changes in the main.rs file
    // {
    //     // input
    //     // -----
    //     processInput(window);
    //
    //     // render
    //     // ------
    //     glClearColor(0.2f, 0.3f, 0.3f, 1.0f);
    //     glClear(GL_COLOR_BUFFER_BIT);
    //
    //     // bind textures on corresponding texture units
    //     glActiveTexture(GL_TEXTURE0);
    //     glBindTexture(GL_TEXTURE_2D, texture1);
    //     glActiveTexture(GL_TEXTURE1);
    //     glBindTexture(GL_TEXTURE_2D, texture2);
    //
    //     // activate shader
    //     ourShader.use();
    //
    //     // create transformations
    //     glm::mat4 model         = glm::mat4(1.0f); // make sure to initialize matrix to identity matrix first
    //     glm::mat4 view          = glm::mat4(1.0f);
    //     glm::mat4 projection    = glm::mat4(1.0f);
    //     model = glm::rotate(model, glm::radians(-55.0f), glm::vec3(1.0f, 0.0f, 0.0f));
    //     view  = glm::translate(view, glm::vec3(0.0f, 0.0f, -3.0f));
    //     projection = glm::perspective(glm::radians(45.0f), (float)SCR_WIDTH / (float)SCR_HEIGHT, 0.1f, 100.0f);
    //     // retrieve the matrix uniform locations
    //     unsigned int modelLoc = glGetUniformLocation(ourShader.ID, "model");
    //     unsigned int viewLoc  = glGetUniformLocation(ourShader.ID, "view");
    //     // pass them to the shaders (3 different ways)
    //     glUniformMatrix4fv(modelLoc, 1, GL_FALSE, glm::value_ptr(model));
    //     glUniformMatrix4fv(viewLoc, 1, GL_FALSE, &view[0][0]);
    //     // note: currently we set the projection matrix each frame, but since the projection matrix rarely changes it's often best practice to set it outside the main loop only once.
    //     ourShader.setMat4("projection", projection);
    //
    //     // render container
    //     glBindVertexArray(VAO);
    //     glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0);
    //
    //     // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    //     // -------------------------------------------------------------------------------
    //     glfwSwapBuffers(window);
    //     glfwPollEvents();
    // }
}

pub unsafe fn transformation_matrices_needed_for_3d() {
    let mut model = nalgebra_glm::identity::<f32, 4>();
    model = nalgebra_glm::rotate(&model, f32::to_radians(-55f32), &nalgebra_glm::vec3(1f32, 0f32, 0f32));

    let mut view = nalgebra_glm::identity::<f32, 4>();
    view = nalgebra_glm::translate(&view, &nalgebra_glm::vec3(0f32, 0f32, -3f32));

    let mut projection = nalgebra_glm::identity::<f32, 4>();
    projection = nalgebra_glm::perspective(f32::to_radians(45f32), 800f32 / 600f32, 0.1f32, 100f32);
}