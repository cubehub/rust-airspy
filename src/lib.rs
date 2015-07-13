/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Andres Vahter (andres.vahter@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#[macro_use]
extern crate log;
extern crate libc;

#[allow(non_camel_case_types)]
pub mod ffiairspy;

use std::mem;
use std::ptr;

#[derive(Debug)]
pub enum AirspyError {
    InvalidParam,
    NotFound,
    Busy,
    NoMem,
    LibUsb,
    Thread,
    StreamingThreadErr,
    StreamingStopped,
    Other,
}

fn convert_to_airspy_error(error: i32) -> AirspyError {
    match error {
        ffiairspy::AIRSPY_ERROR_INVALID_PARAM => AirspyError::InvalidParam,
        ffiairspy::AIRSPY_ERROR_NOT_FOUND => AirspyError::NotFound,
        ffiairspy::AIRSPY_ERROR_BUSY => AirspyError::Busy,
        ffiairspy::AIRSPY_ERROR_NO_MEM => AirspyError::NoMem,
        ffiairspy::AIRSPY_ERROR_LIBUSB => AirspyError::LibUsb,
        ffiairspy::AIRSPY_ERROR_THREAD => AirspyError::Thread,
        ffiairspy::AIRSPY_ERROR_STREAMING_THREAD_ERR => AirspyError::StreamingThreadErr,
        ffiairspy::AIRSPY_ERROR_STREAMING_STOPPED => AirspyError::StreamingStopped,
        ffiairspy::AIRSPY_ERROR_OTHER => AirspyError::Other,
        err => panic!("unknown airspy error code ({})", err),
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AirspySamplerate {
    Sps10000000 = 10_000_000,
    Sps2500000  =  2_500_000,
}

pub struct Airspy {
    device: *mut ffiairspy::Struct_airspy_device,
}

impl Airspy {

    pub fn new() -> Result<Airspy, AirspyError> {
        let mut device = ptr::null_mut();

        let error = unsafe {ffiairspy::airspy_open(&mut device)};
        match error {
            ffiairspy::AIRSPY_SUCCESS => Ok(Airspy {device: device}),
            ffiairspy::AIRSPY_ERROR_NO_MEM => Err(convert_to_airspy_error(ffiairspy::AIRSPY_ERROR_NO_MEM)),
            ffiairspy::AIRSPY_ERROR_LIBUSB => Err(convert_to_airspy_error(ffiairspy::AIRSPY_ERROR_LIBUSB)),
            err => panic!("airspy_open returned error ({:})", err),
        }
    }

    pub fn set_samplerate(&self, samplerate: AirspySamplerate) -> Result<(), AirspyError>{
        let mut supported_samplerates_count: u32 = 0;

        // get count of supported samplerates
        let mut error = unsafe {ffiairspy::airspy_get_samplerates(self.device, &mut supported_samplerates_count as *mut u32, 0)};
        // anything except SUCCESS is library failure, hence panic!
        match error {
            ffiairspy::AIRSPY_SUCCESS => {},
            err => panic!("airspy_get_samplerates returned error ({:})", err),
        }

        // get list of supported samplerates
        let mut samplerates: Vec<u32> = vec![0_u32; supported_samplerates_count as usize];
        error = unsafe {ffiairspy::airspy_get_samplerates(self.device, mem::transmute::<&mut u32, *mut u32>(&mut samplerates[0]), supported_samplerates_count)};
        // supported_samplerates_count must be right because we just asked for it from library, there must be lib error if anything other than SUCCESS is returned
        match error {
            ffiairspy::AIRSPY_SUCCESS => {},
            err => panic!("airspy_get_samplerates returned error ({:})", err),
        }

        let sr = samplerate.clone() as u32;
        // binary search assumes sorted list
        samplerates.sort();
        match samplerates.binary_search(&sr) {
            Ok(index) => {
                let error = unsafe {ffiairspy::airspy_set_samplerate(self.device, index as u32)};
                match error {
                    ffiairspy::AIRSPY_SUCCESS => Ok(()),
                    ffiairspy::AIRSPY_ERROR_LIBUSB => Err(convert_to_airspy_error(ffiairspy::AIRSPY_ERROR_LIBUSB)),
                    err => panic!("airspy_set_samplerate returned error ({:})", err),
                }
            },
            Err(_) => {
                panic!("samplerate {:?} not supported by libairspy, supported samplerates are {:?}", samplerate, samplerates)
            },
        }
    }
}

impl Drop for Airspy {
    fn drop(&mut self) {
        unsafe{ffiairspy::airspy_close(self.device)};
        unsafe{ffiairspy::airspy_exit()};
    }
}
