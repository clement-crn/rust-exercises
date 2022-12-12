use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}
#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elon Musk ".to_string())));
    commands.spawn((Person, Name("Andy".to_string())));
    commands.spawn((Person, Name("Yennie".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}
