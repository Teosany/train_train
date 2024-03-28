fn main() {
    {
        // let mut s = String::from("Hello");
        //
        // s.push_str(", world");
        //
        // println!("{s}");

        // let x = 5;
        // let y = x;
        // println!("x = {}, y = {}", x, y);
        //
        // let s1 = String::from("hello");
        // let s2 = s1;
        // println!("{s2}");

        // let s2 = s1.clone();
        // println!("s1 = {}, s2 = {}", s1, s2);

        // prendre_possession(s2);

        // let x = 5;

        // creer_copie(x);

        // let s1 = donne_possession();
        //
        // let s2 = String::from("Hello");
        //
        // let s3 = prend_et_rend(s2);
    }

    // fn donne_possession() -> String {
    //     let texte = String::from("yours");
    //     texte
    // }
    //
    // fn prend_et_rend(texte: String) -> String {
    //     texte
    // }

//     fn prendre_possession(texte: String) {
//         println!("{}", texte);
//     }
//
//     fn creer_copie(entier: i32) {
//         println!("{}", entier);
//     }

    // TUPLE
    let s1 = String::from("hello");

    let (s2, taille) = calculer_taille(s1);

    println!("La taille de '{}' est {}.", s2, taille);
}

fn calculer_taille(s: String) -> (String, usize) {
    let taille = s.len();

    (s, taille)
}