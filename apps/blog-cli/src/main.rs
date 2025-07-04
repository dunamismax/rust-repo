use clap::Parser;
use markdown_generator::{create_post, Post};
use slug::slugify;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    title: String,

    #[clap(long)]
    author: String,
}

fn main() {
    let args = Args::parse();

    let post = Post {
        title: args.title.clone(),
        author: args.author,
        content: String::from("This is a placeholder for the blog post content."),
    };

    let markdown = create_post(&post);
    let file_name = format!("{}.md", slugify(&args.title));

    let mut file = File::create(&file_name).expect("Unable to create file");
    file.write_all(markdown.as_bytes())
        .expect("Unable to write to file");

    println!("Successfully generated blog post: {}", file_name);
}