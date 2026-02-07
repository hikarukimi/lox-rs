use std::fmt;

#[derive(Debug,Clone)]
pub struct Token{
    pub kind: TokenKind,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug,Clone)]
pub enum TokenKind{
    // ========== 括号类 ==========
    LeftParen,      // ( - 左圆括号，用于函数调用、表达式分组
    RightParen,     // ) - 右圆括号
    LeftBrace,      // { - 左花括号，用于代码块、结构体初始化
    RightBrace,     // } - 右花括号
    
    // ========== 算术运算符 ==========
    Mul,            // * - 乘法运算符
    Div,            // / - 除法运算符
    Plus,           // + - 加法运算符
    Minus,          // - - 减法运算符或负号,用于一元表达式，右结合
    Mod,            // % - 取模运算符（求余数）
    
    // ========== 比较运算符 ==========
    Equal,          // == - 相等比较
    NotEqual,       // != - 不等比较
    Less,           // < - 小于
    LessEqual,      // <= - 小于等于
    Greater,        // > - 大于
    GreaterEqual,   // >= - 大于等于
    
    // ========== 赋值运算符 ==========
    Assignment,     // = - 基本赋值
    PlusAssign,     // += - 加法赋值（a += b 等价于 a = a + b）
    MinusAssign,    // -= - 减法赋值

    // ========== 逻辑运算符 ==========
    And,            // && - 逻辑与
    Or,             // || - 逻辑或
    Not,            // ! - 逻辑非，右结合
    
    // ========== 其他符号 ==========
    Semicolon,      // ; - 语句结束符
    
    // ========== 字面量 ==========
    Number(i32),    // 整数字面量，如：42, -10, 0
    Float(f64),     // 浮点数字面量，如：3.14, -0.5, 2.0
    String(String), // 字符串字面量，如："hello", "world"
    Boolean(bool),  // 布尔字面量，true 或 false
    
    // ========== 标识符 ==========
    Identifier(String), // 变量名、函数名等标识符，如：foo, bar_baz, MyStruct
    
    // ========== 关键字 ==========
    If,             // if - 条件判断
    Else,           // else - 条件分支
    While,          // while - 循环
    For,            // for - 迭代循环
    Break,          // break - 跳出循环
    Continue,       // continue - 继续下一次循环
    Return,         // return - 函数返回
    Let,            // let - 变量声明
    Fn,             // fn - 函数定义
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Mul => write!(f, "*"),
            TokenKind::Div => write!(f, "/"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Mod => write!(f, "%"),
            TokenKind::Equal => write!(f, "=="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::Assignment => write!(f, "="),
            TokenKind::PlusAssign => write!(f, "+="),
            TokenKind::MinusAssign => write!(f, "-="),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Number(n) => write!(f, "{}", n),
            TokenKind::Float(n) => write!(f, "{}", n),
            TokenKind::String(s) => write!(f, "\"{}\"", s),
            TokenKind::Boolean(b) => write!(f, "{}", b),
            TokenKind::Identifier(s) => write!(f, "{}", s),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),
            TokenKind::For => write!(f, "for"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::Fn => write!(f, "fn"),
        }
    }
}

impl Token {
    /// 返回中缀操作符的左结合力(LBP)
    /// LBP决定了该Token与左侧表达式的结合强度
    /// 解析表达式时只有下一个token的LBP>当前token的RBP时才会继续解析，否则停止解析
    pub fn get_binding_power(&self) -> u32 {
        match &self.kind {
            TokenKind::Plus | TokenKind::Minus => 1,    // 加减法的优先级为1
            TokenKind::Mul | TokenKind::Div => 2,        // 乘除法的优先级为2
            _=>{
                0                               // 其他Token的优先级为0
            }
        }
    }
}

/// 将标识符字符串转换为相应的Token类型（关键字或标识符）
fn identifier_to_token(ident: &str, line: usize) -> Token {
    let kind = match ident {
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "for" => TokenKind::For,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "return" => TokenKind::Return,
        "let" => TokenKind::Let,
        "fn" => TokenKind::Fn,
        _ => TokenKind::Identifier(ident.to_string()),
    };
    Token { kind, line }
}

pub fn tokenization(input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut input = input.chars().peekable();
    let mut num_str = String::new();
    let mut ident_str = String::new();
    let mut line = 1;
    
    let flush_num = |tokens: &mut Vec<Token>, num_str: &mut String, line: usize| {
        if !num_str.is_empty() {
            // 检查是否为浮点数（包含小数点）
            if num_str.contains('.') {
                tokens.push(Token {
                    kind: TokenKind::Float(num_str.parse().unwrap()),
                    line,
                });
            } else {
                tokens.push(Token {
                    kind: TokenKind::Number(num_str.parse().unwrap()),
                    line,
                });
            }
            num_str.clear();
        }
    };

    let flush_ident = |tokens: &mut Vec<Token>, ident_str: &mut String, line: usize| {
        if !ident_str.is_empty() {
            tokens.push(identifier_to_token(ident_str, line));
            ident_str.clear();
        }
    };
    
    loop {
        match input.next() {
            None => break,
            Some(c) => {
                // 先判断是否是数字的一部分
                if c.is_ascii_digit() {
                    flush_ident(&mut tokens, &mut ident_str, line);
                    num_str.push(c);
                    continue;
                }

                // 判断是否是标识符的一部分（字母或下划线）
                if c.is_ascii_alphabetic() || c == '_' {
                    flush_num(&mut tokens, &mut num_str, line);
                    ident_str.push(c);
                    continue;
                }

                // 处理小数点（浮点数的一部分）
                if c == '.' && !num_str.is_empty() && !num_str.contains('.') {
                    if let Some(&next_char) = input.peek() {
                        if next_char.is_ascii_digit() {
                            // 这是浮点数的小数点
                            num_str.push(c);
                            continue;
                        } else {
                            // 小数点后没有数字，这是语法错误（如 "123."）
                            panic!("语法错误：小数点后必须有数字（第 {} 行）", line);
                        }
                    } else {
                        // 输入末尾的小数点，这也是语法错误（如 "123."）
                        panic!("语法错误：小数点后必须有数字（第 {} 行）", line);
                    }
                }

                // 非数字、非字母的字符，先 flush 数字和标识符
                flush_num(&mut tokens, &mut num_str, line);
                flush_ident(&mut tokens, &mut ident_str, line);

                // 然后处理具体的操作符和其他字符
                match c {
                    ' ' | '\r' | '\t' => {
                        // 忽略空格、回车符、制表符
                    },
                    '\n' => {
                        line += 1;
                    },
                    '+' => {
                        tokens.push(Token { kind: TokenKind::Plus, line })
                    },
                    '-' => {
                        // 判断是负号还是减号
                        // 如果前一个token是数字、标识符、)、] 等，则为减号
                        // 否则为负号
                        let is_binary_minus = tokens.last().map(|t| {
                            matches!(
                                t.kind,
                                TokenKind::Number(_)     // 数字
                                | TokenKind::Float(_)    // 浮点数
                                | TokenKind::Identifier(_) // 标识符
                                | TokenKind::RightParen   // )
                            )
                        }).unwrap_or(false);
                        
                        if is_binary_minus {
                            // 这是减号（二元运算符）
                            tokens.push(Token { kind: TokenKind::Minus, line })
                        } else {
                            // 这是负号（一元运算符），需要作为前缀处理
                            // 先检查后面是否有数字
                            if let Some(&next_char) = input.peek() {
                                if next_char.is_ascii_digit() {
                                    // 收集负数
                                    num_str.push('-');
                                } else {
                                    // 负号后面没有数字，只生成Minus token
                                    tokens.push(Token { kind: TokenKind::Minus, line })
                                }
                            } else {
                                tokens.push(Token { kind: TokenKind::Minus, line })
                            }
                        }
                    },
                    '*' => {
                        tokens.push(Token { kind: TokenKind::Mul, line })
                    },
                    '/' => {
                        if let Some(&'/') = input.peek() {
                            // 单行注释：跳过直到换行符
                            input.next(); // 消费第二个 '/'
                            while let Some(&c) = input.peek() {
                                if c == '\n' {
                                    break;
                                }
                                input.next();
                            }
                        } else {
                            tokens.push(Token { kind: TokenKind::Div, line })
                        }
                    },
                    '=' => {
                        if let Some(&'=') = input.peek() {
                            input.next();
                            tokens.push(Token { kind: TokenKind::Equal, line })
                        } else {
                            tokens.push(Token { kind: TokenKind::Assignment, line })
                        }
                    },
                    '!' => {
                        if let Some(&'=') = input.peek() {
                            input.next();
                            tokens.push(Token { kind: TokenKind::NotEqual, line })
                        }
                    },
                    '<' => {
                        if let Some(&'=') = input.peek() {
                            input.next();
                            tokens.push(Token { kind: TokenKind::LessEqual, line })
                        } else {
                            tokens.push(Token { kind: TokenKind::Less, line })
                        }
                    },
                    '>' => {
                        if let Some(&'=') = input.peek() {
                            input.next();
                            tokens.push(Token { kind: TokenKind::GreaterEqual, line })
                        } else {
                            tokens.push(Token { kind: TokenKind::Greater, line })
                        }
                    },
                    '(' => {
                        tokens.push(Token { kind: TokenKind::LeftParen, line })
                    },
                    ')' => {
                        tokens.push(Token { kind: TokenKind::RightParen, line });
                    },
                    '"' => {
                        // 读取字符串字面量
                        let mut string_content = String::new();
                        loop {
                            match input.next() {
                                None => panic!("字符串未闭合"),
                                Some('\\') => {
                                    // 处理转义字符
                                    match input.next() {
                                        Some('n') => string_content.push('\n'),
                                        Some('t') => string_content.push('\t'),
                                        Some('\\') => string_content.push('\\'),
                                        Some('"') => string_content.push('"'),
                                        Some(c) => string_content.push(c),
                                        None => panic!("转义字符不完整"),
                                    }
                                },
                                Some('"') => break, // 字符串结束
                                Some(c) => string_content.push(c),
                            }
                        }
                        tokens.push(Token {
                            kind: TokenKind::String(string_content),
                            line,
                        });
                    },
                    _ => {
                        // 未知字符，忽略
                    }
                }
            }
        }
    }
    //避免遗漏最后一个数字或标识符
    flush_num(&mut tokens, &mut num_str, line);
    flush_ident(&mut tokens, &mut ident_str, line);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiline_tracking() {
        // 测试：验证词法分析器正确追踪多行输入的行号
        // 预期：不同行的Token应该有不同的line字段值
        let input = "1+2\n3*4";
        let tokens = tokenization(input);
        
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[2].line, 1);
        assert_eq!(tokens[3].line, 2);
        assert_eq!(tokens[5].line, 2);
    }

    #[test]
    fn test_comparison_operators() {
        // 测试：验证比较运算符(== 和 !=)的正确识别
        // 预期："1==2" 应生成 [Number(1), Equal, Number(2)] Token
        let tokens = tokenization("1==2");
        assert_eq!(tokens.len(), 3);
        match &tokens[1].kind {
            TokenKind::Equal => (),
            _ => panic!("Expected Equal token"),
        }

        let tokens = tokenization("1!=2");
        match &tokens[1].kind {
            TokenKind::NotEqual => (),
            _ => panic!("Expected NotEqual token"),
        }
    }

    #[test]
    fn test_less_greater_operators() {
        // 测试：验证大小比较运算符(<= 和 >=)的正确识别
        // 预期："1<=2" 应识别为 LessEqual，"3>=4" 应识别为 GreaterEqual
        let tokens = tokenization("1<=2 3>=4");
        match &tokens[1].kind {
            TokenKind::LessEqual => (),
            _ => panic!("Expected LessEqual token"),
        }
        match &tokens[4].kind {
            TokenKind::GreaterEqual => (),
            _ => panic!("Expected GreaterEqual token"),
        }
    }

    #[test]
    fn test_single_line_comment() {
        // 测试：验证单行注释(//)被正确忽略
        // 预期："1+2 // 这是注释" 应只生成 [Number(1), Plus, Number(2)] 三个Token，注释部分被忽略
        let tokens = tokenization("1+2 // 这是注释");
        assert_eq!(tokens.len(), 3);
        match &tokens[0].kind {
            TokenKind::Number(n) => assert_eq!(*n, 1),
            _ => panic!("Expected Number token"),
        }
        match &tokens[2].kind {
            TokenKind::Number(n) => assert_eq!(*n, 2),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn test_float_basic() {
        // 测试：验证基础浮点数的识别
        // 预期："3.14" 应识别为单个 Float(3.14) Token
        let tokens = tokenization("3.14");
        assert_eq!(tokens.len(), 1);
        match &tokens[0].kind {
            TokenKind::Float(f) => assert_eq!(*f, 3.14),
            _ => panic!("Expected Float token"),
        }
    }

    #[test]
    fn test_float_in_arithmetic() {
        // 测试：验证浮点数在算术表达式中的识别
        // 预期："1.5+2.5" 应生成 [Float(1.5), Plus, Float(2.5)] 三个Token
        let tokens = tokenization("1.5+2.5");
        assert_eq!(tokens.len(), 3);
        match &tokens[0].kind {
            TokenKind::Float(f) => assert_eq!(*f, 1.5),
            _ => panic!("Expected Float(1.5)"),
        }
        match &tokens[2].kind {
            TokenKind::Float(f) => assert_eq!(*f, 2.5),
            _ => panic!("Expected Float(2.5)"),
        }
    }

    #[test]
    fn test_keywords() {
        // 测试：验证关键字的正确识别
        // 预期：if、else、while、for、break、continue、return、let、fn应识别为关键字Token
        let keywords = vec!["if", "else", "while", "for", "break", "continue", "return", "let", "fn"];
        for keyword in keywords {
            let tokens = tokenization(keyword);
            assert_eq!(tokens.len(), 1, "Keyword '{}' should produce one token", keyword);
            // 验证不是Identifier
            match &tokens[0].kind {
                TokenKind::Identifier(_) => panic!("'{}' should be a keyword, not an identifier", keyword),
                _ => {}
            }
        }
    }

    #[test]
    fn test_identifiers() {
        // 测试：验证标识符的正确识别
        // 预期：变量名、函数名等应识别为Identifier Token
        let tokens = tokenization("myVar foo_bar");
        assert_eq!(tokens.len(), 2);
        match &tokens[0].kind {
            TokenKind::Identifier(name) => assert_eq!(name, "myVar"),
            _ => panic!("Expected Identifier token"),
        }
        match &tokens[1].kind {
            TokenKind::Identifier(name) => assert_eq!(name, "foo_bar"),
            _ => panic!("Expected Identifier token"),
        }
    }

    #[test]
    fn test_keywords_in_expression() {
        // 测试：验证关键字在表达式中的识别
        // 预期："if x" 应生成 [If, Identifier("x")] 两个Token
        let tokens = tokenization("if x");
        assert_eq!(tokens.len(), 2);
        match &tokens[0].kind {
            TokenKind::If => (),
            _ => panic!("Expected If token"),
        }
        match &tokens[1].kind {
            TokenKind::Identifier(name) => assert_eq!(name, "x"),
            _ => panic!("Expected Identifier token"),
        }
    }

    #[test]
    fn test_unary_minus() {
        // 测试：验证一元负号的正确识别
        // 预期："-5" 应生成 [Number(-5)] Token，作为负数处理
        let tokens = tokenization("-5");
        assert_eq!(tokens.len(), 1);
        match &tokens[0].kind {
            TokenKind::Number(n) => assert_eq!(*n, -5),
            _ => panic!("Expected Number(-5) token"),
        }
    }

    #[test]
    fn test_binary_minus() {
        // 测试：验证二元减号的正确识别
        // 预期："10 - 5" 应生成 [Number(10), Minus, Number(5)] Token
        let tokens = tokenization("10 - 5");
        assert_eq!(tokens.len(), 3);
        match &tokens[0].kind {
            TokenKind::Number(n) => assert_eq!(*n, 10),
            _ => panic!("Expected Number(10)"),
        }
        match &tokens[1].kind {
            TokenKind::Minus => (),
            _ => panic!("Expected Minus token"),
        }
        match &tokens[2].kind {
            TokenKind::Number(n) => assert_eq!(*n, 5),
            _ => panic!("Expected Number(5)"),
        }
    }

    #[test]
    fn test_unary_minus_after_operator() {
        // 测试：验证操作符后的负号识别为负号
        // 预期："5 + -3" 应生成 [Number(5), Plus, Number(-3)] Token
        let tokens = tokenization("5 + -3");
        assert_eq!(tokens.len(), 3);
        match &tokens[2].kind {
            TokenKind::Number(n) => assert_eq!(*n, -3),
            _ => panic!("Expected Number(-3)"),
        }
    }

    #[test]
    fn test_float_with_invalid_trailing_dot() {
        // 测试：验证 "123." 这种无后续数字的情况应该报错
        // 预期：应该 panic
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            tokenization("123.")
        }));
        assert!(result.is_err(), "Expected panic for invalid float '123.'");
    }

    #[test]
    fn test_valid_negative_float() {
        // 测试：验证负浮点数的正确识别
        // 预期："-3.14" 应生成 [Float(-3.14)] Token
        let tokens = tokenization("-3.14");
        assert_eq!(tokens.len(), 1);
        match &tokens[0].kind {
            TokenKind::Float(f) => assert!((*f - (-3.14)).abs() < 1e-10),
            _ => panic!("Expected Float(-3.14)"),
        }
    }
}