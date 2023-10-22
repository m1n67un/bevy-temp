use bevy::prelude::Component;

fn main() {
    println!("Hello, world!");
}

pub fn hello_world() {
    println!("Hello World")
}

#[derive(Component)]
pub struct Person {
    pub name: String
}