fn main () {
    // let mut s = String::from("hello world");
    //
    // let mut world = premier_mot(&s);

    // s.clear();

    // println!("{world}");

    // s.clear();
    // println!("{s}");
    // println!("{world}");
    // println!("{s}");

    // let s = String::from("hello world");

    // let hello = &s[..5];
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // let world = &s[6..];
    // let world = &s[..];
    // println!("{world}")

    // let mut s = "sas";

    // s = "df";
    //
    // println!("{s}");

    let ma_string = String::from("hello world");

    //'premier_mot' fonctionne avec les slices de 'String', que ce soit sur une partie ou sur son intégralité
    let mot = premier_mot(&ma_string[0..6]);
    let mot = premier_mot(&ma_string[..]);

    //'premier_mot' fonctionne également sur des références vers des 'String',
    //qui sont équivalentes à des slices de toute la 'String'
    let mot = premier_mot(&ma_string);

    let mon_litteral_de_chaine = "hello world";

    //'premier_mot' fonctionne avec les slices de littéraux de chaîne, qu'elles soient partielles ou intégrales
    let mot = premier_mot(&mon_litteral_de_chaine[0..6]);
    let mot = premier_mot(&mon_litteral_de_chaine[..]);

    //Comme les littéraux de chaîne *sont* dégà des slices de chaînes, cela fonctionne aussi, sans la syntaxe de slice !
    let mot = premier_mot(mon_litteral_de_chaine);

    //les autres slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

// fn second_mot(s: &String) -> &str {
//
// }

fn premier_mot(s: &str) -> &str {
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
