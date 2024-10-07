use serde::{Deserialize, Serialize};
use reqwest::{self, StatusCode};
use std::io::{self, Write};

#[derive(Deserialize, Serialize, Debug)]
struct Blog {
    _id: Option<String>,
    title: String,
    author: String,
    content: String,
    date: String,
}

type BlogsResponse = Vec<Blog>;

fn get_blogs() -> Result<Vec<Blog>, Box<dyn std::error::Error>> {
    let url = format!("https://portfolioapi-hysa.onrender.com/Blogs");
    let response = reqwest::blocking::get(&url)?.json::<BlogsResponse>()?;
    
    Ok(response)
}

fn add_blog(blog: Blog) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://portfolioapi-hysa.onrender.com/Blogs";
    let client = reqwest::blocking::Client::new();
    let response = client.post(url)
        .json(&blog)
        .send()?;

        if response.status() == StatusCode::OK {
            println!("Blog added successfully!");
        } else {
            println!("Failed to add blog. Status: {:?}", response.status());
            println!("Response Body: {:?}", response.text()?);
        }

    Ok(())
}

fn main() {
    match get_blogs() {
        Ok(blogs) => {
            for blog in blogs {
                println!("Title: {}", blog.title);
                println!("Author: {}", blog.author);
                println!("Content: {}", blog.content);
                println!("Date: {}", blog.date);
                println!("----------------------");
            }
        },
        Err(e) => eprintln!("Error fetching blogs: {}", e),
    }

    let mut title = String::new();
    let author = "Håkon Sunde";
    let mut content = String::new();
    let mut date = String::new();

    print!("Enter title: ");
    io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut title).unwrap();
    title = title.trim().to_string(); 

    print!("Write blog content: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut content).unwrap();
    content = content.trim().to_string();

    print!("Enter blog date (YYYY-MM-DD): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut date).unwrap();
    date = date.trim().to_string();

    let new_blog = Blog {
        _id: None,  
        title,
        author: author.to_string(),
        content,
        date,
    };

    if let Err(e) = add_blog(new_blog) {
        eprintln!("Error adding blog: {}", e);
    }
}
