use anyhow::{anyhow, Context, Error, Result};

#[test]
fn test_error() {
    let result = get_err();
    // let result = other_err()1;
    match result {
        Ok(_) => {}
        Err(err) => {
            if let Some(any_err) = err.downcast_ref::<Error>() {
                //错误：无效分支，永远不会执行到。
                eprintln!("any_err: {:?}", any_err);
            } else if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
                eprintln!("err: {:?}", err);
                eprintln!("io_err: {:?}", io_err);
                eprintln!("kind: {}\nmessage: {}", io_err.kind(), io_err.to_string())
            } else if let Some(json_err) = err.downcast_ref::<serde_json::Error>() {
                eprintln!("json_err: {:?}", json_err);
            } else {
                eprintln!("other_err_1: {}\n=====", err);
                eprintln!("other_err_2: {:#}\n=====", err);
                eprintln!("other_err_3: {:?}\n=====", err);
                eprintln!("other_err_4: {:#?}\n=====", err);
            }
        }
    }
}

fn get_err() -> Result<()> {
    any_err1().context("context_err1")?;
    any_err2()?;
    // io_err().context("context_err1")?;
    io_err().with_context(|| "with_context1")?;
    io_err2().context("context_err2")?;
    json_err()?;
    Ok(())
}

fn other_err1() -> std::result::Result<String, std::io::Error> {
    std::fs::read_to_string("")
}

fn other_err2() -> Result<()> {
    let _ = other_err1().context("other_err2")?;
    Ok(())
}

fn any_err1() -> Result<()> {
    Err(anyhow!("any_err1"))
}

fn any_err2() -> Result<()> {
    Err(Error::msg("any_err2"))
}

fn io_err() -> Result<()> {
    let _ = std::fs::read_to_string("")?;
    Ok(())
}

fn io_err2() -> Result<()> {
    let _ = other_err2().context("io_err2")?;
    Ok(())
}

fn json_err() -> Result<()> {
    let _ = serde_json::from_str("")?;
    Ok(())
}
