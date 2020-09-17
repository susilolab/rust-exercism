pub fn verse(n: u32) -> String {
    if n == 0 {
        String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    } else if n == 1 {
        String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
    } else if n == 2 {
        String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n")
    } else {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut res = String::new();

    for x in (end..=start).rev() {
        if x == 0 {
            res.push_str("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
        } else if x == 1 {
            res.push_str("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\n");
        } else if x == 2 {
            res.push_str("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n");
        } else {
            res.push_str(
                &format!("{jum} bottles of beer on the wall, {jum} bottles of beer.\nTake one down and pass it around, {sisa} bottles of beer on the wall.", jum=x, sisa=x-1)
            );

            if x == end {
                res.push_str("\n");
            } else {
                res.push_str("\n\n");
            }
        }
    }
    println!("res: {}", res);
    res
}
