use std::collections::HashMap;

// Task 1: Year Categorization Function (2 Marks)
fn year_categorize(slice: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut even_slice: Vec<i32> = vec![];
    let mut odd_slice: Vec<i32> = vec![];

    for item in slice {
        if item % 2 == 0 {
            even_slice.push(*item);
        } else {
            odd_slice.push(*item);
        }
    }

    (even_slice, odd_slice)
}

// Task 2: Identifier Length Grouping Function (2 Marks)
fn length_grouper(asset_ids: Vec<&str>) -> HashMap<usize, Vec<&str>> {
    let mut res_map: HashMap<usize, Vec<&str>> = HashMap::new();

    for item in &asset_ids {
        let key = item.len();
        let mut val: Vec<&str> = vec![];
        for item in &asset_ids {
            if item.len() == key {
                val.push(item);
            }
        }
        res_map.insert(key, val);
    }

    res_map
}

// Task 3: Year Ending Frequency Function (2 Marks)
fn ending_frequency(rec_years: &Vec<i32>) -> HashMap<i32, i32> {
    let mut res_map: HashMap<i32, i32> = HashMap::new();
    for item in rec_years {
        let key = item % 10;
        let mut val = 0;
        for item in rec_years {
            if item % 10 == key {
                val += 1;
            }
        }
        res_map.insert(key, val);
    }

    res_map
}

// Task 4: Recent Year Selection Function
fn recent_year(rec_years: &[i32]) -> Vec<i32> {
    let mut res_slice: Vec<i32> = rec_years.to_owned();
    res_slice.reverse();

    res_slice.split_off(4)
}

// Task 5: Alphabetic Identifier Filter Function
// Write a function that takes the AssetIDs slice and returns a new slice containing only
// identifiers that:
// ● Have a length greater than 4
// ● Contain only alphabetic characters
fn alpha_filter(asset_ids: Vec<&str>) -> Vec<&str> {
    let mut new_ads: Vec<&str> = vec![];
    for item in asset_ids {
        if item.len() > 4 && item.chars().all(|x| x.is_alphabetic()) {
            new_ads.push(item);
        }
    }

    new_ads
}

fn main() {
    let record_years = vec![1900, 1947, 2000, 2012, 2023, 2024, 2100, 1985, 1975];
    let asset_ids = vec![
        "naman",
        "12321",
        "sweets",
        "malayalam",
        "mela",
        "101",
        "isro",
    ];

    println!("Year Categorize:\n{:?}", year_categorize(&record_years));
    println!("Length Grouper:\n{:?}", length_grouper(asset_ids.clone()));
    println!("Ending Frequency:\n{:?}", ending_frequency(&record_years));
    println!("Recent Year:\n{:?}", recent_year(&record_years));
    println!("Alphabet Filter:\n{:?}", alpha_filter(asset_ids));
}
