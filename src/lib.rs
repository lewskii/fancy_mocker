mod tests;

fn start_uppercase(text: &str) -> bool {
    let ambiguous_char: &[_] = &['i', 'I', 'l', 'L'];

    match text.find(ambiguous_char) {
        Some(index) => {
            match text.chars().nth(index).unwrap() {
                'i' | 'I' => index % 2 != 0,
                _ => index % 2 == 0
            }
        },
        None => false
    }
}

fn should_be_uppercase(c: char) -> Option<bool> {
    match c {
        'i' | 'I' => Some(false),
        'l' | 'L' => Some(true),
        _ => None
    }
}

pub fn mock(text: &str) -> String {
    

    let mut uppercase = start_uppercase(text);

    let mut result = String::with_capacity(text.len());

    for c in text.chars() {
        if !c.is_ascii_alphabetic() {
            result.push(c);
        } else if should_be_uppercase(c).unwrap_or(uppercase) {
            result.push(c.to_ascii_uppercase());
            uppercase = false;
        } else {
            result.push(c.to_ascii_lowercase());
            uppercase = true;
        }
    }

    result
}