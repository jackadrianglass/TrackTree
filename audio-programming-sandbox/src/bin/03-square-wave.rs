use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// See https://en.wikipedia.org/wiki/Square_wave_(waveform)
//
// For the math. It's basically a sign function on the sine wave
fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device");
    let config = device.default_output_config()?;

    let sample_rate = config.sample_rate() as f32;
    let frequency = 440.0;
    let mut phase = 0.0;

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = if (phase * 2.0 * std::f32::consts::PI).sin() > 0.0 {
                    1.0
                } else {
                    -1.0
                };
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
