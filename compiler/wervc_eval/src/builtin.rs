use wervc_ast::{Expression, Ident};
use wervc_object::Object;

pub fn is_builtin(name: &Expression) -> bool {
    if let Expression::Ident(Ident { name, .. }) = name {
        return matches!(name.as_str(), "print" | "println");
    }

    false
}

pub fn call_builtin(name: &Expression, args: &[Object]) -> Option<Object> {
    if let Expression::Ident(Ident { name, .. }) = name {
        return match name.as_str() {
            "print" => Some(builtin_print(args)),
            "println" => Some(builtin_println(args)),
            _ => None,
        };
    }

    None
}

pub fn builtin_print(args: &[Object]) -> Object {
    let prompt = args
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    print!("{prompt}");

    Object::Unit
}

pub fn builtin_println(args: &[Object]) -> Object {
    let prompt = args
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{prompt}");

    Object::Unit
}
