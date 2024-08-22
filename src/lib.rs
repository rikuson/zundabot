use std::error::Error;
use gstreamer as gst;
use gst::prelude::*;
use vvcore::*;

const CHUNK_SIZE: usize = 1024; // Amount of bytes we are sending in each buffer
const SAMPLE_RATE: u32 = 24_000; // Samples per second we are sending

pub fn run() -> Result<(), Box<dyn Error>> {
    // Initialize GStreamer
    gst::init().unwrap();

    let dir = std::ffi::CString::new("/usr/local/lib/open_jtalk_dic_utf_8-1.11").unwrap();
    let vvc = VoicevoxCore::new_from_options(AccelerationMode::Auto, 0, true, dir.as_c_str()).unwrap();

    let text: &str = "こんにちは";
    let speaker: u32 = 1;
    let wav = vvc.tts_simple(text, speaker).unwrap();

    let info = gstreamer_audio::AudioInfo::builder(gstreamer_audio::AudioFormat::S16le, SAMPLE_RATE, 1)
        .build()
        .unwrap();
    let audio_caps = info.to_caps().unwrap();

    // Create the elements
    let source = gstreamer_app::AppSrc::builder()
        .name("audio_source")
        .caps(&audio_caps)
        .format(gst::Format::Bytes)
        .build();
    let converter = gst::ElementFactory::make("audioconvert")
        .name("converter")
        .build()
        .expect("Could not create converter element");
    let resampler = gst::ElementFactory::make("audioresample")
        .name("resampler")
        .build()
        .expect("Could not create resampler element");
    let sink = gst::ElementFactory::make("autoaudiosink")
        .name("sink")
        .build()
        .expect("Could not create sink element");

    // Create the empty pipeline
    let pipeline = gst::Pipeline::with_name("test-pipeline");

    // Build the pipeline
    pipeline.add_many([source.upcast_ref(), &converter, &resampler, &sink]).unwrap();
    gst::Element::link_many(&[source.upcast_ref(), &converter, &resampler, &sink]).unwrap();

    let wav_data = wav.as_slice().to_vec();
    let mut buffer = gst::Buffer::with_size(wav_data.len()).map_err(|_| gst::FlowError::Error)?;
    {
        let buffer_ref = buffer.get_mut().ok_or(gst::FlowError::Error)?;
        let mut map = buffer_ref.map_writable().map_err(|_| gst::FlowError::Error)?;

        map.as_mut_slice().copy_from_slice(&wav_data);
    }
    source.push_buffer(buffer)?;

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
