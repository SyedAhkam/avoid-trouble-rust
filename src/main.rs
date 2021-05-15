#![allow(non_snake_case)]

use bevy::{
    prelude::*,
    ecs::system::EntityCommands,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}
};

use std::fmt;

static GAME_TITLE: &str = "Avoid Trouble";
static WINDOW_WIDTH: f32 = 700.;
static WINDOW_HEIGHT: f32 = 500.;

static BACKGROUND_COLOR: &str = "2E3440";
static PRIMARY_TEXT_COLOR: &str = "BF616A";
static SECONDARY_TEXT_COLOR: &str = "EBCB8B";

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

// Structs for UI
struct AppStateLabel;
struct FpsText;
struct TitleText;

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
        .add_system(change_state_menu.system())
        .add_system(change_state_game.system())

        .add_system(show_state.system())

        // FPS counter
        .add_system(show_fps.system())


        // UI specific systems
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_ui_main_menu.system())
        )
        .add_system_set(
            SystemSet::on_resume(AppState::MainMenu)
                .with_system(setup_ui_main_menu.system())
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(cleanup_ui_main_menu.system())
        )

        // Finallly run the `App`
        .run()
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Starting up"); 

    commands.spawn_bundle(UiCameraBundle::default());
    //TODO: dont forget 2d camera

    // Load fonts
    asset_server.load_untyped("fonts/OpenSans-Regular.ttf");
    asset_server.load_untyped("fonts/OpenSans-Bold.ttf");

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
                    color: Color::hex(SECONDARY_TEXT_COLOR).unwrap()
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
                            color: Color::hex(SECONDARY_TEXT_COLOR).unwrap(),
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

fn change_state_menu(mut app_state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::M) {
        info!("M was pressed");

        app_state.set(AppState::MainMenu).unwrap();
    }
}

fn change_state_game(mut app_state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::G) {
        info!("G was pressed");

        app_state.set(AppState::InGame).unwrap();
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

fn setup_ui_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("MainMenu setup");

    // Title Text
    commands
        .spawn_bundle(TextBundle{
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {left: Val::Px(200.), right: Val::Px(200.), top: Val::Px(50.), ..Default::default()},
                ..Default::default()
            },
            text: Text::with_section(
                GAME_TITLE,
                TextStyle {
                    font: asset_server.get_handle("fonts/OpenSans-Bold.ttf"),
                    font_size: 60.,
                    color: Color::hex(PRIMARY_TEXT_COLOR).unwrap()
                },
                Default::default()
            ),
            ..Default::default()
        })
        .insert(TitleText);
}

fn cleanup_ui_main_menu(mut commands: Commands, query: Query<Entity, With<TitleText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
