#[macro_use]
extern crate glium;
extern crate cgmath;

use glium::{Surface, DisplayBuild};
use cgmath::{Vector4, SquareMatrix};

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 2],
}

const VERT_SHADER_SRC: &'static str = include_str!("shaders/vert.glsl");
const FRAG_SHADER_SRC: &'static str = include_str!("shaders/frag.glsl");

fn main() {
    let mut t: f32 = 0.1;
    let mut i: f32 = -1.0;
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
        t += 0.03 * i;
        if t > 0.5 || t < -0.5 {
            i = -i;
        }

        let mut matrix = cgmath::Matrix4::identity();
        matrix.x = Vector4::new(t.cos(), t.sin(), 0.0, 0.0);
        matrix.y = Vector4::new(-t.sin(), t.cos(), 0.0, 0.0);

        let uniforms = uniform! {
            matrix: Into::<[[f32; 4]; 4]>::into(matrix),
        };

        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target.draw(&vbo, &indices, &program, &uniforms, &Default::default())
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
