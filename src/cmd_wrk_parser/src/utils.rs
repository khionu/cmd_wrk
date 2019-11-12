// Source: http://jkorpela.fi/chars/spaces.html
pub const UTF8_WHITESPACE_CHARS: &'static [char] = &[
    '\u{0020}', '\u{00A0}', '\u{1680}', '\u{180E}', '\u{2000}',
    '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}',
    '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}',
    '\u{200B}', '\u{202F}', '\u{205F}', '\u{3000}', '\u{FEFF}',
];

pub fn take_until_whitespace(src: &str) -> Option<&str> {
    match src.find(UTF8_WHITESPACE_CHARS) {
        Some(0) => None,
        Some(x) => Some(&src[..x]),
        None => Some(src),
    }
}

pub fn take_until_close_quote(src: &str) -> Option<&str> {
    let quote: char = match src.chars().nth(0)? {
        '"' | '\'' => src.chars().nth(0)?,
        _ => return None // It's not a quoted string
    };

    let next = src[1..].find(quote)?;

    Some(&src[1..next])
}
