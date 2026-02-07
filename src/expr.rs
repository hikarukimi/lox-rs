use crate::lexer::{Token, TokenKind};

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

/// ### 参数
/// * `expr` - 要计算的表达式
///
/// ### 返回
/// 计算结果的浮点数值
///
/// ### Panics
/// 当遇到除以零或无效的操作符时会发生panic
pub fn calculate(expr: Expression) -> f64 {
    match expr {
        // 1. 处理二元运算表达式
        // 使用字段名解构：{ left, op, right }
        Expression::Binary { left, op, right } => {
            // 递归求值左右子表达式
            let left_val = calculate(*left);
            let right_val = calculate(*right);

            // 匹配操作符Token并执行计算
            match &op.kind {
                TokenKind::Plus => left_val + right_val,
                TokenKind::Minus => left_val - right_val,
                TokenKind::Mul => left_val * right_val,
                TokenKind::Div => {
                    if right_val == 0.0 {
                        panic!("除数不能为零！");
                    }
                    left_val / right_val
                }
                // 这里应该只会遇到算术运算符
                _ => panic!("二元表达式中的无效操作符：{:?}", op),
            }
        }

        Expression::Unary { op, right } => {
            todo!("处理一元表达式")
        }

        Expression::Grouping{left_paren, expr, right_paren} => {
            calculate(*expr)
        }

        // 2. 处理整数字面量
        Expression::IntLiteral(num) => {
            // 转换整数为浮点数
            num as f64
        }

        // 3. 处理浮点数字面量
        Expression::FloatLiteral(num) => {
            // 浮点数是最终的数值
            num
        }

    }
}
