extern crate libpulse_sys;

use libpulse_sys::*;
use std::ptr::{null, null_mut};
use std::{mem, slice};
use std::ffi::CString;

const SAMPLES: usize = 240;

fn main() {
    let ss = pa_sample_spec {
        format: PA_SAMPLE_S16LE,
        channels: 1,
        rate: 48000
    };
    let s = unsafe {
        let name_c = CString::new("Rust!").unwrap();
        let desc_c = CString::new("Example").unwrap();
        pa_simple_new(null(),             // Use the default server.
                      name_c.as_ptr() as *const i8,  // Our application's name.
                      PA_STREAM_PLAYBACK,
                      null(),             // Use the default device.
                      desc_c.as_ptr() as *const i8,  // Description of our stream.
                      &ss,                // Our sample format.
                      null(),             // Use default channel map
                      null(),             // Use default buffering attributes.
                      null_mut(),         // Ignore error code.
                     )
    };
    assert!(s != null_mut());

    let buf = unsafe { mem::transmute(pa_xmalloc(2 * SAMPLES)) };
    loop {
        let samples: &mut [i16] = unsafe { slice::from_raw_parts_mut(buf, SAMPLES) };
        for (i, sample) in samples.iter_mut().enumerate() {
            *sample = i as i16 * 100;
        }
        let res = unsafe {
            pa_simple_write(s, mem::transmute(buf), 2 * SAMPLES, null_mut())
        };
        assert!(res == 0);
    }
}
