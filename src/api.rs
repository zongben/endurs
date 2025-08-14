use anyhow::Result;

use mlua::{Error, Lua, UserData};
use thirtyfour::WebDriver;

pub struct Api {
    driver: WebDriver,
}

impl UserData for Api {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("add", |_, _, (a, b): (i32, i32)| {
            let result = a + b;
            Ok(result)
        });

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

pub fn create_e2e_api(lua: &Lua, driver: WebDriver) -> Result<()> {
    let userdata = lua.create_userdata(Api { driver })?;
    lua.globals().set("endurs", &userdata)?;
    Ok(())
}
