use crate::lexer::{Token, TokenKind};
use crate::expr::Expression;

// ============================================================================
// 实现一：Pratt 解析（自上而下算符优先解析）
// ============================================================================

/// PrattParser，使用Pratt解析算法解析Token流
pub struct PrattParser {
    /// Token流
    tokens: Vec<Token>,
    /// 当前指向的Token索引
    current: usize,
}

impl PrattParser {
    /// 创建一个新的PrattParser实例
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

// ============================================================================
// 实现二：递归下降解析（基于 BNF 文法）
// ============================================================================
//
// 表达式文法（每层优先级对应一个非终结符，while 循环保证左结合）：
//
// ```bnf
// expression := term ( ("+" | "-") term )*
// term       := factor ( ("*" | "/") factor )*
// factor     := unary
// unary      := "-" unary | primary
// primary    := NUMBER | FLOAT | BOOLEAN | STRING | "(" expression ")"
// ```
//
// 每条 BNF 规则直接映射为一个解析方法。

/// RecursiveDescentParser，使用递归下降算法解析Token流
pub struct RecursiveDescentParser {
    /// Token流
    tokens: Vec<Token>,
    /// 当前指向的Token索引
    current: usize,
}

impl RecursiveDescentParser {
    /// 创建一个新的RecursiveDescentParser实例
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    /// 查看当前Token，不消费它
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// 消费当前Token并前进
    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current);
        if token.is_some() {
            self.current += 1;
        }
        token
    }

    /// 检查当前Token的kind是否与期望的一致（不消费）
    fn check(&self, expected: &TokenKind) -> bool {
        match self.peek() {
            Some(token) => std::mem::discriminant(&token.kind)
                == std::mem::discriminant(expected),
            None => false,
        }
    }

    /// 如果当前Token匹配给定kind之一，则消费并返回它；否则返回None
    fn match_kind(&mut self, kinds: &[TokenKind]) -> Option<Token> {
        if kinds.iter().any(|k| self.check(k)) {
            Some(self.advance().unwrap().clone())
        } else {
            None
        }
    }

    /// 断言当前Token为期望的kind，消费并返回它；否则panic
    fn expect(&mut self, expected: &TokenKind, msg: &str) -> Token {
        match self.advance() {
            Some(token) if std::mem::discriminant(&token.kind)
                == std::mem::discriminant(expected) =>
            {
                token.clone()
            }
            other => panic!("{}，实际找到：{:?}", msg, other),
        }
    }

    /// expression := term ( ("+" | "-") term )*
    ///
    /// 解析加减运算。用while循环持续消费 +/-，每次把已解析的左半部分
    /// 包进新的Binary节点，从而实现左结合。
    fn parse_expression(&mut self) -> Expression {
        let mut left = self.parse_term();

        while let Some(op) = self.match_kind(&[TokenKind::Plus, TokenKind::Minus]) {
            let right = self.parse_term();
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    /// term := factor ( ("*" | "/") factor )*
    ///
    /// 解析乘除运算，优先级高于加减。
    fn parse_term(&mut self) -> Expression {
        let mut left = self.parse_factor();

        while let Some(op) = self.match_kind(&[TokenKind::Mul, TokenKind::Div]) {
            let right = self.parse_factor();
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    /// factor := unary
    fn parse_factor(&mut self) -> Expression {
        self.parse_unary()
    }

    /// unary := "-" unary | primary
    ///
    /// 解析一元负号（右结合，递归调用自身），如 `-(3+4)`。
    /// 注意：lexer 会把紧贴数字的负号折叠进字面量（如 `-5` 直接是 Number(-5)），
    /// 因此这里主要处理负号作用于括号表达式等情况。
    fn parse_unary(&mut self) -> Expression {
        if self.check(&TokenKind::Minus) {
            let op = self.advance().unwrap().clone();
            let right = self.parse_unary();
            Expression::Unary {
                op,
                right: Box::new(right),
            }
        } else {
            self.parse_primary()
        }
    }

    /// primary := NUMBER | FLOAT | BOOLEAN | STRING | "(" expression ")"
    ///
    /// 解析原子表达式：字面量（数字/布尔/字符串）或括号分组。
    fn parse_primary(&mut self) -> Expression {
        let token = self
            .advance()
            .expect("意外的文件结束：期望一个字面量或左括号");

        match &token.kind {
            TokenKind::Number(value) => Expression::IntLiteral(*value),
            TokenKind::Float(value) => Expression::FloatLiteral(*value),
            TokenKind::Boolean(value) => Expression::BooleanLiteral(*value),
            TokenKind::String(value) => Expression::StringLiteral(value.clone()),
            TokenKind::LeftParen => {
                let left_paren = token.clone();
                // 括号内部是一个完整的子表达式，重新从最低优先级开始解析
                let inner = self.parse_expression();
                let right_paren =
                    self.expect(&TokenKind::RightParen, "期望右括号 `)`");
                Expression::Grouping {
                    left_paren,
                    expr: Box::new(inner),
                    right_paren,
                }
            }
            _ => panic!("期望一个字面量或左括号，找到：{:?}", token),
        }
    }

    /// 解析整个Token流的入口点
    ///
    /// ### Panics
    /// 当表达式解析完后仍有残留Token时panic
    pub fn parse(&mut self) -> Expression {
        let expr = self.parse_expression();

        if let Some(rest) = self.peek() {
            panic!("表达式结束后存在多余Token：{:?}", rest);
        }

        expr
    }
}
