use pge::*;
use render::IndexBuffer;
use render::ShaderProgram;
use render::VArray;
use render::VbLayout;
use render::VertexBuffer;
use render::Texture;
use ressources::RessourceLoader;
use std::path::Path;
use window::App;

use gl::types::*;

fn main() {
    let mut app = App::new("Test", 1080, 720);
    let vertices = vec![
        0.5, 0.5, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0, 0.0, 1.0, // top left
    ];
    let indices = vec![
        // note that we start from 0!
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];
    // initialize the path for loading ressouce from
    let res = match RessourceLoader::init(Path::new("../../../examples/rectangle")) {
        Ok(res) => res,
        Err(_) => app.client_logger.fatal(&"Res path is wrong!".to_string()),
    };
    let texture = Texture::from_res(&app.gl, &res, "./Slimes/Slime_16x16.png");
    let vertex_array = VArray::new(&app.gl);
    let prog = ShaderProgram::from_res(&app.gl, &res, "rect").unwrap();
    let vertex_buffer = VertexBuffer::new(&app.gl);
    vertex_buffer.set_data::<f32>(&vertices);
    let mut vb_layout = VbLayout::new();
    vb_layout.push_f32(3 as GLint);
    vb_layout.push_f32(2 as GLint);

    vertex_array.add_buffer(&vertex_buffer, &vb_layout);
    let index_buffer = IndexBuffer::new(&app.gl);
    index_buffer.set_data::<u32>(&indices);

    let draw = || {
        texture.bind(0);
        prog.bind();
        vertex_array.draw_indexed(&index_buffer);
    };

    app.run(&draw);
}
