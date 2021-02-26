use crate::file_utils::read_file_contents;
use std::fs::File;


#[test]
fn read_vertex_shader() {
    let file_contents = read_file_contents("resources/shaders/vertex.glsl");

    assert_eq!(file_contents.is_empty(), false);
}