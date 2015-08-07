
extern crate airspy;
use airspy::Airspy;
use airspy::AirspySamplerate;

#[macro_use]
extern crate log;
extern crate fern;
extern crate time;
extern crate num;
use num::complex::Complex;

use std::io;
use std::io::Write;
use std::env;

fn main() {
    // setup fern logger
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            let t = time::now();
            let ms = t.tm_nsec/1000_000;
            format!("{}.{:3} [{}] {}", t.strftime("%Y-%m-%dT%H:%M:%S").unwrap(), ms, level, msg)
        }),
        output: vec![fern::OutputConfig::stderr()],
        level: log::LogLevelFilter::Trace,
    };

    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}", e);
    }

    // check args
    let mut args = env::args();
    let mut frequency_hz = 103_400_000;
    if args.len() == 1 {
        info!("use default frequency: {:.3} MHz", frequency_hz as f64/1000_000f64);
    }
    else {
        frequency_hz = (args.nth(1).unwrap().parse::<f64>().unwrap() * 1000_000f64) as u32;
        info!("frequency: {:.3} MHz", frequency_hz as f64/1000_000f64);
    }

    match Airspy::<Complex<i16>>::new() {
        Ok(mut device) => {
            info!("device opened sucessfully");
            device.set_samplerate(AirspySamplerate::Sps2500000).unwrap();

            device.set_vga_gain(6).unwrap();
            device.set_mixer_gain(5).unwrap();
            device.set_lna_gain(7).unwrap();

            device.set_frequency(frequency_hz).unwrap();
            device.start_rx().unwrap();

            let mut stdout = io::stdout();

            loop {
                // actually iterator blocks here
                for s in device.samples() {
                    let sliceu8 = Airspy::convert_samples_to_bytes(&s);

                    stdout.write(sliceu8).unwrap();
                    stdout.flush().unwrap();
                }

                info!("sleep");
            }
        }
        Err(e) => {
            error!("error {:?}", e);
        }
    }
}
