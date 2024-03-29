fn main () {
    // let s1 = String::from("hello");
    //
    // let long = calculer_taille(&s1);
    //
    // println!("La taille de '{}' est {}.", s1, long);

    // let mut s = String::from("hello");
    //
    // changer(&mut s);

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}", r2);

    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;
    //
    // let r1 = &s;
    // let r2 = &s;
    // println!("{r1} {r2}");
    // ®
    // let r3 = &mut s;
    // println!("{r3}");

    // let reference_vers_rien = pendouille();

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);
    let x_abs2 = x.abs();
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);
    let s_len2 = s.len();
    assert_eq!(s_len1, s_len2);
}

// fn pendouille() -> String {
//     let s = String::from("hello");
//
//     s
// }

// fn changer(texte: &mut String) {
//     texte.push_str(", world");
// }

// fn calculer_taille(s: &String) -> usize {
//     s.len()
// }