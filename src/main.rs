extern crate glium;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./scop <obj_file>");
        return;
    }

    // Open the given file and return if it fails.
    let _file = match std::fs::File::open(&args[1]) {
        Ok(file) => file,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    // Create an event loop.
    let event_loop = glutin::event_loop::EventLoop::new();
    // Create a context builder.
    let cb = glutin::ContextBuilder::new();
    // Create a window with the given parameters.
    let wb = glutin::window::WindowBuilder::new()
        .with_title("SCOP - Rust OpenGL 3D Model Viewer")
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0));
    // Create a display with the given parameters.
    let display = glium::Display::new(wb, cb, &event_loop)
        .unwrap();

    const TARGET_FPS: u64 = 60;

    // Loop until the user closes the window.
    event_loop.run(move |event, _, control_flow| {
        let start_time = std::time::Instant::now();

        // Handle events.
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

        let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
            true => 1000 / TARGET_FPS - elapsed_time,
            false => 0
        };

        let new_inst = start_time + std::time::Duration::from_millis(wait_millis);

        *control_flow =  glutin::event_loop::ControlFlow::WaitUntil(new_inst);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.finish().unwrap();
    });
}
