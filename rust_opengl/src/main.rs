mod file_utils;
mod tests;
mod shader;
mod examples;

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


    let (shader_program, vao) = unsafe {
        let shader_program = ShaderProgram::create_from_shader_paths("resources/shaders/vertex_with_color.glsl",
                                                                     "resources/shaders/fragment_with_color.glsl");

        let vao = examples::triangle_with_color_attributes();

        (shader_program, vao)
    };

    el.run(move |event, _, control_flow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
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

                gl::UseProgram(shader_program.id);
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);

                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
