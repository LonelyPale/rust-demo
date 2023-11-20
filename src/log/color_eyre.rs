use anyhow::Context;
use color_eyre::{eyre::eyre, Result};
use tracing::{error, info, instrument, Level};

#[instrument]
fn return_err() -> Result<()> {
    Err(eyre!("Something went wrong")
        .wrap_err("e1")
        .wrap_err("e2")
        .wrap_err("e3"))
}

#[instrument]
fn call_return_err() {
    info!("going to log error");
    if let Err(err) = return_err() {
        // 推荐大家运行下，看看这里的输出效果
        error!(?err, "error-test");
    }
}

fn get_err() -> anyhow::Result<()> {
    std::fs::read("")
        .context("err1")
        .context("err2")
        .context("err3")?;
    Ok(())
}

#[test]
fn test() -> anyhow::Result<()> {
    get_err()?;
    Ok(())
}

#[test]
fn test1() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    call_return_err();

    if let Err(err) = get_err() {
        error!(?err, "error-test1");
    }

    Ok(())
}

#[test]
fn test2() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_thread_names(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .init();
    // 安裝 color-eyre-demo 的 panic 处理句柄
    color_eyre::install()?;

    call_return_err();

    if let Err(err) = get_err() {
        error!(?err, "error-test1");
    }

    Ok(())
}
