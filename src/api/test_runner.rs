use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use mlua::{AnyUserData, Function, Lua, UserData};

struct TestCase {
    desc: String,
    cb: Function,
}

pub struct TestRunner {
    test_cases: Rc<RefCell<Vec<TestCase>>>,
}

impl TestRunner {
    fn new() -> TestRunner {
        TestRunner {
            test_cases: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl UserData for TestRunner {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut(
            "describe",
            |lua, this, (desc, callback): (String, Function)| {
                println!("describe: {}", desc);

                let cases_ref = this.test_cases.clone();

                let test_fn = lua.create_function(move |_, (desc, cb): (String, Function)| {
                    println!("add test: {}", desc);
                    cases_ref.borrow_mut().push(TestCase { desc, cb });
                    Ok(())
                })?;

                callback.call::<()>(test_fn)?;
                Ok(())
            },
        );
    }
}

pub fn create_test_runner_api(lua: &Lua) -> Result<AnyUserData> {
    Ok(lua.create_userdata(TestRunner::new())?)
}
