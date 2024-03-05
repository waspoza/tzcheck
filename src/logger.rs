use chrono::Local;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::io::{self, Write};

#[allow(dead_code)]
pub enum Output {
    STDOUT,
    STDERR,
    File(&'static str),
}

pub struct Logger {
    output: Output,
    level: Level,
}

impl Logger {

    fn timestamp() -> String {
        let date = Local::now();
        format!("{}", date.format("%Y-%m-%d %H:%M:%S"))
    }

    pub fn set(output: Output, level: Level) -> Result<(), SetLoggerError> {
        let logger = Logger { output, level };
        log::set_max_level(LevelFilter::max());
        log::set_boxed_logger(Box::new(logger))
    }

}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!("{} {}: {}\n", Logger::timestamp(), record.level(), record.args());
            let mut out_writer = match self.output {
            Output::File(name) => {
                Box::new(std::fs::OpenOptions::new().write(true).append(true).create(true).open(name).unwrap()) as Box<dyn Write>
            }
            Output::STDOUT => Box::new(io::stdout()) as Box<dyn Write>,
            Output::STDERR => Box::new(io::stderr()) as Box<dyn Write>,
};
         let _ = out_writer.write(message.as_bytes());   
        }
    }
    fn flush(&self) {}
}

