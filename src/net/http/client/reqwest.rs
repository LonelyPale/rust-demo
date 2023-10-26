use std::collections::HashMap;

#[tokio::test]
async fn async_demo() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[test]
fn sync_demo() -> Result<(), Box<dyn std::error::Error>> {
    let resp =
        reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
