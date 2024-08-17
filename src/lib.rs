use std::error::Error;
use gstreamer as gst;
use gst::prelude::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    // Initialize GStreamer
    gst::init().unwrap();

    // Create the elements
    let source = gst::ElementFactory::make("audiotestsrc")
        .name("source")
        .build()
        .expect("Could not create source element.");
    let sink = gst::ElementFactory::make("webrtcsink")
        .name("sink")
        .build()
        .expect("Could not create sink element");

    // Create the empty pipeline
    let pipeline = gst::Pipeline::with_name("test-pipeline");

    // Build the pipeline
    pipeline.add_many([&source, &sink]).unwrap();
    source.link(&sink).expect("Elements could not be linked.");

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
