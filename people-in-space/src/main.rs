
fn main() {
    let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;

    println!("body = {:?}", body);

    // let response = requests::get("http://api.open-notify.org/astros.json").unwrap();
    // let data = response.json().unwrap();
    // println!("{:?}", data);
}