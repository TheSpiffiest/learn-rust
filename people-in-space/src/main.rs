use reqwest::Result;
use std::time::Duration;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let user = "ferris-the-crab";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send().await?;

    let x: i32 = 0;
    let y: i32 = 0;
    let x1: i32 = 5;
    let y1: i32 = 5;
    let c: f64 = (y1-y) as f64;
    let d = c.atan2((x1-x) as f64);
    println!("{}", d.to_degrees());

    if response.status().is_success() {
        println!("{} is a user!", user);
    } else {
        println!("{} is not a user!", user);
    }

    Ok(())
}