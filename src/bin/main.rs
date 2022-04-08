use env_logger::{Builder, Env};
use stack_vm::program::Program;

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let s_exp = sexp::parse(r#"((+ 5 2) (print "hello"))"#).unwrap();

    //TODO: implement a s-express parser in another crate
    if let sexp::Sexp::List(l) = s_exp {
        log::debug!("l: {:?}", l);

        // FIXME: do it recursively in compiler module
        l.iter().for_each(|t| match t {
            sexp::Sexp::List(ll) => {
                log::debug!("ll: {:?}", ll);
                ll.iter().for_each(|tt| match tt {
                    sexp::Sexp::List(lll) => {
                        log::debug!("lll: {:?}", lll);
                    }
                    sexp::Sexp::Atom(aaa) => {
                        log::debug!("aaa: {:?}", aaa);
                    }
                })
            }
            sexp::Sexp::Atom(aa) => {
                log::debug!("aa: {:?}", aa);
            }
        });
    }

    let mut program = Program::new();
    let result = program.exec("");

    log::debug!("main(): result: {:?}", result);
}
