fn longest_common_suffix<'a>(s1: &'a str, s2: &'a str) -> &'a str {
   let mut start = s1.len();

    let mut it1 = s1.char_indices().rev();
    let mut it2 = s2.chars().rev();

    while let (Some((idx, c1)), Some(c2)) = (it1.next(), it2.next()) {
        if c1 == c2 {
            start = idx;
        } else {
            break;
        }
    }

    &s1[start..]
}

/*//Alternative implementation using iterators for ASCI
fn longest_common_suffix<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    //s1 = switly
    //s2 = happily
    let mut common_length = 0;
    let mut chars1 = s1.chars().rev();//become [y,l,t,i,w,s]
    let mut chars2 = s2.chars().rev();// become [y,l,i,p,p,a,h]

    //Iterate over characters in reverse to compare suffixes
    loop {
        match (chars1.next(), chars2.next()) {
            //(y,y) +1 ,(l,l) + 1 ,(t,i),(i,p),(w,p),(s,a),(None,h)
            (Some(c1), Some(c2)) if c1 == c2 => common_length += 1,// y == y so +1 until reach index 2
            _ => break,// on the last I'm here (None,h)
        }
    }

    // Return the common suffix if it exists, otherwise return an empty slice
    if common_length > 0 {
        //6-2 4 until the end swift => ly
        &s1[s1.len() - common_length..]
    } else {
        ""//no common lenght so return empty
    }
}*/

pub fn exercise_1() {
    let string1 = String::from("swiftly");
    let string2 = String::from("happily");
    println!("{}", longest_common_suffix(&string1, &string2)); // Expected: "ly"
}
