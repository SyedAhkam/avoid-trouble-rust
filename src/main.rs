#![allow(non_snake_case)]

use bevy::{
    prelude::*,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}
};

use std::fmt;

const GAME_TITLE: &str = "Avoid Trouble";
const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGHT: f32 = 500.;
const BACKGROUND_COLOR: &str = "2E3440";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused
}

// This is a hack to get enum fields as `String`
impl fmt::Display for AppState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct AppStateLabel;
struct FpsText;

fn main() {
    App::build()
        
        // Set window properties
        .insert_resource(WindowDescriptor { 
            title: GAME_TITLE.to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
    
        // Set background color
        .insert_resource(ClearColor(Color::hex(BACKGROUND_COLOR).unwrap())) 

        // Add default plugins
        .add_plugins(DefaultPlugins)

        // Add frame rate plugin to show fps
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        // Add initial state
        .add_state(AppState::MainMenu)

        // Game startup system
        .add_startup_system(startup.system())

        // States
        .add_system(change_state.system())
        .add_system(show_state.system())

        // FPS counter
        .add_system(show_fps.system())
        .run()
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Starting up"); 

    commands.spawn_bundle(UiCameraBundle::default());

    // Load fonts
    asset_server.load_untyped("fonts/OpenSans-Regular.ttf");
    asset_server.load_untyped("fonts/OpenSans-Bold.ttf");

    // Display State 
    // commands
        // .spawn_bundle(TextBundle {
            // style: Style {
                // align_self: AlignSelf::FlexEnd,
                // ..Default::default()
            // },
            // text: Text {
                // sections: vec![
                    // TextSection {
                        // value: "State: ".to_string(),
                        // style: TextStyle {
                            // font: asset_server.get_handle("fonts/OpenSans-Regular.ttf"),
                            // font_size: 30.0,
                            // color: Color::WHITE,
                        // },
                    // },
                    // TextSection {
                        // value: "".to_string(),
                        // style: TextStyle {
                            // font: asset_server.get_handle("fonts/OpenSans-Regular.ttf"),
                            // font_size: 30.0,
                            // color: Color::GOLD,
                        // },
                    // },
                // ],
                // ..Default::default()
            // },
            // ..Default::default()
        // })
        // .insert(AppStateLabel);

    // Display State
    commands
        .spawn_bundle(TextBundle{
            style: Style {
                align_self: AlignSelf::FlexEnd,
                margin: Rect {left: Val::Px(2.), ..Default::default()},
                ..Default::default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_server.get_handle("fonts/OpenSans-Regular.ttf"),
                    font_size: 30.,
                    color: Color::GOLD
                },
                Default::default()
            ),
            ..Default::default()
        })
        .insert(AppStateLabel);

    // Display FPS
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {right: Val::Px(2.), ..Default::default()},
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/OpenSans-Regular.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/OpenSans-Regular.ttf"),
                            font_size: 30.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

fn change_state(mut app_state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::M) {
        info!("M was pressed!");

        match app_state.set(AppState::MainMenu) {
            Ok(()) => {
                info!("Changed to menu state");
            },
            Err(_StateError) => {
                warn!("Already in menu state");
            }
        };
    }
    if keys.just_pressed(KeyCode::G) {
        info!("G was pressed!");
        
        match app_state.set(AppState::InGame) {
            Ok(()) => {
                info!("Changed to game state");
            },
            Err(_StateError) => {
                warn!("Already in game state");
            }
        };
    }
    if keys.just_pressed(KeyCode::P) {
        info!("P was pressed!");
        
        match app_state.set(AppState::Paused) {
            Ok(()) => {
                info!("Changed to paused state");
            },
            Err(_StateError) => {
                warn!("Already in paused state");
            }
        };    
    }
}

fn show_state(app_state: Res<State<AppState>>, mut query: Query<&mut Text, With<AppStateLabel>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = app_state.current().to_string();
    }
}

fn show_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}
