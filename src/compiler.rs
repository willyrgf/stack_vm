pub fn compile(expressions: sexp::Sexp) -> Vec<u8> {
    //TODO: implement a s-expression parser in another crate
    if let sexp::Sexp::List(l) = expressions {
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
    vec![]
}
