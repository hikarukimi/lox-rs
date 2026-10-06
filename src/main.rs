mod lexer;
mod expr;
mod parser;
mod interpreter;

use std::panic::{catch_unwind, AssertUnwindSafe};

use lexer::tokenization;
use parser::{PrattParser, RecursiveDescentParser};
use interpreter::{Interpreter, Value};

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

        let tokens = tokenization(input);

        let mut pratt = PrattParser::new(tokens.clone());
        let pratt_expr = pratt.parse();

        let mut rd = RecursiveDescentParser::new(tokens);
        let rd_expr = rd.parse();

        let pratt_result = interpreter.interpret(&pratt_expr);
        let rd_result = interpreter.interpret(&rd_expr);

        println!("  Pratt+interpreter：{}", pratt_result);
        println!("  RD+interpreter   ：{}", rd_result);
        assert_eq!(pratt_result, rd_result);
        println!("  结果一致 ✓\n");
    }

    // 一元负号、布尔、字符串用例：只有递归下降 parser 支持
    let rd_cases: &[(&str, Value)] = &[
        ("-(3+4)", Value::Number(-7.0)),
        ("2*-(3+4)", Value::Number(-14.0)),
        ("true", Value::Boolean(true)),
        ("false", Value::Boolean(false)),
        ("\"hello\"", Value::Str("hello".to_string())),
        ("\"value: \" + 42", Value::Str("value: 42".to_string())),
        ("\"1\" + 2 * 3", Value::Str("16".to_string())), // * 优先级更高
        ("\"x\" + true", Value::Str("xtrue".to_string())),
        ("1.5 + \"!\"", Value::Str("1.5!".to_string())),
    ];

    for (input, expected) in rd_cases {
        println!("输入表达式：{}", input);
        let tokens = tokenization(input);
        let mut rd = RecursiveDescentParser::new(tokens);
        let rd_expr = rd.parse();
        let result = interpreter.interpret(&rd_expr);
        println!("  interpreter 结果：{} ({:?})", result, result);
        assert_eq!(&result, expected);
        println!("  符合预期 ✓\n");
    }

    // 非法类型组合：应抛出 RuntimeException
    // 静默 panic hook，让 catch_unwind 的演示输出保持干净
    std::panic::set_hook(Box::new(|_| {}));

    let error_cases = [
        "\"a\" - 1",   // 字符串不能参与减法
        "true + 1",    // 布尔不能参与加法（两侧都不是字符串）
        "2 * false",   // 布尔不能参与乘法
        "-true",       // 一元负号不能作用于布尔
        "1 / (2 - 2)", // 除零
    ];

    for input in error_cases {
        print!("输入表达式：{} → ", input);
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            let tokens = tokenization(input);
            let mut rd = RecursiveDescentParser::new(tokens);
            let rd_expr = rd.parse();
            interpreter.interpret(&rd_expr)
        }));
        match outcome {
            Ok(value) => println!("意外成功：{}", value),
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<&str>()
                    .map(|s| s.to_string())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "未知错误".to_string());
                println!("{} ✓", msg);
            }
        }
    }
}
