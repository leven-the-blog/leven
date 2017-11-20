use chrono::{DateTime, Local};
use error::{Error, Result};
use pulldown_cmark;
use pulldown_cmark::Parser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use rayon::scope;
use slug;
use std::{fs, io};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::channel;
use tenjin::Tenjin;
use toml;
use util::{InjectDate, cd2root, load_config, build_tenjin, cpr};

const DEFAULT_RECENTS: usize = 5;
const DEFAULT_DATE_FORMAT: &'static str = "%d %B, %Y";

struct Post {
    content: Html,
    title: String,
    date: DateTime<Local>,
    slug: String,
}

struct ListContext<'a> {
    config: &'a toml::Value,
    date_format: &'a str,
    posts: &'a [Post],
}

struct PostContext<'a> {
    config: &'a toml::Value,
    date_format: &'a str,
    post: &'a Post,
    prev: Option<&'a Post>,
    next: Option<&'a Post>,
}

struct PostWrap<'a> {
    post: &'a Post,
    date_format: &'a str,
}

type Html = String;

context! {
    self: ('a) PostWrap<'a> {
        content => @raw self.post.content.as_str(),
        title => self.post.title.as_str(),
        date => InjectDate {
            date: self.post.date,
            format: self.date_format,
        },
        slug => self.post.slug.as_str(),
    }

    self: ('a) ListContext<'a> {
        config => self.config,
        posts => @iter
            self.posts
                .into_iter()
                .map(|post| post.wrap(self.date_format)),
    }

    self: ('a) PostContext<'a> {
        config => self.config,
        post => self.post.wrap(self.date_format),
        prev => self.prev.map(|post| post.wrap(self.date_format)),
        next => self.next.map(|post| post.wrap(self.date_format)),
    }
}

impl Post {
    fn new<P: AsRef<Path>>(path: P) -> Result<Post> {
        let path = path.as_ref();

        let mut src = String::new();
        File::open(&path)?.read_to_string(&mut src)?;

        let mut content = String::new();
        let parser = Parser::new(&src);
        pulldown_cmark::html::push_html(&mut content, parser);

        let title = path.file_stem()
            .unwrap()
            .to_string_lossy()
            .into();

        let date  = path.metadata()?.modified()?.into();
        let slug  = slug::slugify(&title);

        Ok(Post { content, title, date, slug })
    }

    fn wrap<'a>(&'a self, date_format: &'a str) -> PostWrap<'a> {
        PostWrap {
            post: self,
            date_format,
        }
    }
}

fn build() -> Result<()> {
    let out = Path::new("out");

    cd2root()?;

    let config = load_config()?;
    let tenjin = build_tenjin()?;

    if out.is_dir() {
        for child in fs::read_dir(out)? {
            let child = child?;
            if child.file_type()?.is_dir() {
                fs::remove_dir_all(&child.path())?;
            } else {
                fs::remove_file(&child.path())?;
            }
        }
    } else if out.is_file() {
        fs::remove_file(out)?;
        fs::create_dir(out)?;
    } else {
        fs::create_dir(out)?;
    }

    if let Err(e) = cpr("theme/assets", "out/assets") {
        error!("Failed to copy theme assets: {}.", e);
    }

    let mut posts: Vec<Post> = Vec::new();

    scope(|s| {
        let (tx, rx) = channel();

        let entries = match fs::read_dir("content") {
            Ok(entries) => entries,
            Err(e) => {
                warn!("Could not read content directory: {}.", e);
                return;
            },
        };

        for entry in entries.flat_map(|x| x.ok()) {
            let path = entry.path();

            if path.is_file() && path.extension() == Some("md".as_ref()) {
                let tx = tx.clone();
                s.spawn(move |_| {
                    match Post::new(&path) {
                        Ok(post) => tx.send(post).unwrap(),
                        Err(e) => {
                            error!("Could not read post `{}`: {}.", path.display(), e);
                            return;
                        }
                    }
                });
            } else {
                let dst = out.join(path.file_name().unwrap());

                if let Err(e) = cpr(&path, dst) {
                    error!("Failed to copy `{}`: {}.)", path.display(), e);
                }
            }
        }

        drop(tx);
        while let Ok(post) = rx.recv() {
            posts.push(post);
        }
    });

    posts.par_sort_unstable_by(|a, b| b.date.cmp(&a.date));

    let date_format = match config.get("date-format") {
        Some(&toml::Value::String(ref s)) => s.as_str(),
        None => DEFAULT_DATE_FORMAT,
        Some(x) => {
            warn!("Expected string for `date-format`, found {}.", x);
            DEFAULT_DATE_FORMAT
        },
    };

    // Build `index.html`.
    
    if let Err(e) = build_index(&config, date_format, &tenjin, &posts) {
        error!("Failed to build `index.html`: {}.", e);
    }

    // Build `archive.html`.

    if let Err(e) = build_archive(&config, date_format, &tenjin, &posts) {
        error!("Failed to build `archive.html`: {}.", e);
    }
    
    // Build posts.

    if let Err(e) = build_posts(&config, date_format, &tenjin, &posts) {
        error!("Failed to build posts: {}.", e);
    }
    
    Ok(())
}

fn build_posts(
    config: &toml::Value,
    date_format: &str,
    tenjin: &Tenjin,
    posts: &[Post]
) -> Result<()> {
    let template = match tenjin.get("post") {
        Some(template) => template,
        None => return Err(Error::TemplateNotFound("post".into())),
    };

    (0..posts.len()).into_par_iter().for_each(|i| {
        let next = if i > 0 { Some(&posts[i - 1]) } else { None };
        let prev = posts.get(i + 1);
        let post = &posts[i];

        let ctx = PostContext {
            config,
            date_format,
            prev,
            next,
            post,
        };

        let path = format!("out/{}.html", post.slug);

        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(e) => {
                error!("Could not open `{}`: {}.", path, e);

                if e.kind() == io::ErrorKind::NotFound {
                    info!("Maybe you should try a more unique title?");
                }

                return;
            },
        };

        if let Err(e) = tenjin.render(template, &ctx, &mut file) {
            error!("Could not render `{}`: {}.", post.title, e);
        }
    });

    Ok(())
}

//TODO: Better errors everywhere.
//TODO: Cleanup build code.

fn build_archive(
    config: &toml::Value,
    date_format: &str,
    tenjin: &Tenjin,
    posts: &[Post]
) -> Result<()> {
    let ctx = ListContext {
        config,
        date_format,
        posts,
    };

    let template = match tenjin.get("archive") {
        Some(template) => template,
        None => return Err(Error::TemplateNotFound("archive".into())),
    };

    let mut file = File::create("out/archive.html")?;

    tenjin.render(template, &ctx, &mut file)?;

    Ok(())
}

fn build_index(
    config: &toml::Value,
    date_format: &str,
    tenjin: &Tenjin,
    posts: &[Post]
) -> Result<()> {
    let recents = match config.get("recents") {
        Some(&toml::Value::Integer(n)) if n > 0 => n as usize,
        None => DEFAULT_RECENTS,
        Some(x) => {
            warn!("Expected natural number for `recents`, found {}.", x);
            DEFAULT_RECENTS
        },
    };

    let ctx = ListContext {
        config,
        date_format,
        posts: if recents < posts.len() { &posts[..recents] } else { posts }
    };
    
    let template = match tenjin.get("index") {
        Some(template) => template,
        None => return Err(Error::TemplateNotFound("index".into())),
    };

    let mut file = File::create("out/index.html")?;

    tenjin.render(template, &ctx, &mut file)?;

    Ok(())
}

pub fn execute() {
    match build() {
        Ok(()) => info!("Build complete!"),
        Err(e) => error!("Build failed: {}.", e),
    }
}
