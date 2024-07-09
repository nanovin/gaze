use super::{embed::embed_text, ocr::image_to_text, state::GazeState};
use image::DynamicImage;
use std::time::{Duration, Instant};
use tokio::task::spawn_blocking;
use xcap::{Monitor, Window};

const SCREENSHOT_INTERVAL: u64 = 90; // seconds

// an future that sits in the event loop and calls take_screenshot
// every SCREENSHOT_INTERVAL. thats it.
async fn screenshot_task(app_state: GazeState) {
    loop {
        let start_time = Instant::now();
        let next_execution_time = start_time + Duration::from_secs(SCREENSHOT_INTERVAL);
        let sleep_duration = next_execution_time.saturating_duration_since(Instant::now());

        // Sleep until the next execution time
        tokio::time::sleep(sleep_duration.into()).await;

        take_screenshot(app_state.clone()).await.unwrap();
    }
}

// grabs to PID of the currently focused window.
// currently only supports windows :/
fn get_focused_pid() -> u32 {
    let foreground_pid: u32;

    // find focused window
    unsafe {
        let focused_handle = winapi::um::winuser::GetForegroundWindow();
        let mut focused_pid = 0;
        winapi::um::winuser::GetWindowThreadProcessId(focused_handle, &mut focused_pid);
        foreground_pid = focused_pid;
    }

    foreground_pid
}

// phashes img and checks against last img phash
// returns true if they are too similar
fn should_store_image(img: &DynamicImage) -> bool {
    true
}

// takes a screenshot of the first monitor (usually the primary monitor)
// looks at the currently focused window name
// runs OCR on the screenshot -> embeds the result -> stores the image and
// inserts the embeddings + metadata into the vdb
async fn take_screenshot(gaze_state: GazeState) -> Result<(), Box<dyn std::error::Error>> {
    let monitors = Monitor::all()?;
    let windows = Window::all()?;

    if let Some(monitor) = monitors.into_iter().next() {
        let screenshot = spawn_blocking(move || monitor.capture_image()).await??;
        println!("Captured screenshot");

        let screenshot = image::DynamicImage::ImageRgba8(screenshot);

        if !should_store_image(&screenshot) {
            println!("phash too similar to last screenshot, will skip this frame");
            return Ok(());
        }

        let focused_pid = get_focused_pid();
        let focused_window_title =
            if let Some(window) = windows.into_iter().find(|w| w.process_id() == focused_pid) {
                window.title().to_string()
            } else {
                "Unknown".to_string()
            };

        println!("Focused Window Title: {}", focused_window_title);

        let (ocr_text, embeddings) = spawn_blocking(move || {
            let ocr_text = image_to_text(&screenshot);
            println!("OCR Text: {}", ocr_text);
            let embeddings = embed_text(ocr_text.as_str()).unwrap();
            println!("Embeddings: {:?}", embeddings);
            (ocr_text, embeddings)
        })
        .await?;

        let id = gaze_state
            .lock()
            .await
            .store_embeddings(embeddings, &ocr_text, focused_window_title.as_str())
            .await?;

        println!("Stored embeddings for id: {}", id);

        // save image for reference
        // dynamic_image.save(format!("../screenshots/{}.png", id).as_str())?;
    }

    Ok(())
}

// chucks the screenshot task future into the event loop
pub fn init_screenshot_worker(app_state: GazeState) {
    tokio::spawn(screenshot_task(app_state));
}
