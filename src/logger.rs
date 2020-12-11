use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};
use colored::Colorize;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            print!("{} - ", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            let str_log = format!("{:?} - {}", record.level(), record.args());
            let str_log_colored = match record.level() {
                Level::Debug => str_log.cyan(),
                Level::Info => str_log.magenta(),
                Level::Trace => str_log.blue(),
                Level::Warn => str_log.yellow(),
                Level::Error => str_log.red()
            };
            println!("{}", str_log_colored);
        }
    }

    fn flush(&self) {}
}

pub enum Type {
    CONSOLE
}

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

pub fn init(logger_type: Type, level: LevelFilter) -> Result<(), SetLoggerError> {
    match logger_type {
        Type::CONSOLE => log::set_logger(&CONSOLE_LOGGER).map(|()| log::set_max_level(level))
    }
}