// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, thread};
use rscam::{Camera, Config};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let app_weak = ui.as_weak();

    let mut camera = Camera::new("/dev/video0").unwrap();
    camera
        .start(&Config {
            interval: (1, 10),
            resolution: (1280, 720),
            format: b"RGB3",
            ..Default::default()
        }).unwrap();
    
    thread::spawn(move || {
        loop {
            let frame = camera.capture().unwrap();
            let mut pixel_buffer = slint::SharedPixelBuffer::<slint::Rgb8Pixel>::new(frame.resolution.0, frame.resolution.1);
            pixel_buffer.make_mut_bytes().copy_from_slice(&frame[..]);
            app_weak.upgrade_in_event_loop(|app| {
                app.set_viewfinder(slint::Image::from_rgb8(pixel_buffer));
            }).unwrap();
        }
    });

    ui.run()?;

    Ok(())
}
