use crate::lexer::{Token, TokenKind};
use crate::expr::Expression;

/// Parser结构体，用于使用Pratt解析算法解析Token流
pub struct Parser {
    /// Token流
    tokens: Vec<Token>,
    /// 当前指向的Token索引
    current: usize,
}

impl Parser {
    /// 创建一个新的Parser实例
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    /// 获取当前Token的引用，不消费它
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// 返回当前Token的引用，然后指针前进到下一个位置
    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current);
        if token.is_some() {
            self.current += 1;
        }
        token
    }

    /// Pratt解析算法的核心函数。使用给定的结合力(rbp)解析表达式。
    ///
    /// ### 参数
    /// * `rbp` - 右结合力(Right Binding Power)，决定解析的优先级
    ///
    /// ### 返回
    /// 返回解析得到的表达式树
    ///
    /// ### Panics
    /// 当遇到意外的文件结束或无效的Token时会发生panic
    fn parse_expression(&mut self, rbp: u32) -> Expression {
        // 1. **Nud (前缀记号法) / 前缀解析**
        // 获取当前Token并将指针前进
        let token = self.advance().expect("意外的文件结束！");

        // 如果是数字或浮点数，这是最简单的表达式：字面量
        let mut left = match &token.kind {
            TokenKind::Number(value) => Expression::IntLiteral(*value),
            TokenKind::Float(value) => Expression::FloatLiteral(*value),

            // 处理分组（前缀`(`）
            TokenKind::LeftParen => {
                // 用最低的RBP(0)解析括号内的表达式
                let expr = self.parse_expression(0);

                // 期望右括号
                match self.advance() {
                    Some(token) if matches!(token.kind, TokenKind::RightParen) => expr,
                    _ => panic!("期望右括号`)`)但找不到"),
                }
            }

            _ => panic!("期望一个数字或左括号，找到：{:?}", token),
        };

        // 2. **Lbp (中缀记号法) / 中缀解析循环**
        // 只要下一个Token的结合力大于当前rbp就继续解析
        loop {
            // 查看下一个Token以检查其结合力(LBP)
            let next_token = self.current();
            let next_rbp = next_token.map_or(0, |t| t.get_binding_power());

            // 如果下一个操作符的LBP不大于当前rbp，停止循环
            if next_rbp <= rbp {
                break;
            }

            // 下一个Token是一个中缀操作符，消费它
            let op = self.advance().unwrap().clone();

            // **操作符优先级和结合性：**
            // 对于左结合的操作符（如+、-、*、/），RBP设置为LBP(next_rbp)
            // 这强制右侧只接受优先级严格大于的操作符
            // 例如在`1 + 2 + 3`中：`1+2`先被解析
            let new_rbp = next_rbp;

            // 使用新的结合力递归解析右侧
            let right = self.parse_expression(new_rbp);

            // 创建新的二元表达式，使旧的left成为新表达式的left
            left = Expression::Binary {
                left: Box::new(left),
                op: op.clone(),
                right: Box::new(right),
            };
        }

        left
    }

    /// 解析整个Token流的入口点
    /// 返回解析得到的完整表达式树
    pub fn parse(&mut self) -> Expression {
        // 以最低的结合力(RBP = 0)开始表达式解析
        self.parse_expression(0)
    }
}
