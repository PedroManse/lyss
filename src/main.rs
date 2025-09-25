use std::path::PathBuf;

fn main() {
    let file_name = PathBuf::from("hello.ls");
    let file = std::fs::read_to_string(&file_name).unwrap();
    let o = lyss::tokenizer::tokenize(&file, &file_name);
    let mut i = o.unwrap().into_iter();
    let exprs = lyss::parser::parse(&mut i).unwrap();
    for expr in exprs {
        println!("{}", expr.cont);
    }
}
