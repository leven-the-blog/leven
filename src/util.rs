use chrono::{DateTime, Utc};
use error::{Result, Error};
use std::{env, fs, io};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;
use tenjin;
use tenjin::{Tenjin, Context};
use toml;

pub struct InjectDate<'a> {
    pub format: &'a str,
    pub date: DateTime<Utc>,
}

impl<'a, W: Write> Context<W> for InjectDate<'a> {
    fn truthy(&self, path: tenjin::path::Path) -> bool {
        let mut parts = path.parts();
        match parts.next() {
            None => true,
            Some("iso8601") => parts.next().is_none(),
            _ => false
        }
    }

    fn inject(&self, path: tenjin::path::Path, sink: &mut W) -> tenjin::Result<()> {
        let mut parts = path.parts();

        match parts.next() {
            None => {
                //TODO: HTML INJECTION VECTOR.
                write!(sink, "{}", self.date.format(self.format))?;
                return Ok(());
            }

            Some("iso8601") => {
                if parts.next().is_none() {
                    write!(sink, "{}", self.date.format("%Y-%m-%dT%H:%M:%S%.f%:z"))?;
                    return Ok(());
                }
            }

            _ => {}
        }

        Err(tenjin::Error::Undefined(path.to_owned()))
    }

    fn iterate(&self, path: tenjin::path::Path, _: tenjin::render::Chomp<W>) -> tenjin::Result<()> {
        let mut parts = path.parts();

        match parts.next() {
            None => {
                return Err(tenjin::Error::NotIterable(path.to_owned()));
            }

            Some("iso8601") => {
                if parts.next().is_none() {
                    return Err(tenjin::Error::NotIterable(path.to_owned()));
                }
            }

            _ => {}
        }

        Err(tenjin::Error::Undefined(path.to_owned()))
    }
}

pub fn load_config() -> Result<toml::Value> {
    let mut config = String::new();
    File::open("Leven.toml")?
        .read_to_string(&mut config)?;
    Ok(toml::Value::from_str(&config)?)
}

pub fn build_tenjin() -> Result<Tenjin> {
    let mut path = "theme/templates".into();
    Ok(Tenjin::new(&mut path)?)
}

pub fn cd2root() -> Result<()> {
    let mut path = env::current_dir()?;

    loop {
        path.push("Leven.toml");

        if path.is_file() {
            path.pop();
            env::set_current_dir(&path)?;
            return Ok(());
        }

        path.pop();

        if !path.pop() {
            return Err(Error::ConfigNotFound);
        }
    }
}

pub fn cpr<A: AsRef<Path>, B: AsRef<Path>>(from: A, to: B) -> io::Result<()> {
    let (from, to) = (from.as_ref(), to.as_ref());

    if from.is_file() {
        fs::copy(from, to)?;
    } else {
        fs::create_dir(to)?;

        for entry in fs::read_dir(from)?.flat_map(io::Result::ok) {
            let from = entry.path();
            let to = to.join(from.file_name().unwrap_or(OsStr::new("")));

            if let Err(e) = cpr(&from, &to) {
                error!("failed to copy `{}` ({})", from.display(), e);
            }
        }
    }

    Ok(())
}
