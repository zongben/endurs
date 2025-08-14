use anyhow::Result;

use mlua::{Lua, UserData};
use thirtyfour::WebDriver;

struct Api {
    driver: WebDriver,
}

impl UserData for Api {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("add", |_, _, (a, b): (i32, i32)| {
            let result = a + b;
            Ok(result)
        });
    }
}

pub fn create_e2e_api(lua: &Lua, driver: WebDriver) -> Result<()> {
    let api = lua.create_userdata(Api { driver })?;

    lua.globals().set("endura", api)?;
    Ok(())
}
