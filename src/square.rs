pub fn square_to_coordinate(s: &str) -> Option<(usize, usize)> {
    let mut chars = s.chars();
    let file = chars.next().unwrap() as u8 - b'a'; // subtract by ASCII, 'a'...'h' → 0...7
    let rank = chars.next().unwrap() as u8 - b'1';
    Some((file as usize, rank as usize))
}

pub fn coordinate_to_square((x, y): &(usize, usize)) -> String {
    format!("{}{}", (b'a' + *x as u8) as char, y + 1)
}
