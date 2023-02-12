use bevy::prelude::*;
use rand::Rng;


static mut USER_SELECT: i32 = 0;
static mut computer_select: i32 = 0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(starting_text)
        .add_system(keyboard_input)
        .add_system(computer_input)
        .add_system(select_winner)
        .add_system(reset)
        .run();
}

fn reset() {
    unsafe {
        USER_SELECT = 0;
        computer_select = 0;
    }
}

fn starting_text() {
    println!("Welcome to Rock, Paper, Scissors, Lizard, Spock!");
    println!("Press Q for Rock");
    println!("Press W for Paper");
    println!("Press E for Scissors");
    println!("Press R for Lizard");
    println!("Press T for Spock");
    println!("Press Escape to exit");
}

fn keyboard_input(keys: Res<Input<KeyCode>>) {
    unsafe {
       
            if keys.just_pressed(KeyCode::Q) {
                USER_SELECT = 1;
            } else if keys.just_pressed(KeyCode::W) {
                USER_SELECT = 2;
            } else if keys.just_pressed(KeyCode::E) {
                USER_SELECT = 3;
            } else if keys.just_pressed(KeyCode::R) {
                USER_SELECT = 4;
            } else if keys.just_pressed(KeyCode::T) {
                USER_SELECT = 5;
            } else if keys.just_pressed(KeyCode::Escape) {
                std::process::exit(0);
            }
    }
}

fn computer_input() {
    unsafe {
        computer_select = rand::thread_rng().gen_range(1..6);
    }
}

fn select_winner() {
    /*1 = rock
    2 = paper
    3 = scissors
    4 = lizard
    5 = spock*/
    unsafe {
        if USER_SELECT == computer_select {
            println!("Tie!");
        } else if USER_SELECT == 1 && computer_select == 3
            || USER_SELECT == 1 && computer_select == 4
        {
            println!("You win!");
        } else if USER_SELECT == 2 && computer_select == 1
            || USER_SELECT == 2 && computer_select == 5
        {
            println!("You win!");
        } else if USER_SELECT == 3 && computer_select == 2
            || USER_SELECT == 3 && computer_select == 4
        {
            println!("You win!");
        } else if USER_SELECT == 4 && computer_select == 2
            || USER_SELECT == 4 && computer_select == 5
        {
            println!("You win!");
        } else if USER_SELECT == 5 && computer_select == 1
            || USER_SELECT == 5 && computer_select == 3
        {
            println!("You win!");
        } else {
            println!("You lose!");
        }
    }
}
