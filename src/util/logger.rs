use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

use colored::Colorize;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogContent {
    pub time: DateTime<Local>,
    pub text: String,
}

impl LogContent {
    pub fn to_colored_string(&self) -> String {
        let time = self.time.format("[%x %X:%3f]").to_string().bright_black();
        let text = self.text.trim().bright_white();

        format!("{time} {text}")
    }
}

impl Display for LogContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = self.time.format("[%x %X:%3f]").to_string();
        let text = self.text.trim();

        writeln!(f, "{time} {text}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrContent {
    pub time: DateTime<Local>,
    pub text: String,
    pub long: Option<String>,
}

impl ErrContent {
    pub fn to_colored_string(&self) -> String {
        let kind = "[!]".bright_red();
        let time = self.time.format("[%x %X:%3f]").to_string().bright_black();
        let text = self.text.trim().bright_white();

        format!("{time} {kind} {text}")
    }
    pub fn to_long_string(&self) -> String {
        self.long
            .as_deref()
            .unwrap_or(&self.text)
            .trim()
            .to_string()
    }
}

impl Display for ErrContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const KIND: &str = "[!]";
        let time = self.time.format("[%x %X:%3f]").to_string();
        let text = self.text.trim();

        writeln!(f, "{time} {KIND} {text}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Logger {
    path: PathBuf,
    quiet: bool,
    ephemeral: bool,
}

impl Logger {
    pub const ERR_DIR: &str = "err";
    pub const LOG_DIR: &str = "log";

    pub fn new(quiet: bool, ephemeral: bool) -> Result<Self> {
        let time = Local::now();
        let path = PathBuf::from_str(&time.format("%y%m%d%H%M%S%f.txt").to_string())?;

        if !ephemeral {
            create_dir_all(Self::ERR_DIR)?;
            create_dir_all(Self::LOG_DIR)?;

            File::create(PathBuf::from_str(Self::LOG_DIR)?.join(&path))?;
        }

        Ok(Self {
            path,
            quiet,
            ephemeral,
        })
    }

    pub fn error_code(time: DateTime<Local>) -> String {
        time.format("%y%m%d%H%M%S%f").to_string()
    }

    pub fn path(&self) -> Result<PathBuf> {
        Ok(PathBuf::from_str(Self::LOG_DIR)?.join(&self.path))
    }

    fn __err(&self, time: DateTime<Local>, text: String, long: Option<String>) -> Result<String> {
        let file = Self::error_code(time);
        let path = PathBuf::from_str(Self::ERR_DIR)?
            .join(&file)
            .with_extension("txt");

        let err = ErrContent { time, text, long };

        if !self.quiet {
            eprintln!("{}", err.to_colored_string());
        }
        if !self.ephemeral {
            let mut file = File::create(path)?;

            file.write_all(err.to_long_string().as_bytes())?;
            file.flush()?;

            let mut file = File::options().append(true).open(self.path()?)?;

            file.write_all(err.to_string().as_bytes())?;
            file.flush()?;
        }

        Ok(file)
    }
    #[inline]
    fn __log(&self, text: String) -> Result<()> {
        let time = Local::now();
        let log = LogContent { time, text };

        if !self.quiet {
            println!("{}", log.to_colored_string());
        }
        if !self.ephemeral {
            let mut file = File::options().append(true).open(self.path()?)?;

            file.write_all(log.to_string().as_bytes())?;
            file.flush()?;
        }

        Ok(())
    }

    #[inline]
    pub fn info<T>(&self, text: T) -> Result<()>
    where
        T: TryInto<String>,
        <T as TryInto<String>>::Error: Error + Send + Sync + 'static,
    {
        self.__log(text.try_into()?)
    }
    #[inline]
    pub fn error<T>(&self, time: DateTime<Local>, text: T) -> Result<String>
    where
        T: TryInto<String>,
        <T as TryInto<String>>::Error: Error + Send + Sync + 'static,
    {
        self.__err(time, text.try_into()?, None)
    }
    #[inline]
    pub fn error_long<T>(
        &self,
        time: DateTime<Local>,
        text: T,
        long: &impl ToString,
    ) -> Result<String>
    where
        T: TryInto<String>,
        <T as TryInto<String>>::Error: Error + Send + Sync + 'static,
    {
        self.__err(time, text.try_into()?, Some(long.to_string()))
    }
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $($arg:tt)+) => {
		$logger.info(format_args!($($arg)+).to_string()).ok();
	};
}

#[macro_export]
macro_rules! error {
	($logger:expr, $time:expr, $($arg:tt)+) => {
		$logger.error($time, format_args!($($arg)+).to_string()).ok();
	};
}

#[macro_export]
macro_rules! error_long {
	($logger:expr, $time:expr, $long:expr, $($arg:tt)+) => {
		$logger.error_long($time, format_args!($($arg)+).to_string(), &$long).ok();
	};
}
