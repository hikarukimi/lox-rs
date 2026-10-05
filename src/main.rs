mod lexer;
mod expr;
mod parser;

use lexer::tokenization;
use expr::calculate;
use parser::{PrattParser, RecursiveDescentParser};

fn main() {
    let cases = [
        "(1.5+2.5)+3.0", 
        "1+2*3",         // 乘除优先于加减 → 7
        "(1+2)*3",       // 括号改变优先级 → 9
        "10-4-3",        // 左结合 → 3
        "2*(3+4)/7",     // 混合嵌套 → 2
    ];

    for input in cases {
        println!("输入表达式：{}", input);

        // 1. 词法分析
        let tokens = tokenization(input);
        println!("  Token流：{:?}", tokens);

        // 2a. Pratt 解析
        let mut pratt = PrattParser::new(tokens.clone());
        let pratt_expr = pratt.parse();
        let pratt_result = calculate(pratt_expr);

        // 2b. 递归下降解析
        let mut rd = RecursiveDescentParser::new(tokens);
        let rd_expr = rd.parse();
        let rd_result = calculate(rd_expr);

        // 3. 对照两种实现的结果
        println!("  Pratt      结果：{}", pratt_result);
        println!("  递归下降   结果：{}", rd_result);
        assert_eq!(
            pratt_result, rd_result,
            "两种实现对 `{}` 的求值结果不一致",
            input
        );
        println!("  结果一致 ✓\n");
    }
}
