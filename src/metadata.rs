use chrono::{DateTime, Local};
use toml;

#[derive(Deserialize, Default)]
pub struct Metadata {
    pub title: Option<String>,
    pub date: Option<DateTime<Local>>,
    // pub publish: Option<bool>, // not used yet
    // pub feed: Option<bool>, // not used yet
}

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
    info!("md: {}", md_str);
    match toml::from_str::<Metadata>(&md_str) {
        Ok(md2) => md = md2,
        Err(e) => error!("Error parsing metadata: {:?}", e),
    }
    return (md, v[2])
}
