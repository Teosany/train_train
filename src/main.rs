// use std::io;

fn main() {
    // let espaces = "      ";
    // let espaces = espaces.len();
    // println!("{espaces}");

    // let u: u32 = 10; 53
    // let i: f32 = -10.1;
    // let ama: usize = 9007199254740992 * 2 * 2 *2 *2*2*2*2*2*2*2*2;
    // let ama: usize = 0o77;

    // println!("{ama}");

    // const SIZE: usize = 9;
    // let tab: [i32; SIZE] = [10, 32, 12, 43, 52, 53, 83, 2, 1];
    //
    // let mut min = tab[0];
    //
    // for i in 1..SIZE {
    //     if min > tab[i] {
    //         min = tab[i];
    //     }
    // }
    // println!("{min}");

    // let tup= (500, 6.4, 1, 'C', "Haha");
    // let tup: (i32, f64, u8, char, &str) = (500, 6.4, 1, 'C', "Haha");
    // println!("{}", tup.4);
    //
    // let (x, y, z, a, b) = tup;
    //
    // println!("La valeur de y est : {}", y);

    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    // let premier = a[0];
    // let second = a[1];

    // println!("Veuillez entrer un indice de tableau.");
    //
    // let mut indice = String::new();
    //
    // io::stdin()
    //     .read_line(&mut indice)
    //     .expect("Échec");
    //
    // let indice: usize = indice
    //     .trim()
    //     .parse()
    //     .expect("L'indice entré n'est pas un nombre");
    //
    // let element = a[indice];
    //
    // println!(
    //     "La valeur de l'élément d'indice {} est : {}", indice, element
    // );

    // assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2, "we are testing addithion with {} and {}", find_smallest_int(&[34, 15, 88, 2]), 2);

    afficher_mesure_avec_unite(5, 'h');

}

fn afficher_mesure_avec_unite(valeur: i32, unite: char) {
    println!("La mesure est : {}{}", valeur, unite);
}

// fn find_smallest_int(arr: &[i32]) -> i32 {
    // let mut min = arr[0];
    // let max = 10;
    //
    // for i in 0..arr.len() {
    //     if min > arr[i] {
    //         min = arr[i];
    //     }
    // }
    // min
    // *arr.iter().min().unwrap()
    // arr.iter().min().unwrap().clone()
// }