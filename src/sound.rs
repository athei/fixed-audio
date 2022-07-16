use coreaudio::sys::*;
use std::{mem, os::raw::c_void, ptr::null};

pub fn set_default_device_id(input: bool, id: AudioDeviceID) {
    let selector = if input {
        kAudioHardwarePropertyDefaultInputDevice
    } else {
        kAudioHardwarePropertyDefaultOutputDevice
    };
    let property_address = AudioObjectPropertyAddress {
        mSelector: selector,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

    let data_size = mem::size_of::<AudioDeviceID>();
    unsafe {
        AudioObjectSetPropertyData(
            kAudioObjectSystemObject,
            &property_address as *const _,
            0,
            null(),
            data_size as u32,
            &id as *const _ as *const _,
        )
    };
}

pub struct DefaultInputListener {
    property_address: AudioObjectPropertyAddress,
    callback: Box<dyn Fn(AudioObjectID)>,
}

impl Drop for DefaultInputListener {
    fn drop(&mut self) {
        let _ = self.unregister();
    }
}

impl DefaultInputListener {
    pub fn new(callback: Box<dyn Fn(AudioObjectID)>) -> Box<Self> {
        let property_address = AudioObjectPropertyAddress {
            mSelector: kAudioHardwarePropertyDefaultInputDevice,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMaster,
        };
        let mut ret = Box::new(Self {
            property_address,
            callback,
        });
        ret.register();
        ret
    }

    fn register(&mut self) {
        // Add our listener callback.
        let status = unsafe {
            AudioObjectAddPropertyListener(
                kAudioObjectSystemObject,
                &self.property_address as *const _,
                Some(alive_listener),
                self as *const _ as *mut _,
            )
        };
        assert!(status == 0);
    }

    fn unregister(&mut self) {
        let status = unsafe {
            AudioObjectRemovePropertyListener(
                kAudioObjectSystemObject,
                &self.property_address as *const _,
                Some(alive_listener),
                self as *const _ as *mut _,
            )
        };
        assert!(status == 0);
    }
}

unsafe extern "C" fn alive_listener(
    _device_id: AudioObjectID,
    _n_addresses: u32,
    _properties: *const AudioObjectPropertyAddress,
    self_ptr: *mut c_void,
) -> OSStatus {
    let self_ptr: &mut DefaultInputListener = &mut *(self_ptr as *mut DefaultInputListener);
    let data_size = mem::size_of::<AudioDeviceID>();
    let device_id: AudioDeviceID = 1;
    let result = AudioObjectGetPropertyData(
        kAudioObjectSystemObject,
        &self_ptr.property_address as *const _,
        0,
        null(),
        &data_size as *const _ as *mut _,
        &device_id as *const _ as *mut _,
    );
    (self_ptr.callback)(device_id);
    result
}
