use anyhow::Result;
use mlua::Function;

#[derive(Clone)]
pub struct TestCase {
    pub desc: String,
    pub cb: Function,
}

#[derive(Clone)]
pub struct Describe {
    pub desc: String,
    pub tests: Vec<TestCase>,
    pub before_each: Vec<Function>,
}

impl Describe {
    pub fn new(desc: String) -> Self {
        Describe {
            desc,
            tests: Vec::new(),
            before_each: Vec::new(),
        }
    }

    pub fn add_test(&mut self, test: TestCase) {
        self.tests.push(test);
    }

    pub fn add_before_each_fn(&mut self, cb: Function) {
        self.before_each.push(cb);
    }
}

pub struct TestRunner {
    describes: Vec<Describe>,
}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {
            describes: Vec::new(),
        }
    }

    pub fn add_describe(&mut self, describe: Describe) {
        self.describes.push(describe);
    }

    pub async fn exec_tests(&self) -> Result<()> {
        for desc in self.describes.iter() {
            for test in desc.tests.iter() {
                for be in desc.before_each.iter() {
                    be.call_async::<()>(()).await?;
                }
                test.cb.call_async::<()>(()).await?;
            }
        }
        Ok(())
    }
}
