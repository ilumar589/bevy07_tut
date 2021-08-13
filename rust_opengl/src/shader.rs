use gl::types;
use std::{fs, ptr};
use std::ffi::CString;
use gl::types::{GLint, GLchar, GLfloat};
use std::str::from_utf8;

#[derive(Copy, Clone)]
pub struct ShaderProgram {
    pub id: types::GLuint
}

impl ShaderProgram {
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_bool(&self, name: &str, value: bool) {
        let c_name = CString::new(name).unwrap();
        gl::Uniform1i(gl::GetUniformLocation(self.id, c_name.as_ptr() as *const GLchar), value as GLint)
    }

    pub unsafe fn set_int(&self, name: &str, value: u64) {
        let c_name = CString::new(name).unwrap();
        gl::Uniform1i(gl::GetUniformLocation(self.id, c_name.as_ptr() as *const GLchar), value as GLint);
    }

    pub unsafe fn set_float(&self, name: &str, value: f64) {
        let c_name = CString::new(name).unwrap();
        gl::Uniform1f(gl::GetUniformLocation(self.id, c_name.as_ptr()), value as GLfloat);
    }

    pub unsafe fn set_vec2(&self, name: &str, value: &nalgebra_glm::TVec2<f32>) {
        let c_name = CString::new(name).unwrap();
        gl::Uniform2fv(gl::GetUniformLocation(self.id, c_name.as_ptr() as *const GLchar), 1, value.as_ptr() as *const GLfloat);
    }

    pub unsafe fn set_vec2f(&self, name: &str, x: f32, y: f32) {
        let c_name = CString::new(name).unwrap();
        gl::Uniform2f(gl::GetUniformLocation(self.id, c_name.as_ptr() as *const GLchar), x, y);
    }

    pub unsafe fn set_vec3(&self, name: &str, value: &nalgebra_glm::TVec3<f32>) {
        let c_name = CString::new(name).unwrap();
        gl::Uniform3fv(gl::GetUniformLocation(self.id, c_name.as_ptr() as *const GLchar), 1, value.as_ptr() as *const GLfloat);
    }

    pub unsafe fn set_vec3f(&self, name: &str, x: f32, y: f32, z: f32) {
        let c_name = CString::new(name).unwrap();
        gl::Uniform3f(gl::GetUniformLocation(self.id, c_name.as_ptr() as *const GLchar), x, y, z);
    }

    pub unsafe fn set_vec4(&self, name: &str, value: &nalgebra_glm::TVec4<f32>) {
        let c_name = CString::new(name).unwrap();
        gl::Uniform4fv(gl::GetUniformLocation(self.id, c_name.as_ptr() as *const GLchar), 1, value.as_ptr() as *const GLfloat);
    }

    pub unsafe fn set_vec4f(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        let c_name = CString::new(name).unwrap();
        gl::Uniform4f(gl::GetUniformLocation(self.id, c_name.as_ptr() as *const GLchar), x, y, z, w);
    }

    pub unsafe fn set_mat2(&self, name: &str, value: &nalgebra_glm::TMat2<f32>) {
        let c_name = CString::new(name).unwrap();
        gl::UniformMatrix2fv(gl::GetUniformLocation(self.id, c_name.as_ptr() as *const GLchar), 1, gl::FALSE, value.as_ptr() as *const GLfloat);
    }

    pub unsafe fn set_mat3(&self, name: &str, value: &nalgebra_glm::TMat3<f32>) {
        let c_name = CString::new(name).unwrap();
        gl::UniformMatrix3fv(gl::GetUniformLocation(self.id, c_name.as_ptr() as *const GLchar), 1, gl::FALSE, value.as_ptr() as *const GLfloat);
    }

    pub unsafe fn set_mat4(&self, name: &str, value: &nalgebra_glm::TMat4<f32>) {
        let c_name = CString::new(name).unwrap();
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, c_name.as_ptr() as *const GLchar), 1, gl::FALSE, value.as_ptr() as *const GLfloat);
    }
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