use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[warn(dead_code)]
fn guess_the_number() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    password: String,
    is_active: bool,
}

#[warn(dead_code)]
fn structures() {
    let mut user1 = User {
        name: String::from("Andrés"),
        email: String::from("andres@gmail.com"),
        password: String::from("123"),
        is_active: true,
    };

    let name = user1.name;
    user1.name = String::from("Hey");

    println!("This is the name {}", name);
    println!("This is the name {}", user1.name);

    let new_user = build_user(
        String::from("new andres"),
        String::from("new email"),
        String::from("my new password"),
    );

    println!("This is a new user {}", new_user.name);

    let take_data_from_new_user = User {
        name: String::from("new user but with existing data"),
        ..new_user
    };

    println!("new user but stole data :( {:#?}", take_data_from_new_user,);
}

#[warn(dead_code)]
fn build_user(name: String, email: String, password: String) -> User {
    User {
        name,
        email,
        password,
        is_active: true,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };

    println!("The area is {}", rect.area());
    println!("rect can hold rect1 {}", rect.can_hold(&rect1));
    println!("rect can hold rect2 {}", rect.can_hold(&rect2));
}
