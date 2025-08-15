use anyhow::Result;
use anyhow::anyhow;
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    fs::{self},
    path::PathBuf,
};
use thirtyfour::WebDriver;

use mlua::Lua;

use crate::api::create_e2e_api;
use crate::test_runner::TestRunner;

fn path_buf_to_str(path_buf: &PathBuf) -> Result<&str> {
    path_buf
        .to_str()
        .ok_or_else(|| anyhow!(format!("Path is not valid UTF-8: {}", path_buf.display())))
}

fn is_lua_file(path: &str) -> anyhow::Result<()> {
    let err_msg = format!("Path: {}, it's not a Lua file", path);
    path.split('.')
        .last()
        .ok_or_else(|| anyhow!(err_msg.clone()))?
        .eq("lua")
        .then(|| ())
        .ok_or_else(|| anyhow!(err_msg))
}

fn load_file(path: &str) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub async fn exec_lua(path_buf: PathBuf, driver: WebDriver) -> Result<()> {
    let path_str = path_buf_to_str(&path_buf)?;
    is_lua_file(path_str)?;

    let lua = Lua::new();
    let rc_driver = Rc::new(driver);
    let runner = Rc::new(RefCell::new(TestRunner::new()));

    create_e2e_api(&lua, rc_driver.clone(), runner.clone())?;

    lua.load(load_file(path_str)?).exec_async().await?;

    let runner_ref = runner.borrow();
    runner_ref.exec_tests().await?;

    if let Ok(driver) = Rc::try_unwrap(rc_driver) {
        driver.quit().await?;
    }
    Ok(())
}
