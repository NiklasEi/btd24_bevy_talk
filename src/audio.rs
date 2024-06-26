use bevy::prelude::*;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_background);
    }
}

fn start_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("Windless Slopes.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}
