use bevy::prelude::*;

use crate::enums::GameState;
use crate::functions::despawn_screen;
use crate::menus::{
    MenuButtonAction, MenuState, NORMAL_BUTTON, button_action, button_system, menu_button_node,
};

/// all components on the MainScreen need this
#[derive(Component)]
struct OnMainScreen;

/// menu plugin
pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainScreen>)
            .add_systems(
                Update,
                (button_system, button_action).run_if(in_state(GameState::Menu)),
            );
    }
}

/// change MenuState::Dissabled to MenuState::Main
fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

/// spawns all elements for the mainmenu
fn main_menu_setup(mut commands: Commands) {
    let button_node = menu_button_node();

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnMainScreen,
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
                    MenuButtonAction::Games,
                    children![Text::new("Game selection"),]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![Text::new("Quit")]
                )
            ]
        )],
    ));
}
