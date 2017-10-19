use chrono::NaiveDate;
use error::{Error, Result};
use handlebars::Handlebars;
use glob::glob;
use pulldown_cmark::{html, Parser};
use slug;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

//TODO: Better errors.
pub fn build() -> Result {
    let mut handlebars = Handlebars::new();

    //TODO: Add strftime helper.

    // Load templates.

    for entry in glob("templates/*.hbs")? {
        let path = entry?;

        let stem = path.file_stem().unwrap();
        let stem = match stem.to_str() {
            Some(stem) => stem,
            None => {
                //TODO: Warn of ignored template.
                continue;
            },
        };

        handlebars.register_template_file(stem, &path)?;
    }

    // Build the list of paths.
    
    let mut paths = Vec::new();

    for entry in glob("posts/*.md")? {
        paths.push(entry?);
    }

    // Sort from oldest to newest.

    paths.sort_unstable_by(|a, b| {
        a.file_stem().cmp(&b.file_stem())
    });

    // Calculate slugs.

    let mut used = HashSet::new();
    used.insert("index".to_owned());
    used.insert("archive".to_owned());

    let mut get_slug = move |title| {
        let mut slug = slug::slugify(title);

        if used.contains(&slug) {
            for i in 2.. {
                let next = format!("{}-{}", slug, i);
                if !used.contains(&next) {
                    slug = next;
                    break;
                }
            }
        }

        used.insert(slug.clone());
        slug
    };

    // Build the list of posts.

    let posts = paths.iter().filter_map(|path| {
        if let Some(post) = Post::new(path, &mut get_slug) {
            Some(post)
        } else {
            //TODO: Warn of invalid posts.
            None
        }
    });

    // Create output folder if it doesn't exist.

    if !Path::new("out").is_dir() {
        fs::create_dir("out")?;
    }

    // Process each post.
    
    let mut archive_data = Vec::new();

    const POSTS: usize = 8;
    let mut ohgodwhat = [None, None, None, None, None, None, None, None];
    let mut noooooooo = POSTS - 1;

    for c in posts {
        if let Some(post) = ohgodwhat[noooooooo].as_ref() {
            render_post(
                &handlebars,
                post,
                ohgodwhat[(noooooooo + (POSTS - 1)) % POSTS].as_ref(),
                Some(&c)
            )?;

            archive_data.push(json!({
                "title": post.title,
                "date":  post.ymd,
                "slug":  post.slug,
            }));
        }

        noooooooo = (noooooooo + 1) % POSTS;
        ohgodwhat[noooooooo] = Some(c);
    }

    if let Some(post) = ohgodwhat[noooooooo].as_ref() {
        render_post(
            &handlebars,
            post,
            ohgodwhat[(POSTS - 1 + noooooooo) % POSTS].as_ref(),
            None,
        )?;

        archive_data.push(json!({
            "title": post.title,
            "date":  post.ymd,
            "slug":  post.slug,
        }));
    }

    archive_data.reverse();

    //TODO: Warn if no posts were found.

    // Compile archives.

    handlebars.renderw(
        "archive",
        &json!({ "posts": archive_data }),
        &mut File::create("out/archive.html")?
    )?;

    // Compile index.

    let mut index_data = Vec::new();

    let goal = (noooooooo + 1) % POSTS;
    while noooooooo != goal {
        if let Some(post) = ohgodwhat[noooooooo].as_ref() {
            index_data.push(json!({
                "title":   post.title,
                "date":    post.ymd,
                "slug":    post.slug,
                "content": post.content()?,
            }));
        } else {
            break;
        }

        noooooooo = (POSTS - 1 + noooooooo) % POSTS;
    }

    handlebars.renderw(
        "index",
        &json!({ "posts": index_data }),
        &mut File::create("out/index.html")?
    )?;

    // Done! How on Earth did you read all of this?!
    
    Ok(())
}

fn render_post(
    handlebars: &Handlebars,
    post: &Post,
    prev: Option<&Post>,
    next: Option<&Post>,
) -> Result {
    // Construct the JSON data.

    let data = json!({
        "prev": prev.map(|prev| {
            json!({
                "title": prev.title,
                "date":  prev.ymd,
                "slug":  prev.slug,
            })
        }),

        "next": next.map(|next| {
            json!({
                "title": next.title,
                "date":  next.ymd,
                "slug":  next.slug,
            })
        }),

        "title": post.title,
        "date":  post.ymd,
        "slug":  post.slug,

        "content": post.content()?,
    });

    // Open the file.
    
    let mut file = File::create(format!("out/{}.html", post.slug))?;

    // Render.

    handlebars.renderw("post", &data, &mut file)?;

    Ok(())
}

struct Post<'a> {
    path: &'a Path,
    title: &'a str,
    ymd: &'a str,
    date: NaiveDate,
    slug: String,
}

impl<'a> Post<'a> {
    fn new<P, F>(path: &'a P, get_slug: &mut F) -> Option<Self>
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

    fn content(&self) -> io::Result<String> {
        let mut md = String::new();
        let mut html = String::new();
        File::open(&self.path)?.read_to_string(&mut md)?;
        html::push_html(&mut html, Parser::new(&md));
        Ok(html)
    }
}
