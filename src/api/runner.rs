use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use mlua::{AnyUserData, Function, Lua, UserData};

use crate::test_runner::{Describe, TestCase, TestRunner};

fn create_test_fn(lua: &Lua, describe: Rc<RefCell<Describe>>) -> Result<Function> {
    let test_fn = lua.create_function(move |_, (desc, cb): (String, Function)| {
        describe.borrow_mut().add_test(TestCase { desc, cb });
        Ok(())
    })?;
    Ok(test_fn)
}

fn create_before_each_fn(lua: &Lua, describe: Rc<RefCell<Describe>>) -> Result<Function> {
    let before_each_fn = lua.create_function(move |_, cb: Function| {
        describe.borrow_mut().add_before_each_fn(cb);
        Ok(())
    })?;
    Ok(before_each_fn)
}

struct Runner {
    test_runner: Rc<RefCell<TestRunner>>,
}

impl UserData for Runner {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("describe", |lua, this, (desc, cb): (String, Function)| {
            let runner = this.test_runner.clone();

            let describe = Rc::new(RefCell::new(Describe::new(desc)));

            let test_fn = create_test_fn(lua, describe.clone())?;
            let before_each_fn = create_before_each_fn(lua, describe.clone())?;

            let hook_table = lua.create_table()?;
            hook_table.set("before_each", before_each_fn)?;

            cb.call::<()>((test_fn, hook_table))?;
            runner.borrow_mut().add_describe(describe.borrow().clone());
            Ok(())
        });
    }
}

pub fn create_test_runner_api(lua: &Lua, runner: Rc<RefCell<TestRunner>>) -> Result<AnyUserData> {
    Ok(lua.create_userdata(Runner {
        test_runner: runner,
    })?)
}
