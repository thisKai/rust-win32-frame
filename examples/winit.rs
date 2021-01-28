use winit::event::ElementState;

use {
    win32_frame::{CustomWindowFrame, HitTestArea, WindowFrame},
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
        .with_no_redirection_bitmap(true)
        .build(&event_loop)
        .unwrap()
        .customize_frame(WindowFrame {
            intercept_client_area_hit_test: Some(Box::new(|_pos, _size| Some(HitTestArea::Caption))),
            ..WindowFrame::sheet()
        })?;

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
                        let mut options = window.edit_custom_frame();
                        options.extend_frame.top += 1;
                    }
                    winit::event::VirtualKeyCode::Up => {
                        let mut options = window.edit_custom_frame();
                        options.extend_frame.top -= 1;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
