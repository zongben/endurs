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
    pub before_all: Vec<Function>,
    pub before_each: Vec<Function>,
    pub after_each: Vec<Function>,
    pub after_all: Vec<Function>,
}

impl Describe {
    pub fn new(desc: String) -> Self {
        Describe {
            desc,
            tests: Vec::new(),
            before_all: Vec::new(),
            before_each: Vec::new(),
            after_each: Vec::new(),
            after_all: Vec::new(),
        }
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
            for ba in desc.before_all.iter() {
                ba.call_async::<()>(()).await?;
            }
            for test in desc.tests.iter() {
                for be in desc.before_each.iter() {
                    be.call_async::<()>(()).await?;
                }

                test.cb.call_async::<()>(()).await?;

                for ae in desc.after_each.iter() {
                    ae.call_async::<()>(()).await?;
                }
            }
            for aa in desc.after_all.iter() {
                aa.call_async::<()>(()).await?;
            }
        }
        Ok(())
    }
}
