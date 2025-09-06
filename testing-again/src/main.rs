// Task: Input R x C grid size and a list of student program codes (BCA, BSc, BCom, …). Fill seats row-
// wise but skip to next seat if previous seat has same program; wrap to next row. Print seating
// chart.

use rand::seq::IndexedRandom;

fn main() {
    let classes: Vec<&str> = vec!["BCA", "BSc", "BBA", "BCom", "MBA", "MCA"];
    let mut seats: Vec<Vec<&str>> = vec![vec![""]];
    let mut prev: &str = "";

    for i in 0..4 {
        for j in 0..3 {
            if prev == seats[i][j] {
                prev = "";
                continue;
            } else {
                seats[i][j] = classes.choose(&mut rand::rng()).unwrap();
                prev = seats[i][j];
            }
        }
    }
}
