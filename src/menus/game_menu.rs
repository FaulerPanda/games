use bevy::prelude::*;

use crate::functions::despawn_screen;
use crate::menus::{MenuButtonAction, MenuState, NORMAL_BUTTON, menu_button_node};
/// all elemnts on game screen need this
#[derive(Component)]
struct OnGameScreen;

/// plugin for the GameMenu
pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Games), game_menu_setup)
            .add_systems(OnExit(MenuState::Games), despawn_screen::<OnGameScreen>);
    }
}

/// spawns the game menu
fn game_menu_setup(mut commands: Commands) {
    let button_node = menu_button_node();

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnGameScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Pong,
                    children![Text::new("Pong"),]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![Text::new("Boogle")]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Back,
                    children![Text::new("Back")]
                )
            ]
        )],
    ));
}
