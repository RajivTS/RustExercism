pub fn verse(n: u32) -> String {
    match n {
        2..=99 => {
            format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n", n, n, n - 1, if n > 2 { "s" } else { "" })
        }
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_owned(),
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned(),
        _ => unimplemented!("Verse {} is not supported", n)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .reduce(|acc, elem| format!("{}\n{}", acc, elem))
        .unwrap_or_else(String::new)
}
