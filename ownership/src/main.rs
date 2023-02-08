use std::str::FromStr;

fn eval_rpn(expression: Vec<&str>) -> i32 {
    let mut stack = Vec::new();

    for token in expression {
        match token {
            "+" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left + right);
            },
            "-" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left - right);
            },
            "*" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left * right);
            },
            "/" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left / right);
            },
            _ => {
                let num = i32::from_str(token).unwrap();
                stack.push(num);
            },
        }
    }

    stack.pop().unwrap()
}

fn main() {
    let expression = vec!["2", "1", "+", "3", "*", "9", "*", "3", "/", "2", "-"];
    let result = eval_rpn(expression);
    println!("Result: {}", result);
    // slice 类型
    let s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    // s.clear(); // 这清空了字符串，使其等于 ""
    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
    let hello = &s[..5];
    let world: &str = &s[5..];
    println!("{}, {}, {}", hello, world, word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
