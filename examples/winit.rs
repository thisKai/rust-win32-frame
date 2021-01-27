use winit::event::ElementState;

use {
    win32_frame::{Options, WithSubclass},
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::windows::WindowBuilderExtWindows,
        window::WindowBuilder,
    },
};
fn main() -> windows::Result<()> {
    let event_loop = EventLoop::new();

    let mut window = WindowBuilder::new()
        .with_visible(false)
        .with_no_redirection_bitmap(true)
        .build(&event_loop)
        .unwrap()
        .with_subclass(Options::custom_caption())?;

    window.set_visible(true);

    event_loop.run(move |event, _target, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => *control_flow = ControlFlow::Exit,
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            window_id,
        } if window_id == window.id() && input.state == ElementState::Pressed => {
            if let Some(key) = input.virtual_keycode {
                match key {
                    winit::event::VirtualKeyCode::Down => {
                        let mut options = window.options_mut();
                        options.extend_frame.top += 1;
                    }
                    winit::event::VirtualKeyCode::Up => {
                        let mut options = window.options_mut();
                        options.extend_frame.top -= 1;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
