use coreaudio::audio_unit::macos_helpers as helpers;
mod sound;

fn main() {
    let default = (
        helpers::get_default_device_id(false),
        helpers::get_default_device_id(true),
    );
    println!("default: {:?}", default);
    for id in helpers::get_audio_device_ids().unwrap() {
        let name = helpers::get_device_name(id).unwrap();
        println!("{}: {}", id, name);
    }
    let _listener = sound::DefaultInputListener::new();
    unsafe {
        coreaudio::sys::CFRunLoopRun();
    }
}
