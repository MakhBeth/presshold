/// Returns the accented variants for a base character, or `None` if none exist.
/// The variants follow macOS, except that the grave accent comes before the
/// acute one so Italian vowels (à è ì ò ù) are one tap away.
pub fn variants(ch: char) -> Option<&'static [char]> {
    match ch {
        // — lowercase —
        'a' => Some(&['à', 'á', 'ā', 'ă', 'â', 'å', 'ä', 'ã', 'æ']),
        'c' => Some(&['ç', 'ć', 'č']),
        'd' => Some(&['ð', 'ď']),
        'e' => Some(&['è', 'é', 'ē', 'ĕ', 'ê', 'ě', 'ë', 'ę']),
        'g' => Some(&['ğ']),
        'i' => Some(&['ì', 'í', 'ī', 'ĭ', 'î', 'ï', 'ĩ']),
        'l' => Some(&['ł', 'ľ', 'ĺ']),
        'n' => Some(&['ñ', 'ń', 'ň']),
        'o' => Some(&['ò', 'ó', 'ō', 'ŏ', 'ô', 'ö', 'õ', 'ø', 'œ']),
        'r' => Some(&['ř', 'ŕ']),
        's' => Some(&['ß', 'š', 'ś', 'ş']),
        't' => Some(&['þ', 'ť']),
        'u' => Some(&['ù', 'ú', 'ū', 'ŭ', 'û', 'ü', 'ů']),
        'y' => Some(&['ý', 'ÿ']),
        'z' => Some(&['ž', 'ź', 'ż']),
        // — uppercase —
        'A' => Some(&['À', 'Á', 'Ā', 'Ă', 'Â', 'Å', 'Ä', 'Ã', 'Æ']),
        'C' => Some(&['Ç', 'Ć', 'Č']),
        'D' => Some(&['Ð', 'Ď']),
        'E' => Some(&['È', 'É', 'Ē', 'Ĕ', 'Ê', 'Ě', 'Ë', 'Ę']),
        'G' => Some(&['Ğ']),
        'I' => Some(&['Ì', 'Í', 'Ī', 'Ĭ', 'Î', 'Ï', 'Ĩ']),
        'L' => Some(&['Ł', 'Ľ', 'Ĺ']),
        'N' => Some(&['Ñ', 'Ń', 'Ň']),
        'O' => Some(&['Ò', 'Ó', 'Ō', 'Ŏ', 'Ô', 'Ö', 'Õ', 'Ø', 'Œ']),
        'R' => Some(&['Ř', 'Ŕ']),
        'S' => Some(&['Š', 'Ś', 'Ş']),
        'T' => Some(&['Þ', 'Ť']),
        'U' => Some(&['Ù', 'Ú', 'Ū', 'Ŭ', 'Û', 'Ü', 'Ů']),
        'Y' => Some(&['Ý', 'Ÿ']),
        'Z' => Some(&['Ž', 'Ź', 'Ż']),
        // - punctuation -
        '-' => Some(&['\u{2013}', '\u{2014}', '\u{2212}']), // – — −
        _ => None,
    }
}

pub fn has_variants(ch: char) -> bool {
    variants(ch).is_some()
}

#[cfg(test)]
mod tests {
    use super::variants;

    #[test]
    fn vowels_list_grave_accent_before_acute() {
        // Italian-first ordering: one tap of the same letter lands on the grave accent.
        assert_eq!(variants('a').unwrap()[..2], ['à', 'á']);
        assert_eq!(variants('e').unwrap()[..2], ['è', 'é']);
        assert_eq!(variants('i').unwrap()[..2], ['ì', 'í']);
        assert_eq!(variants('o').unwrap()[..2], ['ò', 'ó']);
        assert_eq!(variants('u').unwrap()[..2], ['ù', 'ú']);
    }

    #[test]
    fn uppercase_vowels_list_grave_accent_before_acute() {
        assert_eq!(variants('E').unwrap()[..2], ['È', 'É']);
        assert_eq!(variants('O').unwrap()[..2], ['Ò', 'Ó']);
    }
}
