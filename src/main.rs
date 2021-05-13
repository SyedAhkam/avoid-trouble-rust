use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}
};

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
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        // Game startup system
        .add_startup_system(startup.system())
        .run()
}

fn startup() {
    info!("Starting up");
}

