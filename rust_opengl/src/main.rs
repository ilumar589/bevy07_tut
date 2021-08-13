mod file_utils;
mod tests;
mod shader;
mod examples;
// mod ecs_experiments;

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use crate::shader::ShaderProgram;


// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

#[allow(non_snake_case)]
fn main() {
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


    let (shader_program, vao, texture1, texture2) = unsafe {
        examples::learn_open_gl_coordinate_systems_example()
    };

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
                gl::BindTexture(gl::TEXTURE_2D,texture1);
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D,texture2);

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
