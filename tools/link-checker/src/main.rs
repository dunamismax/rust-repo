use std::fs;
use reqwest;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.md>", args[0]);
        return;
    }

    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("Unable to read file");

    let client = reqwest::Client::new();
    let mut broken_links = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if let Some(_start) = line.find("[") {
            if let Some(_end) = line.find("]") {
                if let Some(link_start) = line.find("(") {
                    if let Some(link_end) = line.find(")") {
                        let url = &line[link_start + 1..link_end];
                        if url.starts_with("http") {
                            match client.get(url).send().await {
                                Ok(res) => {
                                    if !res.status().is_success() {
                                        println!("Broken link found at line {}: {}", i + 1, url);
                                        broken_links.push(url.to_string());
                                    }
                                }
                                Err(_) => {
                                    println!("Error checking link at line {}: {}", i + 1, url);
                                    broken_links.push(url.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if broken_links.is_empty() {
        println!("No broken links found.");
    } else {
        println!("
Found {} broken links:", broken_links.len());
        for link in broken_links {
            println!("- {}", link);
        }
    }
}