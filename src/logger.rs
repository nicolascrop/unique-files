use log::{Record, Level, Metadata, LevelFilter};

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{:?} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub enum Type {
    CONSOLE
}

pub fn init(logger_type: Type) {
    if logger_type == Type::CONSOLE {
        static LOGGER: ConsoleLogger = ConsoleLogger;
        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
    }
}