use std::fs::File;
use std::fs;


#[test]
fn read_vertex_shader() {
    let file_contents =  fs::read_to_string("resources/shaders/vertex.glsl").unwrap();

    assert_eq!(file_contents.is_empty(), false);
}