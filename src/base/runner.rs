use std::convert::TryFrom;
use std::io::{Read, Stdin, Write, ErrorKind};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::process::Stdio;

use crate::base::tests_supply::Test;

pub trait TryFromVerbal: Sized {
    fn try_from(s: &str) -> Result<Self, String>;
}

pub trait Runner<Input, Output> {
    fn run(&self, input: &[Test<Input, Output>], listener: &mut dyn FnMut(&Test<Input, Output>, &Output) -> bool) -> Result<(), String>;
}

pub struct BatchStdIORunner {
    file: PathBuf,
}

impl BatchStdIORunner {
    pub fn new(file: PathBuf) -> Self {
        BatchStdIORunner { file }
    }
}

fn enforce_interruptable_io<T, F>(block: &mut F) -> std::io::Result<T> where F : FnMut() -> std::io::Result<T> {
    loop {
        match block() {
            Ok(r) => {
                return Ok(r);
            }
            Err(e) => {
                if e.kind() != ErrorKind::Interrupted {
                    return Err(e)
                }
            }
        }
    }
}

impl<Input: ToString, Output: TryFromVerbal> Runner<Input, Output> for BatchStdIORunner {
    fn run(&self, input: &[Test<Input, Output>], listener: &mut dyn FnMut(&Test<Input, Output>, &Output) -> bool) -> Result<(), String> {
        let mut cmd = std::process::Command::new(&self.file);
        cmd.arg("-");
        cmd.arg("-");
        cmd.env("TEST", "true");
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());

        let mut process = cmd.spawn().map_err(|a| a.to_string())?;

        let process_input = process.stdin.as_mut().unwrap();
        let process_output = process.stdout.as_mut().unwrap();


        for test in input {
            let s = test.input.to_string();

            enforce_interruptable_io(&mut || {
                process_input.write_all(s.as_bytes())
            }).map_err(|a| format!("Cannot write to process: {}", a))?;
            process_input.flush().map_err(|a| format!("Cannot flush to process output: {}", a))?;

            let mut output = Vec::<u8>::new();
            let mut buf = [0u8; 1024];

            let read = enforce_interruptable_io(&mut || {
                process_output.read(&mut buf[..])
            }).map_err(|a| format!("Cannot read from process output: {}", a))?;

            &buf[0..read].iter().for_each(|a| output.push(*a));


            let output = String::from_utf8(output).map_err(|a| format!("Cannot parse output to UTF string: {}", a))?;

            let output = Output::try_from(&output).map_err(|a| format!("{}: {}", a, output))?;

            if !listener(&test, &output) {
                break;
            }
        }

        match process.try_wait()
            .map_err(|a| format!("Cannot check if process is died: {}", a))? {
            Some(code) => {
                if !code.success() {
                    return Err(format!("Process exited with exit code: {}", code.code().unwrap()));
                }
            }
            None => {
                process.kill().map_err(|a| format!("Cannot kill process: {}", a))?;
            }
        }

        return Ok(());
    }
}