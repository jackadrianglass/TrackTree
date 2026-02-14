use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> anyhow::Result<()> {
    // Host is what lets you get access to all the audio devices on the system
    let host = cpal::default_host();

    // Device is something that has any number of input and output streams
    let device = host.default_output_device().expect("no output device");

    // Device configuration
    //
    // - Determines the constant of how the loop actually operates
    // - sample rate -> how frequent is the sample?
    // - buffer size -> how big is the buffer that you need to write to
    // - channels -> ???
    let config = device.default_output_config()?;

    // The audio playback loop
    // 
    // - Audio device has a buffer of samples that it will play
    // - The buffer has a sampling rate that represents buckets of time (digital signal) rather
    //   than a continuous wave (analog signal)
    // - Since human ears are really good at detecting changes, the sample rate is really high and
    //   the output callback is callback is called many many times per second
    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // Data is the audio buffer
            // - Literally a buffer of normalized values (-1.0 to 1.0)
            // - This buffer can be thought of as the position of the speaker relative to center
            // - As you change the position, then speaker moves back and forth creating sound waves
            //   at different frequencies
            for sample in data.iter_mut() {
                *sample = 0.0; // Replace this with your audio generation
            }
        },
        |err| eprintln!("Error: {}", err),
        None,
    )?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    Ok(())
}
