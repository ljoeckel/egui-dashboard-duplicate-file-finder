const CHARS_TO_REMOVE: [char; 27] = ['\'', '\"', '/', '=', '-', ',', '.', ':', ';', '\n', '\t', '<', '>', '^', '`', '&', '%', '$', '£', '@', '#', '!', '?', '§', '°', '*', '+'];

// Remove multiples of the given character i.e. '  ' -> ' ', Trims char at end
pub fn trim_char(st: &String, c: char) -> String {
    let mut cnt: usize = 0;
    let mut s = String::with_capacity(st.len() + 1);

    for (i, chr) in st.chars().enumerate() {
        if chr == c && (i + 1) == st.len() { // c at the end
            continue;
        } else if chr == c && cnt == 0 { // first c
            cnt += 1
        } else if chr == c && cnt == 1 { // second,third,... c
            continue;
        } else {
            cnt = 0;
        }
        s.push(chr);
    }
    s
}

pub fn remove_klammer(st: &String, c0: char, c1: char) -> String {
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
    s
}

pub fn normalize_string(value: &String) -> String {
    let mut str = value.to_lowercase();
    str = remove_klammer(&str, '(', ')'); // '(live)', ...
    str = remove_klammer(&str, '{', '}');
    trim_char(&str, ' ')
}

pub fn normalize_option(value: Option<&String>) -> String {
    if value.is_none() {
        String::new()
    } else {
        normalize_string(value.unwrap())
    }
}
