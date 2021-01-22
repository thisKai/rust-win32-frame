use {
    win32_frame::WindowSubclass,
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        platform::windows::WindowBuilderExtWindows,
    },
};
fn main() -> windows::Result<()> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_visible(false)
        .with_no_redirection_bitmap(true)
        .build(&event_loop)
        .unwrap();

    unsafe {
        window.apply_subclass()?;
    }
    window.set_visible(true);

    event_loop.run(move |event, _target, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => *control_flow = ControlFlow::Exit,
        _ => {}
    });
}
