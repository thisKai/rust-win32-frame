use {
    minifb::{Key, Window, WindowOptions},
    win32_frame::{CustomWindowFrame, HitTestArea, Theme, WindowFrame},
};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() -> windows::Result<()> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    })
    .customize_frame(WindowFrame {
        theme: Some(Theme::Dark),
        intercept_client_area_hit_test: Some(Box::new(|_pos, _size| Some(HitTestArea::Caption))),
        ..WindowFrame::custom_sheet()
    })?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }
        let size = window.get_size();
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, size.0, size.1).unwrap();
    }
    Ok(())
}
