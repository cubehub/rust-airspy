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
use libc::{c_int, c_void};

#[allow(non_camel_case_types)]
pub mod ffiairspy;

use std::mem;
use std::ptr;
use std::slice;

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IQ<T> {
    i: T,
    q: T,
}

pub trait AirspySupportedType<T> {
    fn get_sample_type() -> u32;
    fn rawbuf_to_vec(rawbuf: *mut c_void, len: usize) -> Vec<T>;
}

macro_rules! impl_airspy_supported_type {
    ($typ:ty, $sample_type:expr) => {
        impl AirspySupportedType<$typ> for $typ {
            fn get_sample_type() -> u32 {
                $sample_type
            }

            fn rawbuf_to_vec(rawbuf: *mut c_void, len: usize) -> Vec<$typ> {
                let slice: &[$typ] = unsafe {
                        slice::from_raw_parts(rawbuf as *mut $typ, len)
                };
                slice.to_vec()
            }
        }
    };
}

impl_airspy_supported_type!(IQ<f32>, ffiairspy::AIRSPY_SAMPLE_FLOAT32_IQ);
impl_airspy_supported_type!(f32,     ffiairspy::AIRSPY_SAMPLE_FLOAT32_REAL);
impl_airspy_supported_type!(IQ<i16>, ffiairspy::AIRSPY_SAMPLE_INT16_IQ);
impl_airspy_supported_type!(i16,     ffiairspy::AIRSPY_SAMPLE_INT16_REAL);
impl_airspy_supported_type!(u16,     ffiairspy::AIRSPY_SAMPLE_UINT16_REAL);

pub struct Airspy<T: AirspySupportedType<T>> {
    device: *mut ffiairspy::Struct_airspy_device,
    samples: Vec<T>,
}

impl<T: AirspySupportedType<T>> Airspy<T> {

    fn context(&mut self) -> *mut c_void {
        self as *mut _ as *mut c_void
    }

    pub fn new() -> Result<Airspy<T>, AirspyError> {
        let mut device = ptr::null_mut();

        let error = unsafe {ffiairspy::airspy_open(&mut device)};
        match error {
            ffiairspy::AIRSPY_SUCCESS => {
                let airspy = Airspy{device: device, samples: Vec::<T>::new()};

                match unsafe {ffiairspy::airspy_set_sample_type(device, T::get_sample_type())} {
                    ffiairspy::AIRSPY_SUCCESS => {
                        Ok(airspy)
                    },
                    err => {
                        // airspy_set_sample_type should only return SUCCESS
                        // however just in case panic here if for some reason it does anything else
                        panic!("airspy_set_sample_type returned error {}", err)
                    },
                }
            },
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

    pub fn set_lna_gain(&self, gain: u8) -> Result<(), AirspyError> {
        if gain > 15 {
            Err(AirspyError::InvalidParam)
        }
        else {
            match unsafe {ffiairspy::airspy_set_lna_gain(self.device, gain)} {
                ffiairspy::AIRSPY_SUCCESS => Ok(()),
                ffiairspy::AIRSPY_ERROR_LIBUSB => Err(AirspyError::LibUsb),
                err => panic!("airspy_set_lna_gain returned error {}", err),
            }
        }
    }

    pub fn set_mixer_gain(&self, gain: u8) -> Result<(), AirspyError> {
        if gain > 15 {
            Err(AirspyError::InvalidParam)
        }
        else {
            match unsafe {ffiairspy::airspy_set_mixer_gain(self.device, gain)} {
                ffiairspy::AIRSPY_SUCCESS => Ok(()),
                ffiairspy::AIRSPY_ERROR_LIBUSB => Err(AirspyError::LibUsb),
                err => panic!("airspy_set_mixer_gain returned error {}", err)
            }
        }
    }

    pub fn set_vga_gain(&self, gain: u8) -> Result<(), AirspyError> {
        if gain > 15 {
            Err(AirspyError::InvalidParam)
        }
        else {
            match unsafe {ffiairspy::airspy_set_vga_gain(self.device, gain)} {
                ffiairspy::AIRSPY_SUCCESS => Ok(()),
                ffiairspy::AIRSPY_ERROR_LIBUSB => Err(AirspyError::LibUsb),
                err => panic!("airspy_set_vga_gain returned error {}", err),
            }
        }
    }

    pub fn set_frequency(&self, freq_hz: u32) -> Result<(), AirspyError> {
        // 24 MHz to 1.8 GHz
        if freq_hz < 24_000_000 || freq_hz > 1_8_00_000_000 {
            Err(AirspyError::InvalidParam)
        }
        else {
            match unsafe {ffiairspy::airspy_set_freq(self.device, freq_hz)} {
                ffiairspy::AIRSPY_SUCCESS => Ok(()),
                ffiairspy::AIRSPY_ERROR_LIBUSB => Err(AirspyError::LibUsb),
                err => panic!("airspy_set_freq returned error {}", err),
            }
        }
    }

    pub fn start_rx(&mut self) -> Result<(), AirspyError> {
        match unsafe {ffiairspy::airspy_start_rx(self.device, Some(Self::receive), self.context())} {
            ffiairspy::AIRSPY_SUCCESS => Ok(()),
            ffiairspy::AIRSPY_ERROR_BUSY => Err(AirspyError::Busy),
            ffiairspy::AIRSPY_ERROR_THREAD => Err(AirspyError::Thread),
            err => panic!("airspy_start_rx returned error {}", err),
        }
    }

    extern "C" fn receive(transfer: *mut ffiairspy::airspy_transfer) -> c_int {
        let transfer: &mut ffiairspy::airspy_transfer = unsafe {mem::transmute(transfer)};
        assert!(!transfer.ctx.is_null());
        info!("samplecount: {}", transfer.sample_count);
        assert!(transfer.sample_count == 131072 || transfer.sample_count == 65536);

        let selfclient : &mut Self = unsafe {mem::transmute(transfer.ctx)};
        selfclient.samples = T::rawbuf_to_vec(transfer.samples, transfer.sample_count as usize);

        0
    }
}

impl<T: AirspySupportedType<T>> Drop for Airspy<T> {
    fn drop(&mut self) {
        unsafe{ffiairspy::airspy_close(self.device)};
        unsafe{ffiairspy::airspy_exit()};
    }
}
