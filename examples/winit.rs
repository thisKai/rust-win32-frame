use {
    win32_frame::{Margins, Options, WithSubclass},
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::windows::WindowBuilderExtWindows,
        window::WindowBuilder,
    },
};
fn main() -> windows::Result<()> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_visible(false)
        .with_no_redirection_bitmap(true)
        .build(&event_loop)
        .unwrap()
        .with_subclass(Options {
            extend_frame: Margins {
                left: 20,
                top: 100,
                right: 20,
                bottom: 20,
            },
            extend_client_area: Margins::default_caption(),
            hit_test_extended_caption: true,
            ..Default::default()
        })?;

    window.set_visible(true);

    event_loop.run(move |event, _target, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => *control_flow = ControlFlow::Exit,
        _ => {}
    });
}
