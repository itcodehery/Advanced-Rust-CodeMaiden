use std::io;

#[derive(PartialEq)]
enum MealPreference {
    VEG,
    NONVEG,
    VEGAN,
    JAIN,
    GLUTENFREE,
    KIDS,
}

fn as_a_string(meal: &MealPreference) -> String {
    match meal {
        MealPreference::VEG => {
            return "Veg".to_string();
        }
        MealPreference::NONVEG => {
            return "Non-Veg".to_string();
        }
        MealPreference::VEGAN => {
            return "Vegan".to_string();
        }
        MealPreference::JAIN => {
            return "Jain".to_string();
        }
        MealPreference::GLUTENFREE => {
            return "Gluten Free".to_string();
        }
        MealPreference::KIDS => {
            return "Kids".to_string();
        }
    }
}

struct Passenger {
    name: String,
    meal_preference: MealPreference,
}

// Read Input
fn read_input(passengers: &mut Vec<Passenger>) {
    println!("Enter data for 6 passengers:");
    for i in 0..6 {
        let mut temp_meal = String::new();
        let mut temp_pass: Passenger = Passenger {
            name: String::new(),
            meal_preference: MealPreference::VEG,
        };
        println!("Name of Passenger #{}: ", i + 1);
        io::stdin()
            .read_line(&mut temp_pass.name)
            .expect("didn't work lmao");
        temp_pass.name = temp_pass.name.trim().to_string();
        println!("Meal Preference for Passenger #{}", i + 1);
        io::stdin().read_line(&mut temp_meal).unwrap();
        match temp_meal.trim() {
            "veg" => {
                temp_pass.meal_preference = MealPreference::VEG;
            }
            "non-veg" => {
                temp_pass.meal_preference = MealPreference::NONVEG;
            }
            "vegan" => {
                temp_pass.meal_preference = MealPreference::VEGAN;
            }
            "jain" => {
                temp_pass.meal_preference = MealPreference::JAIN;
            }
            "gluten-free" => {
                temp_pass.meal_preference = MealPreference::GLUTENFREE;
            }
            "kids" => {
                temp_pass.meal_preference = MealPreference::KIDS;
            }
            _ => {
                dbg!("Error occured!");
            }
        }
        passengers.push(temp_pass);
    }
}
// Display Manifest
fn display_main_manifest(passengers: &Vec<Passenger>) {
    println!("------------------\nMain Manifest\n------------------");
    for i in 0..passengers.len() {
        println!("Passenger #{}", i + 1);
        println!("Name: {}", passengers[i].name);
        println!(
            "Meal Preference: {}\n",
            as_a_string(&passengers[i].meal_preference)
        );
    }
    println!("\n");
}

// Display Veg Manifest
fn display_veg_manifest(passengers: &Vec<Passenger>) {
    println!("------------------\nVeg Manifest\n------------------");
    for i in 0..passengers.len() {
        if passengers[i].meal_preference == MealPreference::VEG {
            println!("Passenger #{}", i + 1);
            println!("Name: {}", passengers[i].name);
        }
    }
    println!("\n");
}

// Display Non-Veg Manifest
fn display_nonveg_manifest(passengers: &Vec<Passenger>) {
    println!("------------------\nNon-Veg Manifest\n------------------");
    for i in 0..passengers.len() {
        if passengers[i].meal_preference == MealPreference::NONVEG {
            println!("Passenger #{}", i + 1);
            println!("Name: {}", passengers[i].name);
        }
    }
    println!("\n");
}

// Count People on Meal Preferences
fn count_meal_prefs(passengers: &Vec<Passenger>) {
    println!("----------------\nCount of People Grouped on Meal Preferences\n-----------------\n");
    let (mut veg, mut nonveg, mut vegan, mut jain, mut gltnfr, mut kids): (
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
    ) = (0, 0, 0, 0, 0, 0);
    for i in 0..passengers.len() {
        match passengers[i].meal_preference {
            MealPreference::VEG => {
                veg += 1;
            }
            MealPreference::NONVEG => {
                nonveg += 1;
            }
            MealPreference::VEGAN => {
                vegan += 1;
            }
            MealPreference::JAIN => {
                jain += 1;
            }
            MealPreference::GLUTENFREE => {
                gltnfr += 1;
            }
            MealPreference::KIDS => {
                kids += 1;
            }
        }
    }
    println!(
        "Veg: {}\nNon-Veg: {}\nVegan: {}\nJain:{}\nGluten-Free: {}\nKids: {}\n\n",
        veg, nonveg, vegan, jain, gltnfr, kids
    );
}

// Group people based on Meal Preferences
fn group_on_meal_prefs(passengers: &Vec<Passenger>) {
    println!("\n------\nSorted on Meal Preferences\n------\n");
    for pref in [
        MealPreference::VEG,
        MealPreference::NONVEG,
        MealPreference::VEGAN,
        MealPreference::JAIN,
        MealPreference::GLUTENFREE,
        MealPreference::KIDS,
    ] {
        let filtered: Vec<_> = passengers
            .iter()
            .filter(|p| p.meal_preference == pref)
            .collect();
        // Display filtered passengers
        for item in filtered {
            println!("- {}: {}", item.name, as_a_string(&item.meal_preference));
        }
    }
}
// Sort people based on their names
fn sort_on_names(passengers: &mut Vec<Passenger>) {
    passengers.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
}

fn main() {
    let mut passengers: Vec<Passenger> = Vec::new();
    // Read input: mutable reference
    read_input(&mut passengers);
    // Count of people under each Meal Prefs
    count_meal_prefs(&passengers);
    // Display Main Manifest
    display_main_manifest(&passengers);
    // Display Veg Manifest
    display_veg_manifest(&passengers);
    // Display Non-Veg Manifest
    display_nonveg_manifest(&passengers);
    // Group people based on meal prefs
    group_on_meal_prefs(&passengers);
    // Sort people based on their names
    sort_on_names(&mut passengers);
    // Display Sorted Manifest
    println!("\n.....Sorting by Names.....\n");
    println!("\n.....Displaying Manifest after Sorting.....\n");
    display_main_manifest(&passengers);
}
