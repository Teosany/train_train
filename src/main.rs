// Kata
fn main () {

}

//Convert number to reversed array of digits
// fn digitize(n: u64) -> Vec<u8> {
//     let mut digits= Vec::new();
//     let mut n= n;
//     while n > 9 {
//         digits.push((n % 10) as u8);
//         n = n / 10;
//     }
//     digits.push(n as u8);
//     digits
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_fixed() {
//         assert_eq!(digitize(348597), vec![7,9,5,8,4,3]);
//         assert_eq!(digitize(35231), vec![1,3,2,5,3]);
//         assert_eq!(digitize(0), vec![0]);
//     }
// }

//Reversed sequence

// fn reverse_seq(n: u32) -> Vec<u32> {
//     let mut vec = vec![];
//
//     for i in (1..= n).rev() {
//         vec.push(i);
//     }
//     vec
//
//     (1..=n).rev().collect()
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn sample_test() {
//         assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
//     }
// }



//Is he gonna survive?

// fn hero(bullets: u16, dragons: u16) -> bool {
//     bullets >= (dragons * 2)
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn basic() {
//         assert_eq!(hero(10, 5), true);
//         assert_eq!(hero(7, 4), false);
//         assert_eq!(hero(4, 5), false);
//         assert_eq!(hero(100, 40), true);
//         assert_eq!(hero(1500, 751), false);
//         assert_eq!(hero(0, 1), false);
//     }
// }




// Grasshopper Messi goals
// function

// fn goals(la_liga_goals: i32, champions_league_goals: i32, copa_del_rey_goals: i32) -> i32 {
//     la_liga_goals + champions_league_goals + copa_del_rey_goals
// }
//
// #[cfg(test)]
// mod tests {
//     use super::goals;
//
//     fn dotest(a: i32, b: i32, c: i32, expected: i32) {
//         let actual = goals(a, b, c);
//         assert_eq!(actual, expected, "With la_liga_goals = {a}, champions_league_goals = {b}, copa_del_rey_goals = {c}\nExpected {expected} but got {actual}")
//     }
//
//     #[test]
//     fn test_goals() {
//         dotest(0, 0, 0, 0);
//         dotest(43, 10, 5, 58);
//     }
// }