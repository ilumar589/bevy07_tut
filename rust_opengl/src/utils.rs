use std::fs;
use std::error::Error;
use gl::types::GLenum;

// use the file!() and line!() macros to send file and line information to the function
pub unsafe fn gl_check_error(file: &str, line: u32) {
    let error_to_print =  match gl::GetError() {
        gl::NO_ERROR => "NO_ERROR",
        gl::INVALID_ENUM => "INVALID_ENUM",
        gl::INVALID_VALUE => "INVALID VALUE",
        gl::INVALID_OPERATION => "INVALID OPERATION",
        gl::STACK_OVERFLOW => "STACK OVERFLOW",
        gl::STACK_UNDERFLOW => "STACK UNDERFLOW",
        gl::OUT_OF_MEMORY => "OUT OF MEMORY",
        gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID FRAMEBUFFER OPERATION",
        _ => "ERROR CODE NOT MAPPED"
    };

    println!("{} | {} ({})", error_to_print, file, line);
}

pub fn read_file_contents(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}