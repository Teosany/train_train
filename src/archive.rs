// use std::io;
//
// fn main() {
//     let espaces = "      ";
//     let espaces = espaces.len();
//     println!("{espaces}");
//
//     let u: u32 = 10; 53
//     let i: f32 = -10.1;
//     let ama: usize = 9007199254740992 * 2 * 2 *2 *2*2*2*2*2*2*2*2;
//     let ama: usize = 0o77;
//
//     println!("{ama}");
//
//     const SIZE: usize = 9;
//     let tab: [i32; SIZE] = [10, 32, 12, 43, 52, 53, 83, 2, 1];
//
//     let mut min = tab[0];
//
//     for i in 1..SIZE {
//         if min > tab[i] {
//             min = tab[i];
//         }
//     }
//     println!("{min}");
//
//     let tup= (500, 6.4, 1, 'C', "Haha");
//     let tup: (i32, f64, u8, char, &str) = (500, 6.4, 1, 'C', "Haha");
//     println!("{}", tup.4);
//
//     let (x, y, z, a, b) = tup;
//
//     println!("La valeur de y est : {}", y);
//
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//
//     let premier = a[0];
//     let second = a[1];
//
//     println!("Veuillez entrer un indice de tableau.");
//
//     let mut indice = String::new();
//
//     io::stdin()
//         .read_line(&mut indice)
//         .expect("Échec");
//
//     let indice: usize = indice
//         .trim()
//         .parse()
//         .expect("L'indice entré n'est pas un nombre");
//
//     let element = a[indice];
//
//     println!(
//         "La valeur de l'élément d'indice {} est : {}", indice, element
//     );
//
//     assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2, "we are testing addithion with {} and {}", find_smallest_int(&[34, 15, 88, 2]), 2);
//
//     afficher_mesure_avec_unite(5, 'h');
//
//     let y = {
//         let x = 3;
//         x + 1
//     };
//
//     println!("La valeur de y est : {}", y);
//
//     let x = cinq();
//
//     println!("La valeur de x est : {}", x);
//
//     let x = plus_un(5);
//
//     println!("La valeur de x est : {}", x)
//
// //////////////// IF ELSE
//
//     let nombre = 11;
//
//     if nombre % 4 == 0 {
//         println!("Le nombre est divisible par 4");
//     } else if nombre % 3 == 0 {
//         println!("Le nombre est divisible par 3");
//     } else if nombre % 2 == 0 {
//         println!("Le nombre est divisible par 2");
//     } else {
//         println!("Le nombre n'est pas divisible par 4, 3 ou 2");
//     }
//
//     let condition = false;
//     let nombre = if condition {5} else {7};
//
//     if condition {
//         let string = 5; println!("i32 ou char? i32 {}", string)
//     } else {
//         let string = '5'; println!("i32 ou char? char {}", string)
//     };
//
//     println!("La valeur du nombre est : {}", nombre);
//
//
//     //////////////// LOOP
//
//     loop {
//         println!("A nouveau !")
//     }
//
//     let mut compteur = 0;
//     'increment: loop {
//         println!("compteur = {}", compteur);
//         let mut restant = 10;
//
//         loop {
//             println!("restant = {}", restant);
//             if restant == 9 {
//                 break;
//             }
//             if compteur == 2 {
//                 break 'increment;
//             }
//             restant -= 1;
//         }
//
//         compteur += 1;
//     }
//     println!("Fin du compteur = {}", compteur);
//
//     let mut compteur = 0;
//
//     let resultat = loop {
//         compteur += 1;
//
//         if compteur == 10 {
//             break compteur * 2;
//         }
//     };
//
//     println!("Le resultat est {}", resultat);
//
//     let mut nombre = 3;
//
//     while nombre != 0 {
//         println!("{} !", nombre);
//
//         nombre -= 1;
//     }
//
//     println!("DÉCOLLAGE !!!");
//
//     let a = [10, 20, 30, 40, 50];
//     let mut indice = 0;
//
//     while indice < 5 {
//         println!("La valeur est : {}", a[indice]);
//
//         indice += 1;
//     }
//
//     for element in a {
//         println!("La valeur est : {}", element);
//     } // Lorsque nous exécutons ce code, nous obtenons les mêmes messages que dans l'encart 3-4.
//     Mais ce qui est plus important, c'est que nous avons amélioré la sécurité de notre code et
//     éliminé le risque de bogues qui pourraient survenir si on dépassait la fin du tableau, ou si
//     on n'allait pas jusqu'au bout et qu'on ratait quelques éléments.
//
//     for i in (1..4).rev() {
//         println!("{} !", i);
//     }
//     println!("DÉCOLLAGE !!!")
//
//
//     check_size(0);
//     read_command_line();
//
//     let tab = read_command_line();
//     print_tab(tab);
//
//     let tab = read_command_line();
//     println!("Among the numbers in the list:");
//     print_tab(tab);
//     let min = find_min(tab);
//     println!("The minimal value is: {min}");
// }
//
// const SIZE: usize = 9;
//
// fn check_size(size: usize) {
//     if size == 0 {
//         panic!("Size is of tab = 0.");
//     }
// }
//
// fn read_command_line() -> [i32; SIZE] {
//     [10, 32, 12, 43, 52, 53, 83, 2, 9]
//     équivalent à return [10, 32, 12, 43, 52, 53, 83, 2, 9];
// }
//
// fn print_tab(tab: [i32; SIZE]) {
//     for t in tab {
//         print!("{} ", t);
//     }
//     println!();
//     println!();
// }
//
// fn min_i32(lhs: i32, rhs: i32) -> i32 {
//     if lhs < rhs {
//         lhs
//     } else {
//         rhs
//     }
// }
//
// fn find_min(tab: [i32; SIZE]) -> i32 {
//     check_size(SIZE);
//     let mut min = i32::MAX;
//
//     for t in tab {
//         min = min_i32(min, t);
//     }
//     for i in 0..SIZE {
//         min = min_i32(min, tab[i]);
//     }
//
//     min
// }
//
// fn plus_un(x: i32) -> i32 {
//     x + 1
// }
//
// fn cinq() -> i32 {
//     5
// }
//
// fn afficher_mesure_avec_unite(valeur: i32, unite: char) {
//     println!("La mesure est : {}{}", valeur, unite);
// }
//
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

fn main() {

}