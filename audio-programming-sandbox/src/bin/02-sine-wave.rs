use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device");
    let config = device.default_output_config()?;

    // Number of samples processed per second by the device
    let sample_rate = config.sample_rate() as f32;
    // This refers to how often a wave repeats itself per second (measured in Hertz)
    //
    // Try tinkering with this frequency. If you crank it up to really high frequencies, then you
    // get some aliasing (which is just unexpected sounds)
    let frequency = 440.0;
    // Refers to the horizontal shift of a periodic function
    let mut phase = 0.0;

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = (phase * 2.0 * std::f32::consts::PI).sin();
                phase += frequency / sample_rate;

                if phase >= 1.0 {
                    phase -= 1.0
                };
            }
        },
        |err| eprintln!("Error: {}", err),
        None,
    )?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(3));

    Ok(())
}
