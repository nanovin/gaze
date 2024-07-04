use std::{
    io::Cursor,
    thread,
    time::{Duration, Instant},
};

use xcap::{Monitor, Window};

const SCREENSHOT_INTERVAL: u64 = 5; // seconds

struct DataFrame {
    screenshot: Vec<u8>,
    open_windows: Vec<String>,
    created_at: Instant,
}

fn screenshot_task() {
    let mut data_frames = Vec::new();
    loop {
        let start_time = Instant::now();
        let next_execution_time = start_time + Duration::from_secs(SCREENSHOT_INTERVAL);
        let sleep_duration = next_execution_time.saturating_duration_since(Instant::now());

        // Sleep until the next execution time
        thread::sleep(sleep_duration);

        take_screenshot(&mut data_frames);
    }
}

fn take_screenshot(data_frames: &mut Vec<DataFrame>) {
    let monitors = Monitor::all().unwrap();
    let windows = Window::all().unwrap();

    if let Some(monitor) = monitors.first() {
        if let Ok(screenshot) = monitor.capture_image() {
            println!("Captured screenshot");

            let mut bytes = Vec::new();
            let dynamic_image = image::DynamicImage::ImageRgba8(screenshot);
            if let Err(e) =
                dynamic_image.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
            {
                eprintln!("Failed to write screenshot: {:?}", e.to_string());
                return;
            }

            let data_frame = DataFrame {
                screenshot: bytes,
                open_windows: windows
                    .iter()
                    .filter(|w: &&Window| w.current_monitor().id() == monitor.id())
                    .map(|w| w.title().into())
                    .collect::<Vec<String>>(),
                created_at: Instant::now(),
            };

            data_frames.push(data_frame);
        }
    }
}

pub fn init_screenshot_worker() {
    thread::spawn(screenshot_task);
}
