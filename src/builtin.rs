use crate::{
    ast::Expression,
    eval::Environment,
    object::{Object, NULL},
};

pub fn call_builtin(
    name: &str,
    args: &Vec<Expression>,
    env: &mut Environment,
) -> Result<Object, ()> {
    match name {
        "println" => {
            if args.len() != 1 {
                return Err(());
            }

            println!("{}", env.eval(args[0].clone()).or(Err(()))?);
            Ok(NULL)
        }
        "print" => {
            if args.len() != 1 {
                return Err(());
            }

            print!("{}", env.eval(args[0].clone()).or(Err(()))?);
            Ok(NULL)
        }
        _ => Err(()),
    }
}
