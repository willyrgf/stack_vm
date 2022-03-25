use env_logger::{Builder, Env};
use stack_vm::program::Program;
// use stack_vm

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let mut program = Program::new();
    let result = program.exec("");

    log::debug!("main(): result: {:?}", result);
}
