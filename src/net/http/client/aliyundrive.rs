use base64::Engine;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

///自定义类型Error,实现std::fmt::Debug的trait
#[derive(Debug, serde::Deserialize)]
struct DriveError {
    code: String,
    message: String,
}

///实现Display的trait，并实现fmt方法
impl std::fmt::Display for DriveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DriveError")
    }
}

///实现Error的trait,因为没有子Error,不需要覆盖source()方法
///实现Error的trait,因为有子Error:ChildError,需要覆盖source()方法,返回Some(err)
impl std::error::Error for DriveError {}

#[derive(Debug, serde::Deserialize)]
struct DriveResult {
    phone: String,
    user_id: String,
    domain_id: String,
    user_name: String,
    nick_name: String,
    default_drive_id: String,
    backup_drive_id: String,
    resource_drive_id: String,
}

const PREFIX_LEN: usize = "Bearer ".len();
// const TOKEN: &'static str = "Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NWFhOWE2OTA4MjY0MDJmOWFiYzU0ZWU1NmQxZjMyNyIsImN1c3RvbUpzb24iOiJ7XCJjbGllbnRJZFwiOlwiMjVkelgzdmJZcWt0Vnh5WFwiLFwiZG9tYWluSWRcIjpcImJqMjlcIixcInNjb3BlXCI6W1wiRFJJVkUuQUxMXCIsXCJTSEFSRS5BTExcIixcIkZJTEUuQUxMXCIsXCJVU0VSLkFMTFwiLFwiVklFVy5BTExcIixcIlNUT1JBR0UuQUxMXCIsXCJTVE9SQUdFRklMRS5MSVNUXCIsXCJCQVRDSFwiLFwiT0FVVEguQUxMXCIsXCJJTUFHRS5BTExcIixcIklOVklURS5BTExcIixcIkFDQ09VTlQuQUxMXCIsXCJTWU5DTUFQUElORy5MSVNUXCIsXCJTWU5DTUFQUElORy5ERUxFVEVcIl0sXCJyb2xlXCI6XCJ1c2VyXCIsXCJyZWZcIjpcImh0dHBzOi8vd3d3LmFsaXl1bmRyaXZlLmNvbS9cIixcImRldmljZV9pZFwiOlwiZTZlMjRkZDVmMDFhNDg4MmE3NGEyMDM3ZDVjMjEzMzRcIn0iLCJleHAiOjE2OTg0MjYwMDMsImlhdCI6MTY5ODQxODc0M30.fs8gnfOmeR7ALdaQeRRgdff6GLhp1FKMD68G9k4MoWFzLT1-t90ZefpqTbIY9oaqE5VnaURC_31mr1kShXhNkckezF0JRKF7Fja7DicxoWZ1ZxCliVmx24KO8HHalqOHJLLicFSfb_hLabm_TJkwCyHo-wC2dAeYr081KENCzzI";
const TOKEN: &'static str = "Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NWFhOWE2OTA4MjY0MDJmOWFiYzU0ZWU1NmQxZjMyNyIsImN1c3RvbUpzb24iOiJ7XCJjbGllbnRJZFwiOlwiMjVkelgzdmJZcWt0Vnh5WFwiLFwiZG9tYWluSWRcIjpcImJqMjlcIixcInNjb3BlXCI6W1wiRFJJVkUuQUxMXCIsXCJTSEFSRS5BTExcIixcIkZJTEUuQUxMXCIsXCJVU0VSLkFMTFwiLFwiVklFVy5BTExcIixcIlNUT1JBR0UuQUxMXCIsXCJTVE9SQUdFRklMRS5MSVNUXCIsXCJCQVRDSFwiLFwiT0FVVEguQUxMXCIsXCJJTUFHRS5BTExcIixcIklOVklURS5BTExcIixcIkFDQ09VTlQuQUxMXCIsXCJTWU5DTUFQUElORy5MSVNUXCIsXCJTWU5DTUFQUElORy5ERUxFVEVcIl0sXCJyb2xlXCI6XCJ1c2VyXCIsXCJyZWZcIjpcImh0dHBzOi8vd3d3LmFsaXl1bmRyaXZlLmNvbS9cIixcImRldmljZV9pZFwiOlwiYTRjYzViM2Y3MjU0NDBhZjg1MjRiZWU3MjM2MTdkMzRcIn0iLCJleHAiOjE2OTg2MDk4MzMsImlhdCI6MTY5ODYwMjU3M30.KA27x1e1bH2EUnihnzUfonXq95I1gMt7S-rlysDJsAEIVPM_8Oh2g5t3u4XqxukbPrC1OzHdUGY1ouQga40N6EiZouqWXUXOU4Qflkn1Rk_XPYU9-fJig-T7U5L2w2Vj4l2NNGCV-IK8ljA1yUUflLTa-ZfSx0k4kVHPrP2Kd1A";

#[tokio::test]
async fn user_get_struct() -> Result<()> {
    let url = "https://user.aliyundrive.com/v2/user/get";
    let result: DriveResult = post(url).await?;
    println!("Result: {:#?}", result);
    Ok(())
}

#[tokio::test]
async fn user_get_value() {
    let url = "https://user.aliyundrive.com/v2/user/get";
    let value: serde_json::Value = post(url).await.unwrap();
    println!("Result: {:#?}", value);
}

async fn get<T: serde::de::DeserializeOwned, U: reqwest::IntoUrl>(url: U) -> Result<T> {
    request(http::Method::GET, url).await
}

async fn post<T: serde::de::DeserializeOwned, U: reqwest::IntoUrl>(url: U) -> Result<T> {
    request(http::Method::POST, url).await
}

async fn request<T: serde::de::DeserializeOwned, U: reqwest::IntoUrl>(
    method: http::Method,
    url: U,
) -> Result<T> {
    let mut headers = http::header::HeaderMap::new();
    headers.insert("Authorization", TOKEN.parse()?);

    let client = reqwest::Client::new();
    let resp = client.request(method, url).headers(headers).send().await?;
    let version = resp.version();
    let status = resp.status();
    let content_length = resp.content_length().unwrap_or_default();
    // let headers = resp.headers(); //error[E0505]: cannot move out of `resp` because it is borrowed
    let headers = resp.headers().clone();
    let bytes = resp.bytes().await?;
    let text = std::str::from_utf8(&*bytes)?;

    // println!("{:#?}", resp);
    // println!("byte: {:#?}", resp.bytes().await?);
    // println!("text: {:#?}", resp.text().await?);
    // println!("json: {:#?}", resp.json().await?);
    println!("Version: {:?}", version);
    println!("Status: {}", status);
    println!("ContentLength: {}", content_length);
    println!("Headers: {:#?}", headers);
    println!("Bytes: {:?}", bytes);
    println!("Text: {:?}", text);

    if status.is_success() {
        let result: T = serde_json::from_slice(&*bytes)?;
        Ok(result)
    } else {
        let err: DriveError = serde_json::from_slice(&*bytes)?;
        Err(Box::new(err))
    }
}

#[test]
fn jwt_info() {
    let token = &TOKEN[PREFIX_LEN..];
    let arr: Vec<&str> = token.split('.').collect();
    println!("token: {token}");
    println!("arr: {:?}", arr);

    for (i, v) in arr.iter().enumerate() {
        let raw = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(v)
            .unwrap();
        let s: String = String::from_utf8_lossy(&raw).to_string();

        if i < 2 {
            let value: serde_json::Value = serde_json::from_str(&*s).unwrap();
            println!("({i}) {:#?} {s} {v}", value);

            if i == 1 {
                let str = value.get("customJson").unwrap().to_string();
                let str = &str[1..str.len() - 1].replace("\\\"", "\"");
                let val: serde_json::Value = serde_json::from_str(&*str).unwrap();
                println!("{:#?} {str}", val);
            }
        } else {
            println!("({i}) {s} {v}");
        }
    }
}
