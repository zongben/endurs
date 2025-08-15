use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use mlua::{AnyUserData, Function, Lua, UserData};

use crate::test_runner::{Describe, TestCase, TestRunner};

struct Runner {
    test_runner: Rc<RefCell<TestRunner>>,
}

impl UserData for Runner {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("describe", |lua, this, (desc, cb): (String, Function)| {
            let runner = this.test_runner.clone();

            let describe = Rc::new(RefCell::new(Describe::new(desc)));
            let d = describe.clone();
            let test_fn = lua.create_function(move |_, (desc, cb): (String, Function)| {
                d.borrow_mut().add_test(TestCase { desc, cb });
                Ok(())
            })?;

            cb.call::<()>(test_fn)?;
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
