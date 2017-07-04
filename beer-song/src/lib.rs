pub fn verse(beers: i32) -> String {
    match beers {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        n if n > 2 && n <= 99 =>
            format!(
                "{n} bottles of beer on the wall, {n} bottles of beer.\n\
                 Take one down and pass it around, {n_1} bottles of beer on the wall.\n",
                n=n,
                n_1=n - 1),
        _ =>
            panic!(),
        }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..start + 1).rev().map(|n| verse(n)).collect::<Vec<_>>().join("\n")
}
