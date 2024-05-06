pub fn verse(n: u32) -> String {
    let mut result = String::new();
    for i in (0..=n).rev() {
        if i > 2 {
            let song_verse = format!(
                    "{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",
                    i,
                    i,
                    i - 1
                );
            result = song_verse;
        } else if i == 2 {
            let song_verse_two = format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n").to_string();
            result = song_verse_two;
        } else if i == 1 {
            let song_verse_one = format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n").to_string();
            result = song_verse_one;
        } else {
            let end_of_song_verse: String = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
            result = end_of_song_verse;
        }
        break;
    }
    result
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    for i in (end..=start).rev() {
        let verse_text = verse(i);
        result.push_str(&verse_text);
        if i != end {
            result.push_str("\n");
        }
    }
    result
}

fn main() {
    // verse(0);
    sing(8, 6);
}

// Another example that does the same crap:
// pub fn verse(n: u32) -> String {
//     match n {
//         0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
//         1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
//         2 => String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
//         _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
//     }
// }
// pub fn sing(start: u32, end: u32) -> String {
//     (end..=start)
//         .rev()
//         .map(verse)
//         .collect::<Vec<String>>()
//         .join("\n")
// }
