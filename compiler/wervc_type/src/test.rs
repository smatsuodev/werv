use crate::{TypedExpression, TypedExpressionKind, TypedNode};
use wervc_ast::{
    ty::Type, BinaryExpr, BinaryExprKind, Boolean, FunctionDefExpr, Ident, Integer, LetExpr,
    Program, Statement, UnaryExpr, UnaryExprKind,
};

#[test]
fn test_type_resolution() {
    let mut inputs = [
        TypedNode::Program(Program {
            statements: vec![Statement::ExprStmt(TypedExpression {
                kind: TypedExpressionKind::Integer(Integer { value: 1 }),
                ty: Type::unknown(),
            })],
        }),
        TypedNode::Program(Program {
            statements: vec![Statement::ExprReturnStmt(TypedExpression {
                kind: TypedExpressionKind::Integer(Integer { value: 1 }),
                ty: Type::unknown(),
            })],
        }),
        TypedNode::Expression(TypedExpression {
            kind: TypedExpressionKind::Integer(Integer { value: 1 }),
            ty: Type::unknown(),
        }),
        TypedNode::Expression(TypedExpression {
            kind: TypedExpressionKind::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(TypedExpression {
                    kind: TypedExpressionKind::Integer(Integer { value: 1 }),
                    ty: Type::unknown(),
                }),
                rhs: Box::new(TypedExpression {
                    kind: TypedExpressionKind::Integer(Integer { value: 1 }),
                    ty: Type::unknown(),
                }),
            }),
            ty: Type::unknown(),
        }),
        TypedNode::Expression(TypedExpression {
            kind: TypedExpressionKind::Boolean(Boolean { value: true }),
            ty: Type::unknown(),
        }),
        TypedNode::Expression(TypedExpression {
            kind: TypedExpressionKind::LetExpr(LetExpr {
                name: Box::new(TypedExpression {
                    kind: TypedExpressionKind::Ident(Ident {
                        name: "x".to_string(),
                        offset: 4,
                    }),
                    ty: Type::unknown(),
                }),
                value: Box::new(TypedExpression {
                    kind: TypedExpressionKind::Integer(Integer { value: 1 }),
                    ty: Type::unknown(),
                }),
                ty: Type::int(),
            }),
            ty: Type::unknown(),
        }),
        TypedNode::Expression(TypedExpression {
            kind: TypedExpressionKind::FunctionDefExpr(FunctionDefExpr {
                name: Box::new(TypedExpression {
                    kind: TypedExpressionKind::Ident(Ident {
                        name: "x".to_string(),
                        offset: 0,
                    }),
                    ty: Type::unknown(),
                }),
                params: vec![],
                return_ty: Type::int(),
                body: Box::new(TypedExpression {
                    kind: TypedExpressionKind::Integer(Integer { value: 1 }),
                    ty: Type::unknown(),
                }),
            }),
            ty: Type::unknown(),
        }),
        TypedNode::Program(Program {
            statements: vec![
                Statement::ExprStmt(TypedExpression {
                    kind: TypedExpressionKind::LetExpr(LetExpr {
                        name: Box::new(TypedExpression {
                            kind: TypedExpressionKind::Ident(Ident {
                                name: "x".to_string(),
                                offset: 0,
                            }),
                            ty: Type::unknown(),
                        }),
                        value: Box::new(TypedExpression {
                            kind: TypedExpressionKind::Integer(Integer { value: 1 }),
                            ty: Type::unknown(),
                        }),
                        ty: Type::int(),
                    }),
                    ty: Type::unknown(),
                }),
                Statement::ExprReturnStmt(TypedExpression {
                    kind: TypedExpressionKind::UnaryExpr(UnaryExpr {
                        kind: UnaryExprKind::Addr,
                        expr: Box::new(TypedExpression {
                            kind: TypedExpressionKind::Ident(Ident {
                                name: "x".to_string(),
                                offset: 0,
                            }),
                            ty: Type::unknown(),
                        }),
                    }),
                    ty: Type::unknown(),
                }),
            ],
        }),
        TypedNode::Program(Program {
            statements: vec![
                Statement::ExprStmt(TypedExpression {
                    kind: TypedExpressionKind::LetExpr(LetExpr {
                        name: Box::new(TypedExpression {
                            kind: TypedExpressionKind::Ident(Ident {
                                name: "x".to_string(),
                                offset: 0,
                            }),
                            ty: Type::unknown(),
                        }),
                        value: Box::new(TypedExpression {
                            kind: TypedExpressionKind::Integer(Integer { value: 1 }),
                            ty: Type::unknown(),
                        }),
                        ty: Type::int(),
                    }),
                    ty: Type::unknown(),
                }),
                Statement::ExprStmt(TypedExpression {
                    kind: TypedExpressionKind::LetExpr(LetExpr {
                        name: Box::new(TypedExpression {
                            kind: TypedExpressionKind::Ident(Ident {
                                name: "y".to_string(),
                                offset: 0,
                            }),
                            ty: Type::unknown(),
                        }),
                        value: Box::new(TypedExpression {
                            kind: TypedExpressionKind::UnaryExpr(UnaryExpr {
                                kind: UnaryExprKind::Addr,
                                expr: Box::new(TypedExpression {
                                    kind: TypedExpressionKind::Ident(Ident {
                                        name: "x".to_string(),
                                        offset: 0,
                                    }),
                                    ty: Type::unknown(),
                                }),
                            }),
                            ty: Type::unknown(),
                        }),
                        ty: Type::pointer_to(Box::new(Type::int())),
                    }),
                    ty: Type::unknown(),
                }),
                Statement::ExprReturnStmt(TypedExpression {
                    kind: TypedExpressionKind::UnaryExpr(UnaryExpr {
                        kind: UnaryExprKind::Deref,
                        expr: Box::new(TypedExpression {
                            kind: TypedExpressionKind::Ident(Ident {
                                name: "y".to_string(),
                                offset: 0,
                            }),
                            ty: Type::unknown(),
                        }),
                    }),
                    ty: Type::unknown(),
                }),
            ],
        }),
    ];
    let expects = [
        Type::never(),
        Type::int(),
        Type::int(),
        Type::int(),
        Type::bool(),
        Type::int(),
        Type::func(vec![], Box::new(Type::int())),
        Type::pointer_to(Box::new(Type::int())),
        Type::int(),
    ];

    for (input, expect) in inputs.iter_mut().zip(expects.iter()) {
        let (actual, _) = input
            .resolve_type()
            .unwrap_or_else(|e| panic!("\nerror: {:?}\ninput: {:#?}", e, input));

        assert_eq!(actual, *expect, "input: {:?}", input);
    }
}
