use std::{fs, io::Write};
use std::net::SocketAddr;
use chrono::{Datelike, Local, Timelike};

pub struct Logger {
    file: fs::File,
}

impl Logger {
    pub fn create() -> anyhow::Result<Self> {
        let now = Local::now();
        if !std::path::Path::new("logs").exists() {
            std::fs::create_dir("logs")?;
        }
        let log_name = format!(
            "logs/log-{:04}-{:02}-{:02}-{:02}-{:02}-{:02}.txt",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
        );
        let file = fs::File::create(log_name)?;
        let mut logger = Self { file };
        Ok(logger)
    }

    pub fn info(&mut self, message: &str) -> anyhow::Result<()> {
        let now = Local::now();
        writeln!(
            self.file,
            "[{:04}-{:02}-{:02}-{:02}-{:02}-{:02}] INFO > {:?}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
            message
        )?;
        Ok(())
    }

    pub fn error(&mut self, message: &str) -> anyhow::Result<()> {
        let now = Local::now();
        writeln!(
            self.file,
            "[{:04}-{:02}-{:02}-{:02}-{:02}-{:02}] ERROR > {:?}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
            message
        )?;
        Ok(())
    }
}