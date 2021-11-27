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

/// Returns a String with the letters changed to alternate
/// between uppercase and lowercase.
/// 
/// All instances of the letters L and i are set to uppercase
/// and lowercase, respectively.
/// The case of the first letter in the input is determined based on
/// where either aforementioned letter first appears,
/// defaulting to lowercase if neither appears in the text.
/// 
/// The capitalisation in the input is not preserved, although
/// all non-ascii and non-alphabetic characters are preserved as-is.
/// 
/// # Examples
/// 
/// ```
/// let hello = "Hello, world!";
/// let ironic_hello = fancy_mocker::mock(hello);
/// 
/// assert_eq!(ironic_hello, "HeLLo, WoRLd!");
/// ```
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