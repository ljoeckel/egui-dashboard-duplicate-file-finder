const CHARS_TO_REMOVE: [char; 27] = ['\'', '\"', '/', '=', '-', ',', '.', ':', ';', '\n', '\t', '<', '>', '^', '`', '&', '%', '$', '£', '@', '#', '!', '?', '§', '°', '*', '+'];

// Remove multiples of the given character i.e. '  ' -> ' ', Trims char at end
pub fn trim_char(st: String, c: char) -> String {
    let mut cnt: usize = 0;
    let mut s = String::with_capacity(st.len() + 1);

    for (i, chr) in st.chars().enumerate() {
        if chr == ' ' && (i + 1) == st.len() { // blank at the end
            continue;
        } else if chr == ' ' && cnt == 0 { // first blank
            cnt += 1
        } else if chr == ' ' && cnt == 1 { // second,third,... blank
            continue;
        } else {
            cnt = 0;
        }
        s.push(chr);
    }
    return s;
}

pub fn split_by_chars(s: &mut String, chars: &str) {
    for c in chars.chars() {
        let idx = s.find(c);
        if idx.is_some() {
            s.split_off(idx.unwrap());
        }
    }
}

pub fn remove_klammer(st: String, c0: char, c1: char) -> String {
    let mut ignore: bool = false;
    let mut s = String::new();
    for c in st.chars() {
        if c == c1 {
            ignore = false;
            continue;
        }
        if c == c0 {
            ignore = true;
            continue;
        }
        if ignore { continue; }

        // Remove whitespace
        if CHARS_TO_REMOVE.contains(&c) { continue; };

        s.push(c);
    }
    return s;
}

pub fn normalize_string(value: &String) -> String {
    let mut str = value.to_lowercase();
    str = remove_klammer(str, '(', ')');
    str = remove_klammer(str, '[', ']');
    str = remove_klammer(str, '{', '}');
    return trim_char(str, ' ');
}

pub fn normalize_option(value: Option<&String>) -> String {
    if value.is_none() {
        return String::new();
    } else {
        return normalize_string(value.unwrap());
    }
}
