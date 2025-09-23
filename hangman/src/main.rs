use rand::prelude::IndexedRandom;
use rand::prelude::SliceRandom;
use std::io::stdin;

fn is_vowel(letter: char) -> bool {
    let ch: char = letter.clone().to_ascii_lowercase();
    if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
        return true;
    }
    return false;
}

fn is_subset_of(guesses: &Vec<char>, main_set: &Vec<char>) -> bool {
    main_set.iter().all(|&item| guesses.contains(&item))
}

fn choose_random_string() -> String {
    let words: Vec<&str> = vec![
        "Inception",
        "Charlie's Angels",
        "Interstellar",
        "The Dark Knight",
        "Joker",
        "Her",
        "The Hunger Games",
        "The Last of Us",
        "Dark",
    ];
    let mut rng = rand::rng();
    let mut words_to_shu: Vec<&str> = words.clone();
    words_to_shu.shuffle(&mut rng);
    let i = words_to_shu.choose(&mut rng);
    match i {
        Some(x) => x.to_string().trim().to_string(),
        None => words_to_shu[0].to_string().trim().to_string(),
    }
}

fn main() {
    let mut count: i32 = 5;
    println!("Welcome to the Hangman Game:");
    let word: String = choose_random_string();
    println!("The word has been chosen, start guessing: ");
    let mut guesses: Vec<char> = vec![];
    // Initializing guesses with vowels first
    for lett in word.chars() {
        if is_vowel(lett) {
            guesses.push(lett);
        }
    }
    // Printing the things for the first time
    for lett in word.chars() {
        if lett.is_whitespace() {
            print!(" ");
        } else if is_vowel(lett) {
            print!("{}", lett);
        } else {
            print!("_");
        }
    }

    let mut current_guess: String = String::new();
    let word_chars: Vec<char> = word.chars().collect();

    // repeated guess
    while count != 0 {
        current_guess.clear();

        println!("\nEnter your guess: ");
        stdin().read_line(&mut current_guess).unwrap();

        let current_guess = current_guess.trim().parse::<char>().unwrap();
        guesses.push(current_guess);
        if !word_chars.contains(&current_guess) {
            count -= 1;
            println!("Oops, wrong letter! {} more chance(s) remaining!", count);
        }
        if is_subset_of(&guesses, &word_chars) {
            println!("-------------");
            println!("You win!!!!");
            println!("-------------");
            println!("The word was: {}", word);
            println!("-------------");
            return;
        }
        for lett in word.chars() {
            if guesses.contains(&lett) {
                print!("{}", lett);
                continue;
            }
            if lett.is_whitespace() {
                print!(" ");
            } else if is_vowel(lett) {
                print!("{}", lett);
            } else {
                print!("_");
            }
        }
    }

    println!("-------------");
    println!("You lose!!!!");
    println!("-------------");
    println!("The word was: {}", word);
    println!("-------------");
}
