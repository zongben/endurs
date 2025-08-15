use std::rc::Rc;

use anyhow::Result;

use mlua::{Error, Lua, UserData};
use thirtyfour::WebDriver;

use crate::ENTRYPOINT_NAME;

pub struct Api {
    driver: Rc<WebDriver>,
}

impl UserData for Api {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("goto", |_, this, url: String| async move {
            this.driver.goto(url).await.map_err(Error::external)?;
            Ok(())
        });

        methods.add_async_method("close", |_, this, ()| async move {
            this.driver.close_window().await.map_err(Error::external)?;
            Ok(())
        });
    }
}

pub fn create_e2e_api(lua: &Lua, driver: Rc<WebDriver>) -> Result<()> {
    let userdata = lua.create_userdata(Api { driver })?;
    lua.globals().set(ENTRYPOINT_NAME, userdata)?;
    Ok(())
}
