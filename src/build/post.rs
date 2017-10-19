use chrono::NaiveDate;
use pulldown_cmark::{html, Parser};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct Post<'a> {
    pub path: &'a Path,
    pub title: &'a str,
    pub ymd: &'a str,
    pub date: NaiveDate,
    pub slug: String,
}

impl<'a> Post<'a> {
    pub fn new<P, F>(path: &'a P, get_slug: &mut F) -> Option<Self>
    where
        P: AsRef<Path>,
        F: FnMut(&'a str) -> String,
    {
        let path = path.as_ref();
        let stem = path.file_stem();

        let stem = match stem.and_then(|stem| stem.to_str()) {
            Some(stem) => stem,
            None => return None,
        };

        if stem.len() <= 10 {
            return None;
        }
        
        let ymd = &stem[..10];
        let title = stem[10..].trim();

        if title.len() == 0 {
            return None;
        }

        let date = match NaiveDate::parse_from_str(ymd, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return None,
        };

        let slug = get_slug(title);

        Some(Self { path, title, ymd, date, slug })
    }

    pub fn content(&self) -> io::Result<String> {
        let mut md = String::new();
        let mut html = String::new();
        File::open(&self.path)?.read_to_string(&mut md)?;
        html::push_html(&mut html, Parser::new(&md));
        Ok(html)
    }
}
