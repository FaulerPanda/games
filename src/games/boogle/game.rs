use bevy::prelude::*;

use crate::{enums::GameState, functions::despawn_screen, menus::NORMAL_BUTTON};

/// all items on game screen need this
#[derive(Component)]
struct OnGameScreen;
/// all items on declaration screen need this
#[derive(Component)]
struct OnDeclarationScreen;

/// States of Boogle
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum BoogleState {
    #[default]
    Disabled,
    Declaration,
    Game,
}

/// Boogle Plugin
pub struct BooglePlugin;
impl Plugin for BooglePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BoogleState>()
            .add_systems(OnEnter(GameState::Boogle), game_setup)
            //.add_systems(OnEnter(BoogleState::Declaration), ...)
            // .add_systems(
            //     OnExit(BoogleState::Declaration),
            //     despawn_screen::<OnDeclarationScreen>,
            // )
            .add_systems(OnEnter(BoogleState::Game), boogle_setup)
            .add_systems(OnExit(BoogleState::Game), despawn_screen::<OnGameScreen>);
    }
}

/// change the booglestate dissabled to game
fn game_setup(mut boogle_state: ResMut<NextState<BoogleState>>) {
    // change to BoogleState::declaration
    boogle_state.set(BoogleState::Game);
}

/// setup for the boogle game surface
fn boogle_setup(mut commands: Commands) {
    let dice_node = Node {
        width: Val::Px(50.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        OnGameScreen,
        children![
            (
                // first row
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("A")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("B")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("C")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("D")]
                    )
                ]
            ),
            (
                // second row
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("A")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("B")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("C")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("D")]
                    )
                ]
            ),
            (
                // third row
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("A")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("B")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("C")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("D")]
                    )
                ]
            ),
            (
                // fourth row
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("A")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("B")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("C")]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![Text::new("D")]
                    )
                ]
            ),
        ],
    ));
}
