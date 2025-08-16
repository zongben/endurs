use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use mlua::{AnyUserData, Function, Lua, UserData};

use crate::test_runner::{Describe, TestCase, TestRunner};

fn create_test_fn(lua: &Lua, describe: Rc<RefCell<Describe>>) -> Result<Function> {
    let test_fn = lua.create_function(move |_, (desc, cb): (String, Function)| {
        describe.borrow_mut().tests.push(TestCase { desc, cb });
        Ok(())
    })?;
    Ok(test_fn)
}

fn create_before_all_fn(lua: &Lua, describe: Rc<RefCell<Describe>>) -> Result<Function> {
    let before_all_fn = lua.create_function(move |_, cb: Function| {
        describe.borrow_mut().before_all.push(cb);
        Ok(())
    })?;
    Ok(before_all_fn)
}

fn create_before_each_fn(lua: &Lua, describe: Rc<RefCell<Describe>>) -> Result<Function> {
    let before_each_fn = lua.create_function(move |_, cb: Function| {
        describe.borrow_mut().before_each.push(cb);
        Ok(())
    })?;
    Ok(before_each_fn)
}

fn create_after_each_fn(lua: &Lua, describe: Rc<RefCell<Describe>>) -> Result<Function> {
    let after_each_fn = lua.create_function(move |_, cb: Function| {
        describe.borrow_mut().after_each.push(cb);
        Ok(())
    })?;
    Ok(after_each_fn)
}

fn create_after_all_fn(lua: &Lua, describe: Rc<RefCell<Describe>>) -> Result<Function> {
    let after_all_fn = lua.create_function(move |_, cb: Function| {
        describe.borrow_mut().after_all.push(cb);
        Ok(())
    })?;
    Ok(after_all_fn)
}

struct Runner {
    test_runner: Rc<RefCell<TestRunner>>,
}

impl UserData for Runner {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("describe", |lua, this, (desc, cb): (String, Function)| {
            let describe = Rc::new(RefCell::new(Describe::new(desc)));

            let test_fn = create_test_fn(lua, describe.clone())?;

            let hook_table = lua.create_table()?;
            hook_table.set("before_all", create_before_all_fn(lua, describe.clone())?)?;
            hook_table.set("before_each", create_before_each_fn(lua, describe.clone())?)?;
            hook_table.set("after_each", create_after_each_fn(lua, describe.clone())?)?;
            hook_table.set("after_all", create_after_all_fn(lua, describe.clone())?)?;

            cb.call::<()>((test_fn, hook_table))?;
            this.test_runner
                .borrow_mut()
                .add_describe(describe.borrow().clone());
            Ok(())
        });
    }
}

pub fn create_test_runner_api(lua: &Lua, runner: Rc<RefCell<TestRunner>>) -> Result<AnyUserData> {
    Ok(lua.create_userdata(Runner {
        test_runner: runner,
    })?)
}
