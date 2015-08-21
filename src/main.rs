#[macro_use]
extern crate glium;
extern crate genmesh;
extern crate clock_ticks;
extern crate obj;

use glium::Surface;
use glium::glutin;
use glium::index::PrimitiveType;

use std::thread;
use std::f32::consts::PI;

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(mut callback: F) where F: FnMut() -> Action {
    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };

        let now = clock_ticks::precise_time_ns();
        accumulator += now - previous_clock;
        previous_clock = now;

        const FIXED_TIME_STAMP: u64 = 16666667;
        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;

            // if you have a game, update the state here
        }

        thread::sleep_ms(((FIXED_TIME_STAMP - accumulator) / 1000000) as u32);
    }
}

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 3],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);

        glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0] },
                Vertex { position: [ 0.0,  0.5, 0.0], color: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
            ]
        ).unwrap()
    };

    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
                                               &[0u16, 1, 2]).unwrap();

    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 matrix;

                in vec3 position;
                in vec3 color;

                out vec3 vColor;

                void main() {
                    gl_Position = vec4(position, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 140
                in vec3 vColor;
                out vec4 f_color;

                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        }
    ).unwrap();
    let mut t = -PI;
    start_loop(|| {
        t += 0.02;
        if t > PI {
            t = -PI;
        }
        let uniforms = uniform! {
            matrix: [
                [t.cos(), 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return Action::Stop,
                _ => ()
            }
        }

        Action::Continue
    });
}
