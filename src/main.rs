use std::collections::HashMap;
use std::path::PathBuf;

use lyss::display::DisplayValue;
use lyss::runtime::HostFunc;
use lyss::runtime::api::Api;
use lyss::runtime::object::ObjectEntry;
use lyss::{LyssRuntimeError, Value};

fn main() {
    let file_name = PathBuf::from("hello.ls");
    let file = std::fs::read_to_string(&file_name).unwrap();
    let o = lyss::tokenizer::tokenize(&file, &file_name);
    let mut i = o.unwrap().into_iter();
    let exprs = lyss::parser::parse(&mut i).unwrap();

    let mut ctx = lyss::runtime::HostContext::new();
    let mut builtins: HashMap<String, ObjectEntry<HostFunc>> = HashMap::new();
    let mut math: HashMap<String, ObjectEntry<HostFunc>> = HashMap::new();

    builtins.insert(
        "local".to_owned(),
        ObjectEntry::Leaf(HostFunc(|ctx, args| {
            Api::assert_args_count(args, 2)?;
            let var_name = Api::needs_nth_arg(args, 0)?;
            let name = Api::expect_ident(var_name).ok_or(LyssRuntimeError::UnexpectedArg {
                arg: var_name.clone(),
                expected: "Identifier",
            })?;
            let value = Api::needs_nth_arg(args, 1)?;
            let value = ctx.eval_argument(value)?;
            ctx.set_var(name.to_string(), value.clone());
            Ok(value)
        })),
    );

    builtins.insert(
        "print".to_owned(),
        ObjectEntry::Leaf(HostFunc(|ctx, args| {
            let mut out = String::new();
            for arg in args {
                let value = ctx.eval_argument(arg)?;
                let cnt = DisplayValue(value).to_string();
                out.push_str(&cnt);
            }
            print!("{out}");
            Ok(Value::Num(out.len() as f64))
        })),
    );

    math.insert(
        "=".to_owned(),
        ObjectEntry::Leaf(HostFunc(|ctx, args| {
            let lhs = Api::needs_nth_arg(args, 0)?;
            let rhs = Api::needs_nth_arg(args, 1)?;
            let lhs = ctx.eval_argument(lhs)?;
            let rhs = ctx.eval_argument(rhs)?;
            todo!()
        })),
    );

    builtins.insert(
        "Math".to_owned(),
        ObjectEntry::Branch(lyss::runtime::object::Object(math)),
    );
    ctx.register_object(
        "Builtin".to_owned(),
        lyss::runtime::object::Object(builtins),
    );
    ctx.register_entry(
        "if".to_owned(),
        ObjectEntry::Leaf(HostFunc(|ctx, args| {
            if args.len() != 4 {
                return Err(LyssRuntimeError::UnmatchedArgCount {
                    got: args.to_vec(),
                    could_usize: vec![2, 4],
                });
            }
            let if_code = Api::needs_nth_arg(args, 0)?;
            Api::expect_this_text(Api::needs_nth_arg(args, 2)?, "else")?;

            let Some(Value::Code(if_code)) = Api::expect_literal(if_code) else {
                return Err(LyssRuntimeError::UnexpectedArg {
                    arg: if_code.clone(),
                    expected: "code",
                });
            };
            let Some(if_res) = ctx.run(&if_code.exprs)? else {
                return Err(LyssRuntimeError::NeedsArg);
            };
            let Value::Bool(if_res) = if_res else {
                return Err(LyssRuntimeError::UnexpectedArg {
                    arg: lyss::parser::Argument::Value(if_res),
                    expected: "boolean",
                });
            };

            Ok(if if_res {
                let true_code = Api::needs_nth_arg(args, 1)?;
                let Some(Value::Code(true_code)) = Api::expect_literal(true_code) else {
                    return Err(LyssRuntimeError::UnexpectedArg {
                        arg: true_code.clone(),
                        expected: "code",
                    });
                };
                ctx.run(&true_code.exprs)?
                    .ok_or(LyssRuntimeError::NeedsArg)?
            } else {
                let false_code = Api::needs_nth_arg(args, 3)?;
                let Some(Value::Code(false_code)) = Api::expect_literal(false_code) else {
                    return Err(LyssRuntimeError::UnexpectedArg {
                        arg: false_code.clone(),
                        expected: "code",
                    });
                };
                ctx.run(&false_code.exprs)?
                    .ok_or(LyssRuntimeError::NeedsArg)?
            })
        })),
    );
    ctx.run(&exprs).unwrap();
}
