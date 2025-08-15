use std::rc::Rc;

use anyhow::Result;

use mlua::Lua;
use thirtyfour::WebDriver;

use crate::api::{
    assert::create_assert_api, driver::create_webdriver_api, test_runner::{create_test_runner_api},
};

mod assert;
mod driver;
mod test_runner;

const ENTRYPOINT_NAME: &str = "endurs";

pub fn create_e2e_api(lua: &Lua, driver: Rc<WebDriver>) -> Result<()> {
    let webdriver_userdata = create_webdriver_api(lua, driver)?;
    let assert_userdata = create_assert_api(lua)?;
    let test_runner_userdata = create_test_runner_api(lua)?;

    let t = lua.create_table()?;
    t.set("driver", webdriver_userdata)?;
    t.set("assert", assert_userdata)?;
    t.set("test_runner", test_runner_userdata)?;
    lua.globals().set(ENTRYPOINT_NAME, t)?;

    Ok(())
}
