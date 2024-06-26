mod animation;
mod audio;
mod debug_info;
mod player;

use bevy::prelude::*;
use crate::animation::SpriteAnimationPlugin;
use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, SpriteAnimationPlugin))
        .add_systems(Startup, hello_world)
        .run();
}

fn hello_world() {
    println!("Hello World!");
}
