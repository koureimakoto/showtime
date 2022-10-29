use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, self, WindowId}
};


/* Definitivamente essa a maneira mais porca e desorganizada para se montar um event_loop saindo do tutorial não usarei mais isso */
pub fn 
run() {
    env_logger::init();
    let event_loop: EventLoop<()> = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
 
    event_loop.run(
        move |event,
        _,
        control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested |
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        }
    );
}
