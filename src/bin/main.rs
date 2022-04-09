use env_logger::{Builder, Env};
use stack_vm::program::Program;

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    // let code = r#"
    // (
    //     (+ 5 2)
    //     (print "hello")
    // )
    // "#;

    let code = r#"
    (
        (+ Hello World)
    )
    "#;

    let mut program = Program::new();
    let result = program.exec(code);

    log::debug!("main(): result: {:?}", result);
}
