fn main() {
    // U+0072 LATIN SMALL LETTER R
    let ascii_char = 'r';
    // U+03BC GREEK SMALL LETTER MU
    let special_char = 'ฮผ';
    // U+0154 LATIN CAPITAL LETTER R WITH ACUTE
    let accented_char = 'ล';
    // U+2622 RADIOACTIVE SIGN
    let emoji_char = '\u{2622}';
    // U+1F468 U+200D U+1F469 U+200D U+1F467 U+200D U+1F467
    let seven_chars_emoji = '๐จโ๐ฉโ๐งโ๐ง'; // Error: char must be one codepoint long
}
