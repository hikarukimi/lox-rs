mod lexer;
mod expr;
mod parser;

use lexer::tokenization;
use expr::{calculate};
use parser::Parser;

fn main() {
    // 演示：解析和计算 (1.5+2.5)+3.0 的结果
    let input = "(1.5+2.5)+3.0";
    println!("📝 输入表达式：{}", input);

    // 1. 词法分析（Tokenization）：将字符串转换为Token流
    let tokens = tokenization(input);
    println!("🔤 Token流：{:?}", tokens);

    // 2. 语法分析（Parsing）：使用Pratt算法解析Token流为表达式树
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    println!("🌳 表达式树：{:?}", expr);

    // 3. 计算（Evaluation）：递归计算表达式树的结果
    let result = calculate(expr);
    println!("✅ 计算结果：{}", result);
    println!("\n(1.5+2.5)+3.0 = {}", result);
}