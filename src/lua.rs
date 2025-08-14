use anyhow::Result;
use anyhow::anyhow;
use std::{
    fs::{self},
    path::PathBuf,
};
use thirtyfour::WebDriver;

use mlua::Lua;

use crate::api::create_e2e_api;

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

    create_e2e_api(&lua, driver)?;
    lua.load(load_file(path_str)?).exec()?;

    Ok(())
}
