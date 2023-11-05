use tokio::io::AsyncReadExt;

#[tokio::test]
async fn read_file() {
    let filepath = "data/index.m3u8";
    let content = tokio::fs::read(filepath).await.unwrap();
    let text = std::str::from_utf8(&*content).unwrap();
    println!("The text: {}", text);
}

#[tokio::test]
async fn read_file1() {
    let filepath = "data/index.m3u8";
    let mut f = tokio::fs::File::open(filepath).await.unwrap();
    let mut buffer = [0; 1000];
    let n = f.read(&mut buffer).await.unwrap();
    let text = std::str::from_utf8(&buffer[..n]).unwrap();
    println!("The bytes: {:?}", &buffer[..n]);
    println!("The text: {}", text);
}
