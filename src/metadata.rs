use chrono::{DateTime, Local};
use toml;

#[derive(Default)]
pub struct Metadata(toml::value::Table);

pub fn parse_metadata(src:&str) -> (Metadata, &str) {
    let mut md = Metadata::default();
    if !src.starts_with("+++") {
        return (md, src)
    }
    let v:Vec<&str> = src.splitn(3, "+++").collect();
    if v.len() != 3 {
        return (md, src)
    }
    let md_str = v[1];
    match md_str.parse::<toml::Value>() {
        Ok(v) => {
            match v {
                toml::Value::Table(t) => {
                    md = Metadata(t)
                }
                x => error!("Unexpected Toml::Value: {:?}", x),
            }
        }
        Err(e) => error!("Error parsing metadata: {:?}", e),
    }
    return (md, v[2])
}

impl Metadata {
    pub fn get_string(&self, name:&str) -> Option<String> {
        match self.0.get(name) {
            None => None,
            Some(v) => match *v {
                toml::Value::String(ref s) => Some(s.clone()),
                _ => None,
            }
        }
    }

    pub fn get_date(&self, name:&str) -> Option<DateTime<Local>> {
        if let Some(s) = self.get_string(name) {
            match s.parse::<DateTime<Local>>() {
                Ok(d) => Some(d),
                Err(e) => {
                    error!("Error parsing date {}: {:?}", s, e);
                    None
                }
            }
        } else {
            None
        }
    }
}
