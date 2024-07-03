use crate::animation::{AnimationIndices, AnimationTimer};
use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_player));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("female_adventurer.png"),
        ..default()
    });
}



























fn _spawn_player_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(96., 99.),
        8,
        1,
        None,
        None,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("female_adventurer_sheet.png"),
            ..default()
        },
        TextureAtlas::from(layout),
        AnimationTimer(Timer::new(Duration::from_millis(120), Repeating)),
        AnimationIndices { first: 0, last: 7 },
    ));
}
