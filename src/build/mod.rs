use error::{Result, ResultExt};
use handlebars::Handlebars;
use glob::glob;
use self::post::Post;
use slug;
use std::collections::HashSet;
use std::fs::{self, File};
use std::path::Path;

mod post;

//TODO: Clean this mess up.
//TODO: Parallelization.
//TODO: Incremental compilation.
pub fn build() -> Result<()> {
    let mut handlebars = Handlebars::new();

    // Load templates.

    let templates = glob("templates/*.hbs")
        .chain_err(|| "couldn't read templates directory")?;

    for entry in templates {
        let path = entry
            .chain_err(|| "couldn't read template")?;

        let stem = path.file_stem().unwrap().to_string_lossy();

        handlebars.register_template_file(&stem, &path)
            .chain_err(|| "couldn't register template")?;
    }

    // Build the list of paths.
    
    let mut paths = Vec::new();

    let entries = glob("posts/*.md")
        .chain_err(|| "couldn't read posts directory")?;

    for entry in entries {
        paths.push(entry.chain_err(|| "couldn't read post")?);
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

    let posts: Vec<_> = paths.iter()
        .filter_map(|path| {
            if let Some(post) = Post::new(path, &mut get_slug) {
                Some(post)
            } else {
                warn!(
                    "Ignoring '{}' because it has an invalid filename.",
                    path.display()
                );
                None
            }
        })
        .collect();

    // Create output folder if it doesn't exist.

    if !Path::new("out").is_dir() {
        fs::create_dir("out")
            .chain_err(|| "couldn't create out directory")?;
    }

    // Render each post.

    let mut a = None;
    let mut b = None;

    for c in posts.iter() {
        if let Some(post) = b {
            render_post(&handlebars, post, a, Some(c))?;
        }

        a = b;
        b = Some(c);
    }

    if let Some(post) = b {
        render_post(&handlebars, post, a, None)?;
    }

    // Compile archives.
    
    let archive_data = posts.iter()
        .map(|post| json!({
            "title": post.title,
            "date": post.ymd,
            "slug": post.slug,
        }))
        .collect::<Vec<_>>();

    handlebars.renderw(
        "archive",
        &json!({ "posts": archive_data }),
        &mut File::create("out/archive.html")
            .chain_err(|| "couldn't create archive")?
    ).chain_err(|| "couldn't render archive")?;

    // Compile index.

    //TODO: Configurable index size.
    const POSTS: usize = 16;

    let mut index_data = Vec::with_capacity(POSTS);

    for post in posts.iter().rev().take(POSTS) {
        let content: String = post.content()
            .chain_err(|| format!("couldn't process '{}'", post.title))?;

        index_data.push(json!({
            "title":   post.title,
            "date":    post.ymd,
            "slug":    post.slug,
            "content": content
        }))
    }

    handlebars.renderw(
        "index",
        &json!({ "posts": index_data }),
        &mut File::create("out/index.html")
            .chain_err(|| "couldn't create index")?
    ).chain_err(|| "couldn't render index")?;

    // Done! How on Earth did you read all of this?!
    
    Ok(())
}

fn render_post(
    handlebars: &Handlebars,
    post: &Post,
    prev: Option<&Post>,
    next: Option<&Post>,
) -> Result<()> {
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

        "content": post.content()
            .chain_err(|| format!("couldn't process '{}'", post.title))?,
    });

    // Open the file.
    
    let mut file = File::create(format!("out/{}.html", post.slug))
        .chain_err(|| format!("couldn't open 'out/{}.html'", post.slug))?;

    // Render.

    handlebars.renderw("post", &data, &mut file)
        .chain_err(|| format!("couldn't render '{}'", post.title))?;

    Ok(())
}
