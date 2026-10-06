use crate::expr::Expression;
use crate::lexer::{Token, TokenKind};

/// Tree-walking 解释器
///
/// 直接在表达式 AST 上做递归遍历求值，遍历顺序类似二叉树的中序遍历：
/// * 字面量：直接返回数值
/// * 分组(Grouping)：递归遍历内部表达式
/// * 一元(Unary)：先遍历右操作数，再应用操作符（相当于"根 → 右"）
/// * 二元(Binary)：按 **左子树 → 操作符 → 右子树** 的中序顺序遍历
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    /// 解释执行的入口：遍历整棵表达式树并返回结果
    pub fn interpret(&self, expr: &Expression) -> f64 {
        self.eval(expr)
    }

    /// 递归遍历单个表达式节点
    fn eval(&self, expr: &Expression) -> f64 {
        match expr {
            // 字面量节点：叶子节点，直接返回数值
            Expression::IntLiteral(num) => *num as f64,
            Expression::FloatLiteral(num) => *num,

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

    /// 应用一元操作符
    fn eval_unary(&self, op: &Token, value: f64) -> f64 {
        match op.kind {
            TokenKind::Minus => -value,
            _ => panic!("不支持的一元操作符：{}", op),
        }
    }

    /// 应用二元操作符
    fn eval_binary(&self, op: &Token, left: f64, right: f64) -> f64 {
        match op.kind {
            TokenKind::Plus => left + right,
            TokenKind::Minus => left - right,
            TokenKind::Mul => left * right,
            TokenKind::Div => {
                if right == 0.0 {
                    panic!("除数不能为零！");
                }
                left / right
            }
            _ => panic!("二元表达式中的无效操作符：{}", op),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
