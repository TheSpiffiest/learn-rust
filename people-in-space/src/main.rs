use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let people_in_space = "http://api.open-notify.org/astros.json";
    let request_url = people_in_space;


    let resp = reqwest::get(request_url)
        .await?
        .text()
        .await?;
    println!("{}", resp);

    Ok(())
}