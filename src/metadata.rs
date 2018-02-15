use chrono::{DateTime, Local};
use toml;

#[derive(Deserialize, Default)]
pub struct Metadata {
    pub title: Option<String>,
    pub date: Option<DateTime<Local>>,
    // pub publish: Option<bool>, // not used yet
    // pub feed: Option<bool>, // not used yet
}

pub fn parse_metadata(src:String) -> (Metadata, String) {
    let mut md_found = false;
    let mut md_done = false;
    let mut md = Metadata::default();
    let mut md_lines = vec![];
    let mut no_md_lines = vec![];
    for line in src.lines() {
        if md_done {
            no_md_lines.push(line);
            continue;
        }
        if !md_found {
            if line.starts_with("+++") {
                md_found = true;
            } else {
                md_done = true;
            }
        } else {
            if line.starts_with("+++") {
                md_done = true;
            } else {
                md_lines.push(line)
            }
        }
    }
    if md_found {
        let md_string = md_lines.join("\n");
        info!("md: {}", md_string);
        match toml::from_str::<Metadata>(&md_string) {
            Ok(md2) => md = md2,
            Err(e) => error!("Error parsing metadata: {:?}", e),
        }
    }
    let remainder = no_md_lines.join("\n");
    (md, remainder)
}
