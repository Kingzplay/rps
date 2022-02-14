

use rand::{ //importing crates that enables the ability to randomize stuff
    distributions::{Distribution, Standard},
    Rng,
    prelude::*
};

pub mod uis; //function to take user input without writing 1984 lines

pub enum Objects { //It's obvious lol
    Rock,
    Paper,
    Scissor
}



impl Distribution<Objects> for Standard {  //Enables the random function to randomize enum variants
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Objects {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..2) { // rand 0.8
            0 => Objects::Rock,
            1 => Objects::Paper,
            _ => Objects::Scissor,
        }
    }
}

use Objects::*; //makes sure there's no need to precise the enum's name before its variants each time


fn v(x: Objects) -> String {  //Set a comparable value to each variant  
       let x =  match x {
        Rock => String::from("rock"),
        Paper => String::from("paper"),
        Scissor => String::from("scissor"),
        };

        x
    }

pub fn matcher(usr_in: String) { // function to compare the user's choice and computer's choice and determine the winner, or a draw
    let computer: Objects = random();
    let computer = v(computer); //the variable is now comparable to the user input thanks to the previous function
    println!("You played: {}\nComputer played: {}", usr_in,computer);

    if usr_in == computer {
        println!("Equal");
    }

    // if statement that handles every winning case for the player
    else if usr_in == v(Rock) && computer == v(Scissor) || usr_in == v(Paper) && computer == v(Rock) || usr_in == v(Scissor) && computer == v(Paper) {
        println!("You won!")
    }

    // if statement handling every loss case for the player
    else if computer == v(Rock) && usr_in == v(Scissor) || computer == v(Paper) && usr_in == v(Rock) || computer == v(Scissor) && usr_in == v(Paper) {
        println!("You lost!") }
    
    else {
        println!("Something went wrong somehow")
    }
                        }
