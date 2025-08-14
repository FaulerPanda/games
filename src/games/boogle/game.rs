use bevy::prelude::*;
use rand::prelude::*;

use crate::{enums::GameState, functions::despawn_screen, menus::NORMAL_BUTTON};

/// all items on game screen need this
#[derive(Component)]
struct OnGameScreen;
/// all items on declaration screen need this
#[derive(Component)]
struct OnDeclarationScreen;

const DICE1: [char; 6] = ['I', 'U', 'N', 'E', 'E', 'S'];
const DICE2: [char; 6] = ['F', 'P', 'K', 'A', 'S', 'F'];
const DICE3: [char; 6] = ['R', 'D', 'I', 'X', 'L', 'E'];
const DICE4: [char; 6] = ['V', 'T', 'H', 'R', 'E', 'W'];
const DICE5: [char; 6] = ['L', 'R', 'Y', 'T', 'T', 'E'];
const DICE6: [char; 6] = ['O', 'T', 'S', 'E', 'S', 'I'];
const DICE7: [char; 6] = ['T', 'I', 'T', 'S', 'D', 'Y'];
const DICE8: [char; 6] = ['O', 'A', 'W', 'T', 'O', 'T'];
const DICE9: [char; 6] = ['A', 'C', 'S', 'O', 'H', 'P'];
const DICE10: [char; 6] = ['M', 'I', 'Q', 'N', 'U', 'H'];
const DICE11: [char; 6] = ['L', 'R', 'E', 'V', 'Y', 'D'];
const DICE12: [char; 6] = ['N', 'Z', 'N', 'R', 'H', 'L'];
const DICE13: [char; 6] = ['J', 'B', 'O', 'A', 'B', 'O'];
const DICE14: [char; 6] = ['U', 'I', 'C', 'T', 'M', 'O'];
const DICE15: [char; 6] = ['E', 'E', 'N', 'H', 'G', 'W'];
const DICE16: [char; 6] = ['N', 'A', 'E', 'A', 'G', 'E'];

const ALLDICES: [[char; 6]; 16] = [
    DICE1, DICE2, DICE3, DICE4, DICE5, DICE6, DICE7, DICE8, DICE9, DICE10, DICE11, DICE12, DICE13,
    DICE14, DICE15, DICE16,
];

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
        width: Val::Px(75.0),
        height: Val::Px(75.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let mut letters = Vec::new();
    for dice in ALLDICES {
        let letter = random_letter(&dice);
        letters.push(letter);
    }

    let mut rng = rand::rng();
    letters.shuffle(&mut rng);

    let letter_text_font = TextFont {
        font_size: 40.0,
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
                        children![(Text::new(letters[0].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[1].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[2].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[3].to_string()), letter_text_font.clone())]
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
                        children![(Text::new(letters[4].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[5].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[6].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[7].to_string()), letter_text_font.clone())]
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
                        children![(Text::new(letters[8].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[9].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[10].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[11].to_string()), letter_text_font.clone())]
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
                        children![(Text::new(letters[12].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[13].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[14].to_string()), letter_text_font.clone())]
                    ),
                    (
                        dice_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        children![(Text::new(letters[15].to_string()), letter_text_font.clone())]
                    )
                ]
            ),
        ],
    ));
}

/// give a random letter out of dice list
fn random_letter(dice: &[char]) -> char {
    let mut rng = rand::rng();
    *dice.choose(&mut rng).expect("Arry is empty")
}
