use pge::*;
use render::ShaderProgram;
use render::VArray;
use render::VbLayout;
use render::VertexBuffer;
use ressources::RessourceLoader;
use std::path::Path;
use window::App;

use gl::types::*;

fn main() {
    let mut app = App::new("Test", 1080, 720);
    let vertices: Vec<_> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    // initialize the path for loading ressouce from
    let res = match RessourceLoader::init(Path::new("../../../examples/triangle")) {
        Ok(res) => res,
        Err(_) => app.client_logger.fatal(&"Res path is wrong!".to_string()),
    };

    let prog = ShaderProgram::from_res(&app.gl, &res, "tri").unwrap();
    let vertex_buffer = VertexBuffer::new(&app.gl);
    vertex_buffer.set_data::<f32>(&vertices);
    let mut vb_layout = VbLayout::new();
    vb_layout.push_f32(3 as GLint);

    let vertex_array = VArray::new(&app.gl);
    vertex_array.add_buffer(&vertex_buffer, &vb_layout);

    let draw = || {
        prog.bind();
        vertex_array.draw();
    };

    app.run(&draw);
}
