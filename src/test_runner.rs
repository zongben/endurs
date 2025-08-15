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
}

impl Describe {
    pub fn new(desc: String) -> Self {
        Describe {
            desc,
            tests: Vec::new(),
        }
    }

    pub fn add_test(&mut self, test: TestCase) {
        self.tests.push(test);
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
                test.cb.call_async::<()>(()).await?;
            }
        }
        Ok(())
    }
}
