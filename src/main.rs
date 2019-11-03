extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

struct NeedleMatch(u32, u32);

static NEEDLE: &str = "|";
static A: u32 = 'a' as u32;

fn index_if_needle((char_ref, character): (usize, &str)) -> Option<u32> {
    if character == NEEDLE {
        Some(char_ref as u32)
    } else {
        None
    }
}

fn search_line((line_ref, line): (usize, &str)) -> Option<NeedleMatch> {
    let needle: Option<u32> = line
        .graphemes(true)
        .enumerate()
        .find_map(index_if_needle);

    match needle {
        Some(index) => Some(NeedleMatch(line_ref as u32, index)),
        None => None,
    }
}

fn integer_to_character_string(i: u32) -> String {
    match std::char::from_u32(i + A) {
        Some(c) => c,
        None => '?', // Something has gone horribly wrong
    }.to_string()
}

fn get_column_characters(character_index: u32) -> String {
    let place_value: u32 = character_index / 26;
    let digit = integer_to_character_string(character_index % 26);

    if place_value > 0 {
        // -1 necessary as our column counting system doesn't have a '0'
        get_column_characters(place_value - 1) + &digit
    } else {
        digit
    }
}

fn print_match(needle_match: NeedleMatch) {
    let NeedleMatch(line, character) = needle_match;

    println!("Result: {}{}",
        get_column_characters(character).to_uppercase(),
        line + 1, // 1 indexing
    );
}

fn search_haystack(haystack: &str) -> Option<NeedleMatch> {
    haystack.lines()
        .enumerate()
        .find_map(search_line)
}

fn print_haystack_search(haystack: &str) {
    println!("\r\nSearch haystack:\r\n{}", haystack);
    match search_haystack(haystack) {
        Some(needle_match) => print_match(needle_match),
        None => println!("No match found!"),
    }
}

fn main() {
    print_haystack_search("######\r\n###|##\r\n######\r\n");
    print_haystack_search("###\r\n##|\r\n###");
    print_haystack_search("############################\r\n###########################|");
    print_haystack_search("|####\r\n#####\r\n#####");
}