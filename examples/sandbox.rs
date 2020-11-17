use pge::*;
use std::path::Path;
use window::App;
use ressources::Ressources;
use render::Vbuf;
use render::Ibuf;
use render::VbLayout;
use render::VArray;
use render::Program;
use render::GlObj;

use gl::types::*;

fn main() {
    let mut app = App::new("Test", 1080, 720);
    let vertices: Vec<_> = vec![
        -0.5,  -0.5, 0.0,
        0.5,  -0.5, 0.0,
        0.0,   0.5, 0.0
        ];
    let indices = vec![
        1,2,3
    ];

    // initialize the path for loading ressouce from
    let res = match Ressources::from_rel_path(Path::new("./")){
        Ok(res) => res,
        Err(_) => app.client_logger.fatal(&"Res path is wrong!".to_string()),
    };

    let prog = Program::from_res(&app.gl, &res, "tri").unwrap();
    let vertex_buffer = Vbuf::new(&app.gl, vertices);
    let index_buffer = Ibuf::new(&app.gl, indices);
    let mut vb_layout = VbLayout::new();
    vb_layout.push_f64(3 as GLint);

    let vertex_array = VArray::new(&app.gl);
    vertex_array.add_buffer(&vertex_buffer, &vb_layout);


    let draw = || {
        prog.bind();
        vertex_array.draw();
    };

    app.run(&draw);
}