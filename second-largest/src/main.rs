// fn second_largest_element(nums: Vec<i32>) -> i32 {
//     let mut mod_vec = nums.clone();
//     mod_vec.sort();
//     mod_vec.reverse();
//     let res = mod_vec[1];
//     res
// }

fn second_largest_element(nums: Vec<i32>) -> i32 {
    let mut first_largest = &nums[0];
    // First loop for finding the largest
    for i in &nums {
        if i > first_largest {
            first_largest = i;
        }
    }
    // Assume second_largest is smaller by just one
    let mut second_largest = first_largest - 1;
    let mut diff = second_largest;

    // Second loop to find the second largest
    for i in &nums {
        // Ignore first largest
        if i == first_largest {
            continue;
        // if assumption is correct, return second largest
        } else if i == &second_largest {
            return second_largest;
        // if not, the value is lower than first largest by diff amount
        } else {
            let temp = first_largest - i;
            if diff < temp {
                continue;
            } else {
                diff = temp;
                second_largest = first_largest - diff;
            }
        }
    }

    second_largest
}

fn main() {
    let vec = vec![2, 5, 31, 6, 2, 7, 23, 61];
    println!(
        "The second largest element: {}",
        second_largest_element(vec)
    );
}
