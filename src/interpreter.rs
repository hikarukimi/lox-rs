use std::fmt;

use crate::expr::Expression;
use crate::lexer::{Token, TokenKind};

/// 解释器运行时值：所有 AST 节点求值后都产生一个 Value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// 数字（整数与浮点数统一用 f64 表示）
    Number(f64),
    /// 布尔值
    Boolean(bool),
    /// 字符串
    Str(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // f64 的 Display 会把 7.0 打印为 "7"
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
        }
    }
}

/// Tree-walking 解释器
///
/// 直接在表达式 AST 上做递归遍历求值，遍历顺序类似二叉树的中序遍历：
/// * 字面量：直接返回对应类型的值
/// * 分组(Grouping)：递归遍历内部表达式
/// * 一元(Unary)：先遍历右操作数，再应用操作符（相当于"根 → 右"）
/// * 二元(Binary)：按 **左子树 → 操作符 → 右子树** 的中序顺序遍历
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    /// 解释执行的入口：遍历整棵表达式树并返回结果
    pub fn interpret(&self, expr: &Expression) -> Value {
        self.eval(expr)
    }

    /// 递归遍历单个表达式节点
    fn eval(&self, expr: &Expression) -> Value {
        match expr {
            // 字面量节点：叶子节点，直接返回对应类型的值
            Expression::IntLiteral(num) => Value::Number(*num as f64),
            Expression::FloatLiteral(num) => Value::Number(*num),
            Expression::BooleanLiteral(b) => Value::Boolean(*b),
            Expression::StringLiteral(s) => Value::Str(s.clone()),

            // 分组节点：括号不产生运算，继续向内遍历
            Expression::Grouping { expr: inner, .. } => self.eval(inner),

            // 一元节点：先遍历右操作数，再对结果应用操作符
            Expression::Unary { op, right } => {
                let value = self.eval(right);
                self.eval_unary(op, value)
            }

            // 二元节点：中序遍历 —— 先左子树，再右子树，最后由操作符合并
            Expression::Binary { left, op, right } => {
                let left_val = self.eval(left);
                let right_val = self.eval(right);
                self.eval_binary(op, left_val, right_val)
            }
        }
    }

    /// 应用一元操作符（目前只有一元负号，且只接受数字）
    fn eval_unary(&self, op: &Token, value: Value) -> Value {
        match op.kind {
            TokenKind::Minus => match value {
                Value::Number(n) => Value::Number(-n),
                other => panic!(
                    "RuntimeException: 一元操作符 `-` 要求操作数为数字，但找到 {}",
                    type_name(&other)
                ),
            },
            _ => panic!("RuntimeException: 不支持的一元操作符：{}", op),
        }
    }

    /// 应用二元操作符
    ///
    /// 类型规则：
    /// * `+`：任一操作数为字符串时做字符串拼接（另一操作数按 Display 转成字符串）；
    ///        两个操作数都是数字时做加法
    /// * `-` `*` `/`：只接受数字
    fn eval_binary(&self, op: &Token, left: Value, right: Value) -> Value {
        match op.kind {
            TokenKind::Plus => {
                // 字符串 + 任意类型 → 字符串拼接
                if let Value::Str(s) = &left {
                    return Value::Str(format!("{}{}", s, right));
                }
                if let Value::Str(s) = &right {
                    return Value::Str(format!("{}{}", left, s));
                }
                // 其余组合必须是两个数字
                let left_num = self.as_number(&left, op, "左");
                let right_num = self.as_number(&right, op, "右");
                Value::Number(left_num + right_num)
            }
            TokenKind::Minus | TokenKind::Mul | TokenKind::Div => {
                let left_num = self.as_number(&left, op, "左");
                let right_num = self.as_number(&right, op, "右");
                match op.kind {
                    TokenKind::Minus => Value::Number(left_num - right_num),
                    TokenKind::Mul => Value::Number(left_num * right_num),
                    TokenKind::Div => {
                        if right_num == 0.0 {
                            panic!("RuntimeException: 除数不能为零！");
                        }
                        Value::Number(left_num / right_num)
                    }
                    _ => unreachable!(),
                }
            }
            _ => panic!("RuntimeException: 无效的二元操作符：{}", op),
        }
    }

    /// 要求 value 是数字，否则抛出运行时异常
    fn as_number(&self, value: &Value, op: &Token, side: &str) -> f64 {
        match value {
            Value::Number(n) => *n,
            _ => panic!(
                "RuntimeException: 二元操作符 `{}` 的{}操作数必须是数字，但找到 {}({})",
                op,
                side,
                value,
                type_name(value)
            ),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// 返回值的类型名，用于运行时错误信息
fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Number(_) => "number",
        Value::Boolean(_) => "boolean",
        Value::Str(_) => "string",
    }
}
