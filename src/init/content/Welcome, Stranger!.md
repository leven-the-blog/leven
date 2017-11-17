This is Leven, a minimal blogging framework that I hope you'll have fun using! Here are some instructions if you need them.

## Configuring

There are a couple of settings in `Leven.toml`, like your blog's name, that you can change.

## Writing

To make a new post:

1. Go to the `content` folder.
2. Create a new [Markdown](http://commonmark.org/help) file with your post's title as its name.
3. Write your post in your Markdown editor of choice!

If you want any images or other media on your site, just put it in the `content` directory, too - Leven will copy that over for you automatically.

## Building

After you're done writing, you'll want to *build* your blog - convert everything into a bunch of HTML files. This command does that for you:

```sh
leven build
```

After you run it, your website will be waiting for you in the `out` folder.

## Styling

The default theme isn't much to look at, but fortunately Leven comes with a command to change your blog's theme effortlessly. For example, to set the theme to [`sidney-pham/amazing-theme`](https://github.com/quadrupleslap/midnight), run this command:

```sh
leven theme sidney-pham/amazing-theme
```

## Publishing

It's a static website, so publishing is the easy part! You can choose from a vast number of hosting services, but my favorites are [Neocities](https://neocities.org) and [GitHub Pages](https://pages.github.com).
