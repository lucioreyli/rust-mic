use cpal::{traits::{HostTrait, DeviceTrait, StreamTrait}, Sample};

fn main() {
    let host = cpal::default_host();                                    
    let default_host = host.default_input_device().expect("doesn't have input device");
    let mut supported_config_range = default_host.supported_input_configs().expect("device not supported");
    let supported_config = supported_config_range.next().expect("not supported config").with_max_sample_rate();
    let default_config = supported_config.config();
    let config = default_config;
                
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let stream = default_host.build_input_stream(
        &config,    move |data: &[f32], callback_info: &cpal::InputCallbackInfo| {
            println!("{:?}", callback_info);
        },
        err_fn,
        None
    ).unwrap();
    let a = stream.play().expect("err");
    println!("done!, {:?}", a);
    loop {}
}

fn _write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::EQUILIBRIUM;
    }
}

