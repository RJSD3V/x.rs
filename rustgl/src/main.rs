use glium::Surface;
fn main() {
    // A simple window can be created in winit/glutin using the following steps:
    // Create an Event Loop
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");

    // Make a simple SimpleWindowBuilder and setting the desired parameters.
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    // OpenGL First Draws Image to Memory, then that Image is transfered to the Window - Frame

    let mut target = display.draw();

    target.clear_color(0.0, 0.0, 1.0, 1.0);

    target.finish().unwrap();

    // Running an event loop to make sure the window doesn't close till a close windowevent is triggered.
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
