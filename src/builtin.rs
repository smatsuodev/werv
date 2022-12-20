use crate::{
    ast::Expression,
    environment::Environment,
    eval::eval,
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

            println!("{}", eval(args[0].clone(), env).or(Err(()))?);
            Ok(NULL)
        }
        "print" => {
            if args.len() != 1 {
                return Err(());
            }

            print!("{}", eval(args[0].clone(), env).or(Err(()))?);
            Ok(NULL)
        }
        _ => Err(()),
    }
}
