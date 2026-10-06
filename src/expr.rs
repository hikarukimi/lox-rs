use crate::lexer::Token;

/// 表达式枚举，定义了所有可能的表达式类型
#[derive(Debug)]
pub enum Expression {
    /// 整数字面量
    IntLiteral(i32),

    /// 浮点数字面量
    FloatLiteral(f64),

    Grouping{
        left_paren: Token,
        expr:Box<Expression>,
        right_paren: Token,
    },

    /// 一元运算表达式：包含操作符和右操作数
    Unary { op: Token, right: Box<Expression> },

    /// 二元运算表达式：包含左操作数、操作符、右操作数
    Binary {
        /// 左操作数
        left: Box<Expression>,
        /// 操作符Token
        op: Token,
        /// 右操作数
        right: Box<Expression>,
    },
}

impl Expression {
    /// 将一个表达式转化成lisp风格的字符串用于验证AST是否正确
    fn parenthesize(&self) -> String {
        match self {
            Expression::IntLiteral(num) => num.to_string(),
            Expression::FloatLiteral(num) => num.to_string(),
            Expression::Grouping{left_paren, expr, right_paren} => format!("({}{}{})", left_paren, expr.parenthesize(), right_paren),
            Expression::Unary { op, right } => format!("({}{})", op, right.parenthesize()),
            Expression::Binary { left, op, right } => format!("({}{}{})", left.parenthesize(), op, right.parenthesize()),
        }
     }
}
