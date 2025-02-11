use glium::winit::event::KeyEvent;
use glium::winit::event::*;
use glium::winit::keyboard::*;
use glium::{Surface, index::PrimitiveType};

#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    // A simple window can be created in winit/glutin using the following steps:
    // Create an Event Loop
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");

    // Make a simple SimpleWindowBuilder and setting the desired parameters.
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    // OpenGL First Draws Image to Memory, then that Image is transfered to the Window - Frame

    let shape = vec![
        Vertex {
            position: [0.0, 0.5],
        },
        Vertex {
            position: [0.5, -0.5],
        },
        Vertex {
            position: [-0.5, -0.5],
        },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // Start drawing here

    let vertex_shader_vertical = r#"
            #version 140

            in vec2 position;

            uniform float y;

            void main() {
            vec2 pos = position;
            pos.y += y;
            gl_Position = vec4(pos, 0.0,1.0);
            }
        "#;

    let vertex_shader_horizontal = r#"
        #version 140

        in vec2 position;

        uniform float x;

        void main() {
        vec2 pos = position;
        pos.x += x;
            gl_Position = vec4(pos, 0.0, 1.0);
        }

        "#;

    let fragment_shader_src = r#"

        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }

        "#;
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program_vertical =
        glium::Program::from_source(&display, vertex_shader_vertical, fragment_shader_src, None)
            .unwrap();

    let program_horizontal = glium::Program::from_source(
        &display,
        vertex_shader_horizontal,
        fragment_shader_src,
        None,
    )
    .unwrap();
    // let program =
    //     glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
    //         .unwrap();

    // Running an event loop to make sure the window doesn't close till a close windowevent is triggered.
    let mut t: f32 = 0.0;
    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                }

                glium::winit::event::WindowEvent::RedrawRequested => {
                    // Moving the Draw code here.
                    t += 0.02;
                    let x_off = t.sin();

                    let uniforms = uniform! {
                        x: x_off
                    };

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    target
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program_horizontal,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();

                    target.finish().unwrap();
                }
                _ => (),
            },

            glium::winit::event::Event::AboutToWait => {
                _window.request_redraw();
            }
            _ => (),
        };
    });
}
