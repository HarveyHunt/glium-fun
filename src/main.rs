#[macro_use]
extern crate glium;

use glium::{Surface, DisplayBuild};

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 2],
}

const VERT_SHADER_SRC: &'static str = include_str!("shaders/vert.glsl");
const FRAG_SHADER_SRC: &'static str = include_str!("shaders/frag.glsl");

fn main() {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    implement_vertex!(Vertex, pos);

    let triangle = vec![
        Vertex {pos: [0.5, -0.5]},
        Vertex {pos: [0.0, 0.5]},
        Vertex {pos: [-0.5, -0.5]},
    ];

    let vbo = glium::VertexBuffer::new(&display, &triangle).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, VERT_SHADER_SRC, FRAG_SHADER_SRC, None)
        .unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target.draw(&vbo,
                  &indices,
                  &program,
                  &glium::uniforms::EmptyUniforms,
                  &Default::default())
            .unwrap();

        target.finish().unwrap();

        for e in display.poll_events() {
            match e {
                glium::glutin::Event::Closed => return,
                _ => {}
            }
        }
    }
}
