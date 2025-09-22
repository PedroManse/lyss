use std::path::PathBuf;

fn main() {
    let file_name = PathBuf::from("hello.ls");
    let file = std::fs::read_to_string(&file_name).unwrap();
    let o = lyss::tokenizer::tokenize(&file, &file_name);
    println!("{o:?}");
    println!("Hello, world!");
}
