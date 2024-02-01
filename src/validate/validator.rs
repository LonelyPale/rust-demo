use serde::Deserialize;

// A trait that the Validate derive will impl
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(phone)]
    phone1: String,
    #[validate(url)]
    site: String,
    #[validate(length(min = 1), custom = "validate_unique_username")]
    #[serde(rename = "firstName")]
    first_name: String,
    #[validate(range(min = 18, max = 20))]
    age: u32,
}

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}

#[test]
fn test() {
    let signup_data = SignupData {
        mail: "my@email.com".to_string(),
        phone1: "".to_string(),
        site: "https://baidu.com".to_string(),
        first_name: "hello_rust".to_string(),
        age: 19,
    };

    match signup_data.validate() {
        Ok(data) => println!("Ok: {:?}", data),
        Err(err) => println!("Err: {}", err),
    };
}
