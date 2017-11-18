Leven is a minimal blogging framework which I hope you'll have fun using! Here's how to get stuff done.

## Configuring

Open `Leven.toml` in a text editor.

## Writing

Create a [Markdown](http://commonmark.org/help) document in the `content` directory. Its title should be the title of your post.

## Building

Once you're done writing, run this command:

```sh
leven build
```

The compiled website should now be in the `out` directory.

## Testing

After building the website, run this command:

```sh
cd out
python3 -m http.server 8080
```

Then open `localhost:8080` in your web browser to test out your blog.

## Publishing

It's a static website, so publishing is the easy part! You can choose from countless hosting services, but my favorites are [Neocities](https://neocities.org) and [GitHub Pages](https://pages.github.com). Just upload everything in the `out` directory.

## Styling

The default theme isn't much to look at, but fortunately Leven comes with a command to change your blog's theme effortlessly. For example, to set the theme to [`sidney-pham/amazing-theme`](https://github.com/sidney-pham/amazing-theme), run this command:

```sh
leven theme sidney-pham/amazing-theme
```
