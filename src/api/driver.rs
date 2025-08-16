use std::rc::Rc;

use anyhow::Result;

use mlua::{AnyUserData, Error, Lua, UserData};
use thirtyfour::{By, WebDriver, WebElement};

fn create_by(by: String, value: String) -> Result<By> {
    let by = match by.as_str() {
        "id" => By::Id(value),
        "name" => By::Name(value),
        _ => return Err(Error::external(format!("{} is not supported", by)))?,
    };
    Ok(by)
}

struct Element {
    elem: WebElement,
}

impl UserData for Element {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("send_keys", |_, this, input: String| async move {
            this.elem.send_keys(input).await.map_err(Error::external)?;
            Ok(())
        });

        methods.add_async_method("click", |_, this, ()| async move {
            this.elem.click().await.map_err(Error::external)?;
            Ok(())
        });

        methods.add_async_method(
            "find",
            |lua, this, (by, value): (String, String)| async move {
                let elem = this
                    .elem
                    .find(create_by(by, value)?)
                    .await
                    .map_err(Error::external)?;
                let elem_userdata = lua.create_userdata(Element { elem });
                Ok(elem_userdata)
            },
        );
    }
}

struct Driver {
    driver: Rc<WebDriver>,
}

impl UserData for Driver {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("goto", |_, this, url: String| async move {
            this.driver.goto(url).await.map_err(Error::external)?;
            Ok(())
        });

        methods.add_async_method(
            "find",
            |lua, this, (by, value): (String, String)| async move {
                let elem = this
                    .driver
                    .find(create_by(by, value)?)
                    .await
                    .map_err(Error::external)?;
                let elem_userdata = lua.create_userdata(Element { elem });
                Ok(elem_userdata)
            },
        );

        methods.add_async_method(
            "find_all",
            |lua, this, (by, value): (String, String)| async move {
                let by = match by.as_str() {
                    "id" => By::Id(value),
                    "name" => By::Name(value),
                    _ => return Err(Error::external(format!("{} is not supported", by))),
                };

                let elems = this.driver.find_all(by).await.map_err(Error::external)?;

                let t = lua.create_table()?;
                for (i, elem) in elems.into_iter().enumerate() {
                    t.set(i + 1, lua.create_userdata(Element { elem })?)?;
                }

                Ok(t)
            },
        );

        methods.add_async_method("close", |_, this, ()| async move {
            this.driver.close_window().await.map_err(Error::external)?;
            Ok(())
        });
    }
}

pub fn create_webdriver_api(lua: &Lua, driver: Rc<WebDriver>) -> Result<AnyUserData> {
    Ok(lua.create_userdata(Driver { driver })?)
}
