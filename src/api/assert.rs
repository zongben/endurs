use anyhow::Result;
use mlua::{AnyUserData, Lua, UserData, Value};

struct Assert {}

impl UserData for Assert {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("eq", |_, _, (actual, expect): (Value, Value)| {
            if actual != expect {
                println!("✖ eq failed: {:?} != {:?}", actual, expect);
            }
            Ok(())
        });
    }
}

pub fn create_assert_api(lua: &Lua) -> Result<AnyUserData> {
    Ok(lua.create_userdata(Assert {})?)
}
