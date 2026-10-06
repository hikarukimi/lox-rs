mod lexer;
mod expr;
mod parser;
mod interpreter;

use lexer::tokenization;
use parser::{PrattParser, RecursiveDescentParser};
use interpreter::Interpreter;

fn main() {
    let interpreter = Interpreter::new();

    // 两种 parser 都能解析的用例：统一由 tree-walking interpreter 求值并对照
    let cases = [
        "(1.5+2.5)+3.0", // 原有示例 → 7
        "1+2*3",         // 乘除优先于加减 → 7
        "(1+2)*3",       // 括号改变优先级 → 9
        "10-4-3",        // 左结合 → 3
        "2*(3+4)/7",     // 混合嵌套 → 2
    ];

    for input in cases {
        println!("输入表达式：{}", input);

        // 1. 词法分析
        let tokens = tokenization(input);

        // 2a. Pratt 解析
        let mut pratt = PrattParser::new(tokens.clone());
        let pratt_expr = pratt.parse();

        // 2b. 递归下降解析
        let mut rd = RecursiveDescentParser::new(tokens);
        let rd_expr = rd.parse();

        // 3. 统一由 tree-walking 解释器求值
        let pratt_result = interpreter.interpret(&pratt_expr);
        let rd_result = interpreter.interpret(&rd_expr);

        println!("  Pratt+interpreter：{}", pratt_result);
        println!("  RD+interpreter   ：{}", rd_result);
        assert_eq!(pratt_result, rd_result);
        println!("  结果一致 ✓\n");
    }

    // 一元负号用例：只有递归下降 parser 支持
    // （Pratt parser 未实现 unary 前缀分支）
    let unary_cases = ["-(3+4)", "2*-(3+4)"];
    for input in unary_cases {
        println!("输入表达式：{}", input);
        let tokens = tokenization(input);
        let mut rd = RecursiveDescentParser::new(tokens);
        let rd_expr = rd.parse();
        println!("  AST：{:?}", rd_expr);
        println!("  interpreter 结果：{}", interpreter.interpret(&rd_expr));
    }
}
