use std::io;

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
            "Meal Preference: {}",
            as_a_string(&passengers[i].meal_preference)
        );
    }
    println!("\n");
}

fn main() {
    let mut passengers: Vec<Passenger> = Vec::new();
    read_input(&mut passengers);
    display_main_manifest(&passengers);
}
