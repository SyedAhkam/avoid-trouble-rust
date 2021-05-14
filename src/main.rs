use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin}
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

fn main() {
    App::build()
        
        // Set window properties
        .insert_resource(WindowDescriptor { 
            title: GAME_TITLE.to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: true,
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
        .run()
}

fn startup(mut commands: Commands, app_state: Res<State<AppState>>, asset_server: Res<AssetServer>) {
    info!("Starting up"); 

    commands.spawn_bundle(UiCameraBundle::default());

    // display State 
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "State: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateLabel);
}

fn change_state(mut app_state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::M) {
        info!("M was pressed!");

        match app_state.set(AppState::MainMenu) {
            Ok(()) => {
                info!("Changed to menu state");
            },
            Err(StateError) => {
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
            Err(StateError) => {
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
            Err(StateError) => {
                warn!("Already in paused state");
            }
        };    }
}

fn show_state(app_state: Res<State<AppState>>, mut query: Query<&mut Text, With<AppStateLabel>>) {
    for mut text in query.iter_mut() {
        text.sections[1].value = app_state.current().to_string();
    }
}
