
extern crate airspy;
use airspy::AirspySamplerate;

#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

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

    match airspy::Airspy::new() {
        Ok(mut device) => {
            info!("device opened sucessfully");
            device.set_samplerate(AirspySamplerate::Sps10000000).unwrap();
        }
        Err(e) => {
            error!("error {:?}", e);
        }
    }
}
