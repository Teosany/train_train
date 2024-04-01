fn main () {
    let mut s = String::from("he llo world");

    let world = premier_mot(&s);

    // s.clear();

    println!("{world}");

    s.clear();
    // println!("{s}");
    println!("{world}");
    // println!("{s}");

    // let s = String::from("hello world");

    // let hello = &s[..5];
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // let world = &s[6..];
    // let world = &s[..];
    // println!("{world}")
}

// fn second_mot(s: &String) -> &str {
//
// }

fn premier_mot(s: &String) -> &str {
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// fn premier_mot(s: &String) -> usize {
//     let octets = s.as_bytes();
//     println!("{:?}", octets);
//
//     for (i, &element) in octets.iter().enumerate() {
//         println!("{i}, {element}");
//
//         if element == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

//soit en slice
