use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EverdellPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Clone, Debug)]
enum CardType {
    Traveler,
    Production,
    Destination,
    Governance,
    Prosperity,
}

impl Default for CardType {
    fn default() -> Self {
        CardType::Traveler
    }
}

#[derive(Component, Default, Debug)]
struct Card {
    name: String,
    cost: usize, // TODO: cost should be in terms of stash currency
    value: usize,
    card_type: CardType,
}

#[derive(Component, Default, Debug)]
struct CardUi;

#[derive(Default, Debug)]
struct Deck {
    cards: Vec<Card>,
}

#[derive(Component, Default, Debug)]
struct DrawPile;

#[derive(Component, Default, Debug)]
struct PlayerHand(Deck);

#[derive(Component)]
struct Farm;

#[derive(Component, Default, Debug)]
struct PlayerStash {
    berries: usize,
    resin: usize,
    stones: usize,
    tokens: usize,
    wood: usize,
}

#[derive(Component, Default, Debug)]
struct Player;

#[derive(Bundle, Default, Debug)]
struct PlayerBundle {
    player: Player,
    hand: PlayerHand,
    stash: PlayerStash,
}

#[derive(Resource, Default, Debug)]
struct Game {
    draw_pile: Deck,
    discard_pile: Deck,
}

#[derive(Component)]
struct PlayerStashUi;

#[derive(Component)]
struct PlayerHandUi;

struct PlayCardEvent {
    card: Entity,
}

struct EverdellPlugin;

impl Plugin for EverdellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Game>()
            .add_event::<PlayCardEvent>()
            .add_startup_system(setup)
            .add_startup_system(everdell_setup)
            .add_system(hand_interaction_system)
            .add_system(stash_ui_system);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // gameboard
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

fn everdell_setup(mut commands: Commands, asset_server: Res<AssetServer>, game: Res<Game>) {
    let font = asset_server.load("screen_debug_text.ttf");

    game.draw_pile.cards.push(Card {
        name: String::from("Farm"),
        cost: 1, // TODO: cost should be in terms of stash currency
        value: 1,
        card_type: CardType::Production,
    });
    game.draw_pile.cards.push(Card {
        name: String::from("Farm"),
        cost: 1, // TODO: cost should be in terms of stash currency
        value: 1,
        card_type: CardType::Production,
    });
    game.draw_pile.cards.push(Card {
        name: String::from("Farm"),
        cost: 1, // TODO: cost should be in terms of stash currency
        value: 1,
        card_type: CardType::Production,
    });
    game.draw_pile.cards.push(Card {
        name: String::from("Resin Refinery"),
        cost: 1, // TODO: cost should be in terms of stash currency
        value: 1,
        card_type: CardType::Production,
    });
    game.draw_pile.cards.push(Card {
        name: String::from("Resin Refinery"),
        cost: 1, // TODO: cost should be in terms of stash currency
        value: 1,
        card_type: CardType::Production,
    });
    game.draw_pile.cards.push(Card {
        name: String::from("Twig Barge"),
        cost: 1, // TODO: cost should be in terms of stash currency
        value: 1,
        card_type: CardType::Production,
    });

    commands.spawn(PlayerBundle {
        hand: PlayerHand(Deck {
            cards: vec![
                Card {
                    name: String::from("Farm"),
                    cost: 1, // TODO: cost should be in terms of stash currency
                    value: 1,
                    card_type: CardType::Production,
                },
                Card {
                    name: String::from("Resin Refinery"),
                    cost: 1, // TODO: cost should be in terms of stash currency
                    value: 1,
                    card_type: CardType::Production,
                },
                Card {
                    name: String::from("Twig Barge"),
                    cost: 1, // TODO: cost should be in terms of stash currency
                    value: 1,
                    card_type: CardType::Production,
                }
            ],
        }),
        ..default()
    });

    // player stash ui
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new("", TextStyle {
                    font: font.clone(),
                    font_size: 14.0,
                    color: Color::rgb(0.7, 0.7, 0.7),
                }),
            ]).with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            })
        )
        .insert(PlayerStashUi);

    // player hand ui
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(0.0),
                    ..default()
                },
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(PlayerHandUi);
}

fn hand_ui_system(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut hand_query: Query<&PlayerHand, With<Player>>,
    mut hand_ui_query: Query<Entity, With<PlayerHandUi>>
) {
    let &hand = hand_query.single();
    let mut hand_ui = hand_ui_query.single();

    for card in hand.0.cards {
        commands.entity(hand_ui).add_children(|parent| {
            spawn_card_ui(parent, asset_server, card);
        });
    }
}

fn spawn_card_ui(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, card: Card) -> Entity {
    let font = asset_server.load("screen_debug_text.ttf");

    parent
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                margin: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: Color::rgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(card.name, TextStyle {
                    font: font.clone(),
                    font_size: 14.0,
                    color: Color::rgb(0.0, 0.0, 0.0),
                })
            );
        })
        .insert(CardUi)
        .id()
}

fn stash_ui_system(
    mut text_query: Query<&mut Text, With<PlayerStashUi>>,
    stash_query: Query<&PlayerStash>
) {
    let mut text = text_query.single_mut();
    let stash = stash_query.single();
    text.sections[0].value = format!(
        "Berries: {}\nResin: {}\nStones: {}\nWood: {}\nTokens: {}",
        stash.berries,
        stash.resin,
        stash.stones,
        stash.wood,
        stash.tokens
    );
}

const NORMAL_CARD_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_CARD_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_CARD_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);
fn hand_interaction_system(
    commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &Card),
        (Changed<Interaction>, With<CardUi>)
    >,
    eventWriter: EventWriter<PlayCardEvent>
) {
    for (e, interaction, mut color, card_ui) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_CARD_COLOR.into();
                commands.entity(e).remove::<(CardUi,)>();
                eventWriter.send(PlayCardEvent { card: e });
            }
            Interaction::Hovered => {
                *color = HOVERED_CARD_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_CARD_COLOR.into();
            }
        }
    }
}