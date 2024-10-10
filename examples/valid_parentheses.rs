static VALID_CHARS: [char; 6] = ['(', ')', '[', ']', '{', '}'];

fn main() {
    let s1 = "{[()]}";
    println!("{s1} {}", is_valid_sequence(s1));

    println!("Hello")
}

fn is_valid_sequence(val: &str) -> bool {
    if val.chars()
        .filter(|c| VALID_CHARS.contains(c))
        .count() != val.chars().count() {
        panic!("Invalid chars");
    }
    let mut stack: Vec<char> = vec![];
    for c in val.chars() {
        if is_opening(c) {
            stack.push(c);
        } else {
            if stack.is_empty() {
                return false;
            }
            let last = *stack.last().unwrap();
            if is_opening(last) && c == closing(last) {
                stack.pop();
            } else {
                return false;
            }
        }
    }
    stack.is_empty()
}

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => panic!("Invalid char {}", c),
    }
}

fn is_opening(c: char) -> bool {
    match c {
        '(' | '[' | '{' => true,
        _ => false,
    }
}