use bevy::prelude::*;

use crate::{
    enums::PongState,
    functions::{countdown, despawn_screen},
    structs::GeneralTimer,
};

#[derive(Component)]
struct OnLoadingScreen;

pub struct LoadingScreenPlugin;
impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PongState::LoadingScreen), loading_screen_setup)
            .add_systems(
                OnExit(PongState::LoadingScreen),
                despawn_screen::<OnLoadingScreen>,
            )
            .add_systems(Update, timer.run_if(in_state(PongState::LoadingScreen)));
    }
}
fn loading_screen_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let turtle = asset_server.load("turtle.png");

    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        OnLoadingScreen,
        children![(
            ImageNode::new(turtle),
            Node {
                // This will set the logo to be 200px wide, and auto adjust its height
                width: Val::Px(500.0),
                ..default()
            },
        )],
    ));
    commands.insert_resource(GeneralTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn timer(
    time: Res<Time>,
    mut game_state: ResMut<NextState<PongState>>,
    timer: ResMut<GeneralTimer>,
) {
    if countdown(time, timer) {
        game_state.set(PongState::Menu);
    }
}
