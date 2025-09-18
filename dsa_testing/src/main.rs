// Merge Subroutine
fn main() {
    // Given two sorted arrays, merge them
    let arr = [1, 2, 3];
    let arr2 = [2, 5, 6];
    let mut res: [i32; 6] = [42; 6];
    for i in 0..arr.len() {
        for j in 0..arr2.len() {
            if arr[i] < arr2[j] {
                // Insert into arr at index i
                res[i] = arr[i];
            }
        }
    }

    println!("{:?}", res);
}
