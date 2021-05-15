use bevy::prelude::*;

fn main() {
    App::build()
    .insert_resource(bevy::log::LogSettings {
        level: bevy::log::Level::DEBUG,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_state(GameState::Menu)
    .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(on_menu.system()))
    .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_play_button.system()))
    .add_system_set(SystemSet::on_enter(GameState::Game).with_system(on_game.system()))
    .add_system_set(SystemSet::on_update(GameState::Game).with_system(click_back_button.system()))
    .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Game,
}

type ButtonInteraction<'a> = (
    Entity,
    &'a Interaction,
    &'a Children,
);

struct PlayButton;
struct BackButton;

fn on_menu(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.15, 0.85, 0.15).into()),
            ..Default::default()
        })
        .insert(PlayButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn on_game(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.85, 0.15, 0.15).into()),
            ..Default::default()
        })
        .insert(BackButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Back".to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.0, 0.0, 0.0),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn click_play_button(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<Button>)>,
    text_query: Query<Entity, With<Text>>,
) {
    for (button, interaction, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        if *interaction == Interaction::Clicked {
            commands.entity(button).despawn();
            commands.entity(text).despawn();
            state.set(GameState::Game).unwrap();
        }
    }
}

fn click_back_button(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<Button>)>,
    text_query: Query<Entity, With<Text>>,
) {
    for (button, interaction, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        if *interaction == Interaction::Clicked {
            commands.entity(button).despawn();
            commands.entity(text).despawn();
            state.set(GameState::Menu).unwrap();
        }
    }
}
