mod utils;
mod tests;
mod shader;
mod examples;
mod texture;
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
use crate::examples::{learn_open_gl_coordinate_systems_example, triangle_with_texture};


// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    unsafe {
        // triangle_with_texture();
        learn_open_gl_coordinate_systems_example();
    }
}
