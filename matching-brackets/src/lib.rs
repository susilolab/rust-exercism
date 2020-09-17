use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');

    let mut stack: Vec<char> = Vec::new();

    let mut res = true;
    for x in string.chars() {
        if x == '(' || x == '[' || x == '{' {
            stack.push(x);
        } else if x == ')' || x == ']' || x == '}' {
            res = match stack.pop() {
                Some(last) => {
                    match map.get(&last) {
                        Some(&a) => x == a,
                        None => false
                    }
                }
                None => false,
            };
        }
    }

    res && stack.len() == 0
}
