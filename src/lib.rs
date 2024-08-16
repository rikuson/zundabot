use std::error::Error;
use gstreamer as gst;
use gst::prelude::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    // Initialize GStreamer
    gst::init().unwrap();

    // Build the pipeline
    let room_id = "1234";
    let pipeline = gst::parse::launch(&format!("videotestsrc ! janusvrwebrtcsink signaller::room-id={room_id}")).unwrap();

    // Start playing
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    Ok(())
}
