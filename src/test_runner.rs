use mlua::Function;

pub struct TestCase {
    pub desc: String,
    pub cb: Function,
}

pub struct TestRunner {
    test_cases: Vec<TestCase>,
}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {
            test_cases: Vec::new(),
        }
    }

    pub fn add_case(&mut self, case: TestCase) {
        self.test_cases.push(case);
    }

    pub fn get_cases(&self) -> &Vec<TestCase> {
        &self.test_cases
    }
}
