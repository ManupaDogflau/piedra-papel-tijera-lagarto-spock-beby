
use rand::Rng;

use bevy::prelude::*;

// Define un tipo de dato para los posibles jugadores
#[derive(PartialEq, Default)]
enum Player {
    #[default]
    User,
    Robot,
    Answer,
    Reset,
}

#[derive(PartialEq, Default)]
enum Decision {
    #[default]
    Nothing,
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

#[derive(Default, Component, Resource)]
struct GameState {
    current_player: Player,
    user_select: Decision,
    computer_select: Decision,
}

#[derive(Component)]
struct Texto;

fn main() {
    App::new()
        .insert_resource(GameState::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_game_state)
        .add_startup_system(starting_text)
        .add_startup_system(setup_window)
        .add_system(keyboard_input)
        .add_system(computer_input)
        .add_system(select_winner)
        .add_system(reset)
        .add_system(text_color_system)
        .run();
}

fn reset(mut game_state: ResMut<GameState>) {
    if game_state.current_player == Player::Reset {
        game_state.user_select = Decision::Nothing;
        game_state.computer_select = Decision::Nothing;
        game_state.current_player = Player::User;
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

fn setup_game_state(mut commands: Commands) {
    commands.spawn(GameState {
        current_player: Player::User,
        user_select: Decision::Nothing,
        computer_select: Decision::Nothing,
    });
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut game_state: ResMut<GameState>) {
    if game_state.current_player == Player::User {
        if keys.just_pressed(KeyCode::Q) {
            game_state.user_select = Decision::Rock;
            game_state.current_player = Player::Robot;
        } else if keys.just_pressed(KeyCode::W) {
            game_state.user_select = Decision::Paper;
            game_state.current_player = Player::Robot;
        } else if keys.just_pressed(KeyCode::E) {
            game_state.user_select = Decision::Scissors;
            game_state.current_player = Player::Robot;
        } else if keys.just_pressed(KeyCode::R) {
            game_state.user_select = Decision::Lizard;
            game_state.current_player = Player::Robot;
        } else if keys.just_pressed(KeyCode::T) {
            game_state.user_select = Decision::Spock;
            game_state.current_player = Player::Robot;
        } else if keys.just_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
    }
}

fn computer_input(mut game_state: ResMut<GameState>) {
    if game_state.current_player == Player::Robot {
        let al = rand::thread_rng().gen_range(1..6);
        match al {
            1 => game_state.computer_select = Decision::Rock,
            2 => game_state.computer_select = Decision::Paper,
            3 => game_state.computer_select = Decision::Scissors,
            4 => game_state.computer_select = Decision::Lizard,
            5 => game_state.computer_select = Decision::Spock,
            _ => println!("Error"),
        }
        game_state.current_player = Player::Answer;
    }
}

fn select_winner(mut game_state: ResMut<GameState>) {
    /*1 = rock
    2 = paper
    3 = scissors
    4 = lizard
    5 = spock*/
    if game_state.current_player == Player::Answer {
        if game_state.user_select == game_state.computer_select {
            println!("Tie!");
        } else if game_state.user_select == Decision::Rock && game_state.computer_select == Decision::Scissors {
            println!("Rock crushes scissors!");
            println!("You win!");
        } else if game_state.user_select == Decision::Rock && game_state.computer_select == Decision::Lizard {
            println!("Rock crushes lizard!");
            println!("You win!");
        } else if game_state.user_select == Decision::Paper && game_state.computer_select == Decision::Rock {
            println!("Paper covers rock!");
            println!("You win!");
        } else if game_state.user_select == Decision::Paper && game_state.computer_select == Decision::Spock {
            println!("Paper disproves Spock!");
            println!("You win!");
        } else if game_state.user_select == Decision::Scissors && game_state.computer_select == Decision::Paper {
            println!("Scissors cuts paper!");
            println!("You win!");
        } else if game_state.user_select == Decision::Scissors && game_state.computer_select == Decision::Lizard {
            println!("Scissors decapitates lizard!");
            println!("You win!");
        } else if game_state.user_select == Decision::Lizard && game_state.computer_select == Decision::Paper {
            println!("Lizard eats paper!");
            println!("You win!");
        } else if game_state.user_select == Decision::Lizard && game_state.computer_select == Decision::Spock {
            println!("Lizard poisons Spock!");
            println!("You win!");
        } else if game_state.user_select == Decision::Spock && game_state.computer_select == Decision::Rock {
            println!("Spock vaporizes rock!");
            println!("You win!");
        } else if game_state.user_select == Decision::Spock && game_state.computer_select == Decision::Scissors {
            println!("Spock smashes scissors!");
            println!("You win!");
        } else {
            println!("You lose!");
        }
        game_state.current_player = Player::Reset;
    }
}

fn setup_window(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ]),
        Texto,
    ));
}

fn text_color_system(mut query: Query<&mut Text, With<Texto>>, mut game_state: ResMut<GameState>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{0:#?}", game_state.user_select);
    }
}
