use anyhow::Result; 
use opencv::core::bitwise_not;
use opencv::{
    prelude::*,
    videoio,
    core::*,
    highgui
}; // Note, the namespace of OpenCV is changed (to better or worse). It is no longer one enormous.
fn main() -> Result<()> { // Note, this is anyhow::Result
    // Open a GUI window
    highgui::named_window("window", highgui::WINDOW_NORMAL)?;
    // Open the web-camera (assuming you have one)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2)?;
    cam.set(videoio::CAP_PROP_FRAME_WIDTH,1280.0)?;
    cam.set(videoio::CAP_PROP_FRAME_HEIGHT,720.0)?;

    let mut frame1 = Mat::default(); // This array will store the web-cam data
    let nothing = Mat::default(); // This array will store the web-cam data
    // Read the camera
    // and display in the window
    let mut last_frame = Mat::default();
    cam.read(&mut frame1)?;

    cam.read(&mut last_frame)?;

    loop {
        cam.read(&mut frame1)?;
        let mut frame2 = frame1.clone();
        bitwise_not(&last_frame, &mut frame2, &nothing)?;
        let mut added = Mat::default(); 
        add_weighted(&frame1, 0.5, &frame2, 0.5, 1.0, &mut added, -1)?;
        highgui::imshow("window", &added)?;
        let key = highgui::wait_key(1)?;
        if key == 113 { // quit with q
            break;
        }
        last_frame = frame1.clone();
    }
    Ok(())
}