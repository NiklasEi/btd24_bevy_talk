use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

pub struct DebugInfoPlugin;

impl Plugin for DebugInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PerfUiPlugin)
            // we want Bevy to measure these values for us:
            .add_plugins((
                FrameTimeDiagnosticsPlugin,
                EntityCountDiagnosticsPlugin,
                SystemInformationDiagnosticsPlugin,
            ))
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}
