use crate::base::tests_supply::{TestsSupplier, TestsNode, Test};
use crate::base::runner::Runner;
use std::fmt::{Display, Formatter, Debug};
use rand::RngCore;


pub struct TestsRunner<Input, Output> where Input: Clone+Display, Output: Eq, Output: Clone+Display {
    supplier: Box<dyn TestsSupplier<Input, Output>>,
    runner: Box<dyn Runner<Input, Output>>
}

pub struct FailedTest<Input, Output> {
    pub test: Test<Input, Output>,
    pub answer: Output
}

impl <Input: Display, Output: Display>Debug for FailedTest<Input, Output> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl <Input: Display, Output: Display>Display for FailedTest<Input, Output> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Input:\n{}\nExpected:\n{}\nActual:\n{}", self.test.input, self.test.output, self.answer))
    }
}

impl <Input, Output>TestsRunner<Input, Output> where Input: Clone+Display, Output: Eq, Output: Clone+Display {
    pub fn new<Supplier, Run>(supplier: Supplier, runner: Run) -> TestsRunner<Input, Output>
        where Supplier: 'static + TestsSupplier<Input, Output>,
              Run: 'static + Runner<Input, Output>
    {
        TestsRunner {
            supplier: Box::new(supplier),
            runner: Box::new(runner),
        }
    }

    pub fn run(&self, rng: &mut dyn RngCore) -> Result<Option<FailedTest<Input, Output>>, String> {
        println!("Generating tests...");
        let tests = self.supplier.supply_tests(rng);
        println!("Tests generated");
        self.run_node(0, &tests)
    }

    fn run_chunk(&self, chunk: &[Test<Input, Output>]) -> Result<Option<FailedTest<Input, Output>>, String> {
        let mut result: Option<FailedTest<Input, Output>> = None;
        let mut last_test: Option<Test<Input, Output>> = None;
        let run_result = self.runner.run(chunk, &mut |test, output| {
            if !test.output.eq(output) {
                result = Some(FailedTest {
                    test: test.clone(),
                    answer: output.clone(),
                })
            }
            last_test = Some(test.clone());
            true
        });

        if let Err(e) = run_result {
            let last_test = last_test.or(chunk.first().cloned()).map(|a| a.to_string()).unwrap();
            return Err(format!("{}\nLast run test:\n{}", e, last_test))
        }

        Ok(result)
    }

    fn run_node(&self, indent_size: u16, node: &TestsNode<Input, Output>) -> Result<Option<FailedTest<Input, Output>>, String>{
        let indent = String::from("\t").repeat(indent_size.clone() as usize);
        if node.tests.is_empty() {
            println!("{}{}", indent, node.name);
        } else if node.tests.len() > 1 {
            print!("{}{}: {} ", indent, node.name, node.tests.len());
        } else {
            print!("{}{}:", indent, node.name);
        }

        if node.tests.len() > 10 {
            println!();
            let mut cnt = 0;
            for chunk in node.tests.chunks(10) {
                cnt += chunk.len();
                print!("{}\t{} {}:", indent, node.name, cnt);
                if let Some(e) = self.run_chunk(chunk)? {
                    return Ok(Some(e))
                }
                println!(" Passed");
            }
        } else if !node.tests.is_empty() {
            if let Some(e) = self.run_chunk(&node.tests)? {
                return Ok(Some(e))
            }
            println!(" Passed");
        }
        for child in &node.children {
            if let Some(e) = self.run_node(indent_size+1, child)? {
                return Ok(Some(e))
            }
        }
        return Ok(None)

    }
}