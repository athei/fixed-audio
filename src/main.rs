mod sound;

use coreaudio::audio_unit::macos_helpers as helpers;
use crate::sound::set_default_device_id;

fn main() {
    let default = helpers::get_default_device_id(true).unwrap();
    println!("default_input: {:?}", default);
    for id in helpers::get_audio_device_ids().unwrap() {
        let name = helpers::get_device_name(id).unwrap();
        println!("{}: {}", id, name);
    }
    let _listener = sound::DefaultInputListener::new(Box::new(move |device| {
        println!("callback: {}", device);
        if device != default {
            set_default_device_id(true, default);
        }
    }));
    unsafe {
        coreaudio::sys::CFRunLoopRun();
    }
}
