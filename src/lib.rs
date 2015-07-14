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
    InvalidParam       = ffiairspy::AIRSPY_ERROR_INVALID_PARAM as isize,
    NotFound           = ffiairspy::AIRSPY_ERROR_NOT_FOUND as isize,
    Busy               = ffiairspy::AIRSPY_ERROR_BUSY as isize,
    NoMem              = ffiairspy::AIRSPY_ERROR_NO_MEM as isize,
    LibUsb             = ffiairspy::AIRSPY_ERROR_LIBUSB as isize,
    Thread             = ffiairspy::AIRSPY_ERROR_THREAD as isize,
    StreamingThreadErr = ffiairspy::AIRSPY_ERROR_STREAMING_THREAD_ERR as isize,
    StreamingStopped   = ffiairspy::AIRSPY_ERROR_STREAMING_STOPPED as isize,
    Other              = ffiairspy::AIRSPY_ERROR_OTHER as isize,
}

#[derive(Debug, Copy, Clone)]
pub enum AirspySamplerate {
    Sps10000000 = 10_000_000,
    Sps2500000  =  2_500_000,
}

#[derive(Debug)]
pub enum AirspySampleType {
    /// 2 * 32bit float per sample
    Float32IQ   = ffiairspy::AIRSPY_SAMPLE_FLOAT32_IQ as isize,
    /// 1 * 32bit float per sample
    Float32Real = ffiairspy::AIRSPY_SAMPLE_FLOAT32_REAL as isize,
    /// 2 * 16bit int per sample
    Int16IQ     = ffiairspy::AIRSPY_SAMPLE_INT16_IQ as isize,
    /// 1 * 16bit int per sample
    Int16Real   = ffiairspy::AIRSPY_SAMPLE_INT16_REAL as isize,
    /// 1 * 16bit unsigned int per sample
    Uint16Real  = ffiairspy::AIRSPY_SAMPLE_UINT16_REAL as isize,
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
            ffiairspy::AIRSPY_ERROR_NO_MEM => Err(AirspyError::NoMem),
            ffiairspy::AIRSPY_ERROR_LIBUSB => Err(AirspyError::LibUsb),
            err => panic!("airspy_open returned error ({:})", err),
        }
    }

    pub fn set_samplerate(&self, samplerate: AirspySamplerate) -> Result<(), AirspyError> {
        let mut supported_samplerates_count: u32 = 0;

        // get count of supported samplerates
        let mut error = unsafe {ffiairspy::airspy_get_samplerates(self.device, &mut supported_samplerates_count as *mut u32, 0)};
        // anything except SUCCESS is library failure, hence panic!
        match error {
            ffiairspy::AIRSPY_SUCCESS => {},
            err => panic!("airspy_get_samplerates returned error ({:})", err),
        }

        assert!(supported_samplerates_count == ffiairspy::AIRSPY_SAMPLERATE_END);

        // get list of supported samplerates
        let mut samplerates: Vec<u32> = vec![0_u32; supported_samplerates_count as usize];
        error = unsafe {ffiairspy::airspy_get_samplerates(self.device, mem::transmute::<&mut u32, *mut u32>(&mut samplerates[0]), supported_samplerates_count)};
        // supported_samplerates_count must be right because we just asked for it from library, there must be lib error if anything other than SUCCESS is returned
        match error {
            ffiairspy::AIRSPY_SUCCESS => {},
            err => panic!("airspy_get_samplerates returned error ({:})", err),
        }

        let sr = samplerate.clone() as u32;
        let mut index: Option<u32> = None;
        for (i, rate) in samplerates.iter().enumerate() {
            if *rate == sr {
                index = Some(i as u32);
                break;
            }
        }

        if index.is_some() {
            let error = unsafe {ffiairspy::airspy_set_samplerate(self.device, index.unwrap())};
            match error {
                ffiairspy::AIRSPY_SUCCESS => Ok(()),
                ffiairspy::AIRSPY_ERROR_LIBUSB => Err(AirspyError::LibUsb),
                err => panic!("airspy_set_samplerate returned error ({:})", err),
            }
        }
        else {
            panic!("samplerate {:?} not supported by libairspy, supported samplerates are {:?}", samplerate, samplerates);
        }
    }

    pub fn set_sample_type(&self, sampletype: AirspySampleType) -> Result<(), AirspyError> {
        match unsafe {ffiairspy::airspy_set_sample_type(self.device, sampletype as u32)} {
            ffiairspy::AIRSPY_SUCCESS => {
                Ok(())
            },
            err => {
                // airspy_set_sample_type should only return SUCCESS
                // however just in case panic here if for some reason it does anything else
                panic!("airspy_set_sample_type returned error {}", err)
            },
        }
    }

    pub fn set_bias_tee(&self, onoff: bool) -> Result<(), AirspyError> {
        let c_onoff = match onoff {
            true => 1,
            false => 0,
        };

        match unsafe {ffiairspy::airspy_set_rf_bias(self.device, c_onoff)} {
            ffiairspy::AIRSPY_SUCCESS => Ok(()),
            ffiairspy::AIRSPY_ERROR_LIBUSB => Err(AirspyError::LibUsb),
            err => {
                panic!("airspy_set_rf_bias returned error {}", err)
            }
        }
    }
}

impl Drop for Airspy {
    fn drop(&mut self) {
        unsafe{ffiairspy::airspy_close(self.device)};
        unsafe{ffiairspy::airspy_exit()};
    }
}
