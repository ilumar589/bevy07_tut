use gl::types;
use std::{fs, ptr};
use std::ffi::CString;
use gl::types::{GLint, GLchar};
use std::str::from_utf8;

pub struct ShaderProgram {
    pub id: types::GLuint
}

impl ShaderProgram {
    pub unsafe fn create_from_shader_paths(vertex_path: &str, fragment_path: &str) -> Self {
        // vertex shader
        let vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(fs::read_to_string(vertex_path).unwrap()).unwrap();
        gl::ShaderSource(vertex_shader_id, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader_id);

        // check for shader compile errors
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character

        gl::GetShaderiv(vertex_shader_id, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(vertex_shader_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", from_utf8(&info_log).unwrap());
        }

        // fragment shader
        let fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(fs::read_to_string(fragment_path).unwrap()).unwrap();
        gl::ShaderSource(fragment_shader_id, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader_id);
        // check for shader compile errors
        gl::GetShaderiv(fragment_shader_id, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(fragment_shader_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", from_utf8(&info_log).unwrap());
        }

        // link shaders
        let shader_program_id = gl::CreateProgram();
        gl::AttachShader(shader_program_id, vertex_shader_id);
        gl::AttachShader(shader_program_id, fragment_shader_id);
        gl::LinkProgram(shader_program_id);
        // check for linking errors
        gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(shader_program_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", from_utf8(&info_log).unwrap());
        }
        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);

        ShaderProgram {
            id: shader_program_id
        }
    }
}