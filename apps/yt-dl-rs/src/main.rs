use dialoguer::{theme::ColorfulTheme, Input};
use std::path::PathBuf;
use youtube_dl::YoutubeDl;

fn get_download_path() -> PathBuf {
    home::home_dir()
        .expect("Could not find home directory.")
        .join("Downloads")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a YouTube URL")
        .interact_text()?;

    let download_path = get_download_path();

    println!("Starting download...");

    tokio::task::spawn_blocking(move || {
        YoutubeDl::new(url)
            .format("bv*+ba/b")
            .download_to(download_path)
    })
    .await??;

    println!("Download finished!");

    Ok(())
}
