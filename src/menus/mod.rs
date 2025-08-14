use bevy::prelude::*;

use crate::enums::GameState;

pub mod game_menu;
pub mod main_menu;

pub const NORMAL_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5);
pub const HOVERED_BUTTON: Color = Color::srgb(0.9, 0.9, 0.9);

/// different menu states
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Games,
    #[default]
    Disabled,
}

/// button actions
#[derive(Component)]
pub enum MenuButtonAction {
    Games,
    Pong,
    Boogle,
    Back,
    Quit,
}

/// node for all menu buttons
pub fn menu_button_node() -> Node {
    Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        // horizontale ausrichtung
        justify_content: JustifyContent::Center,
        // vertikale ausrichtung
        align_items: AlignItems::Center,
        ..default()
    }
}

/// change the collors of the button if pressed or hovered
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        *background_color = match *interaction {
            Interaction::Pressed => Color::srgb(1.0, 1.0, 1.0).into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}

/// change the state depends on wich button is pressed
pub fn button_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit.write(AppExit::Success);
                }
                MenuButtonAction::Back => menu_state.set(MenuState::Main),
                MenuButtonAction::Games => {
                    menu_state.set(MenuState::Games);
                }
                MenuButtonAction::Pong => {
                    menu_state.set(MenuState::Disabled);
                    game_state.set(GameState::Pong);
                }
                MenuButtonAction::Boogle => {
                    menu_state.set(MenuState::Disabled);
                    game_state.set(GameState::Boogle);
                }
            }
        }
    }
}
