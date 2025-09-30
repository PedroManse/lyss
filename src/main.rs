use std::collections::HashMap;
use std::path::PathBuf;

use lyss::Value;
use lyss::display::DisplayValue;
use lyss::parser::{Argument, FnName};
use lyss::runtime::HostFunc;
use lyss::runtime::object::ObjectEntry;

fn main() {
    let file_name = PathBuf::from("hello.ls");
    let file = std::fs::read_to_string(&file_name).unwrap();
    let o = lyss::tokenizer::tokenize(&file, &file_name);
    let mut i = o.unwrap().into_iter();
    let exprs = lyss::parser::parse(&mut i).unwrap();

    let mut ctx = lyss::runtime::HostContext::new();
    let mut builtins: HashMap<String, ObjectEntry<HostFunc>> = HashMap::new();

    builtins.insert(
        "local".to_string(),
        ObjectEntry::Leaf(HostFunc(|ctx, args| {
            let name = if let Some(Argument::Value(Value::Ident(FnName(path)))) = &args.first()
                && let Some(name) = path.first()
            {
                name.to_string()
            } else {
                todo!()
            };
            let value = ctx.eval_argument(&args[1])?;
            ctx.set_var(name, value.clone());
            Ok(value)
        })),
    );

    builtins.insert(
        "print".to_string(),
        ObjectEntry::Leaf(HostFunc(|ctx, args| {
            let mut out = String::new();
            for arg in args.iter() {
                let value = ctx.eval_argument(arg)?;
                let cnt = DisplayValue(value).to_string();
                out.push_str(&cnt);
            }
            print!("{out}");
            Ok(Value::Num(out.len() as f64))
        })),
    );

    ctx.register_object(
        "Builtin".to_string(),
        lyss::runtime::object::Object(builtins),
    );
    ctx.run(&exprs).unwrap();
}
