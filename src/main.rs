use crate::slae::{SLAESupplier, Answer, Matrix};
use crate::base::test_runner::{TestsRunner, FailedTest};
use crate::base::runner::BatchStdIORunner;
use std::path::PathBuf;
use clap::Parser;
use rand::SeedableRng;

mod base;
mod slae;

#[derive(Debug, Parser)]
#[clap(about)]
struct Cli {
    /// Path to executable you want to test
    #[clap(parse(from_os_str), long)]
    executable: PathBuf,

    /// Name of lab you want to test. Options: slae
    #[clap(long)]
    lab: String,

    /// Seed to tune random generator
    #[clap(long, default_value="qwerty")]
    seed: String
}

fn main() {
    let args = Cli::parse();
    let slae = SLAESupplier::new();

    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(args.seed);
    let mut seed = [77;32];
    &mut seed[..].copy_from_slice(&hasher.finalize()[..]);

    let runner = TestsRunner::new(
        slae,
        BatchStdIORunner::new(args.executable)
    );

    let mut rng = rand::rngs::SmallRng::from_seed(seed);
    match runner.run(&mut rng) {
        Ok(failed) => {
            if let Some(failed) = failed {
                eprintln!("{}", failed);
            }
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    }

}
