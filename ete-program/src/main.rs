use std::{io, ops::Deref};

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
        println!("Passenger #{}", i + 1);
        println!("Name: {}", passengers[i].name);
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
    let mut veg_pass: Vec<&Passenger> = Vec::new();
    let mut nonveg_pass: Vec<&Passenger> = Vec::new();
    let mut vegan_pass: Vec<&Passenger> = Vec::new();
    let mut jain_pass: Vec<&Passenger> = Vec::new();
    let mut gltnfr_pass: Vec<&Passenger> = Vec::new();
    let mut kids_pass: Vec<&Passenger> = Vec::new();
    for i in 0..passengers.len() {
        let temp: &Passenger = &passengers[i];
        match passengers[i].meal_preference {
            MealPreference::VEG => {
                veg_pass.push(temp);
            }
            MealPreference::NONVEG => {
                nonveg_pass.push(temp);
            }
            MealPreference::VEGAN => {
                vegan_pass.push(temp);
            }
            MealPreference::JAIN => {
                jain_pass.push(temp);
            }
            MealPreference::GLUTENFREE => {
                gltnfr_pass.push(temp);
            }
            MealPreference::KIDS => {
                kids_pass.push(temp);
            }
        }
    }
    println!("\nGrouped on Meal Prefs:\n");
    for i in 0..veg_pass.len() {
        println!("{}", veg_pass[i].deref().name);
    }
    for i in 0..nonveg_pass.len() {
        println!("{}", nonveg_pass[i].deref().name);
    }
    for i in 0..vegan_pass.len() {
        println!("{}", vegan_pass[i].deref().name);
    }
    for i in 0..jain_pass.len() {
        println!("{}", jain_pass[i].deref().name);
    }
    for i in 0..gltnfr_pass.len() {
        println!("{}", gltnfr_pass[i].deref().name);
    }
    for i in 0..kids_pass.len() {
        println!("{}", kids_pass[i].deref().name);
    }
    println!("\n");
}

fn main() {
    let mut passengers: Vec<Passenger> = Vec::new();
    read_input(&mut passengers);
    count_meal_prefs(&passengers);
    display_main_manifest(&passengers);
    display_veg_manifest(&passengers);
    display_nonveg_manifest(&passengers);
    group_on_meal_prefs(&passengers);
}
