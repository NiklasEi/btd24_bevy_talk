mod animation;
mod audio;
mod debug_info;
mod player;

use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, hello_world).run();
}

fn hello_world() {
    println!("Hello World!");
}
