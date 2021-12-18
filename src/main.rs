use chrono::prelude::*;
use std::io;

fn main() {
    let mut firstname = String::new();
    let mut year_of_birth = String::new();

    println!("What is you name?");

    io::stdin()
        .read_line(&mut firstname)
        .expect("Wrong input 😤");

    println!(
        "Thank you {}. Now, can you tell me your birth year?",
        firstname.trim()
    );

    io::stdin()
        .read_line(&mut year_of_birth)
        .expect("Wrong input 😤");

    let year_of_birth_int: i32 = year_of_birth.trim().to_string().parse().unwrap();

    let age = calculate_age(year_of_birth_int);

    println!(
        "Dear {}, according to my calculations, you are {} years old. Or almost {}... give me a break",
        firstname.trim(),
        age,
        age
    )
}

pub fn calculate_age(year_of_birth: i32) -> i32 {
    let current_year = Utc::now().year();
    current_year - year_of_birth
}

//tests that only works in 2021 😹

#[test]
fn calc_age() {
    assert_eq!(calculate_age(1990), 31);
    assert_eq!(calculate_age(1980), 41);
    assert_eq!(calculate_age(1966), 55);
    assert_eq!(calculate_age(1945), 76);
}
