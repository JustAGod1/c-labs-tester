use std::fmt::{Display, Formatter, Debug};
use rand::RngCore;

pub struct Test<Input, Output> {
    pub input: Input,
    pub output: Output
}

impl <Input, Output>Test<Input, Output> {

    pub fn new(input: Input, output: Output) -> Test<Input, Output> {
        Test {
            input, output
        }
    }
}

impl<A: Clone, B: Clone> Clone for Test<A, B> {
    fn clone(&self) -> Self {
        Test {
            input: self.input.clone(),
            output: self.output.clone()
        }
    }
}

impl <A: Display, B: Display> Debug for Test<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl <A: Display, B: Display> Display for Test<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Input: \n{}\nOutput:\n{}", self.input, self.output))
    }
}

pub struct TestsNode<Input, Output> {
    pub name: String,
    pub tests: Vec<Test<Input, Output>>,
    pub children: Vec<TestsNode<Input, Output>>
}

impl <Input, Output>TestsNode<Input, Output> {

    pub fn new<S: Into<String>>(name: S) -> TestsNode<Input, Output> {
        TestsNode {
            name: name.into(),
            tests: Vec::new(),
            children: Vec::new()
        }
    }

    pub fn add_test(&mut self, test: Test<Input, Output>) -> &mut Self {
        self.tests.push(test);

        self
    }

    pub fn child<S : Into<String>>(&mut self, name: S) -> &mut TestsNode<Input, Output> {
        let child = TestsNode::new(name);

        self.children.push(child);

        let idx = self.children.len()-1;
        self.children.get_mut(idx).unwrap()
    }
}


pub trait TestsSupplier<Input, Output> {
    fn supply_tests(&self, rng: &mut dyn RngCore) -> TestsNode<Input, Output>;
}

