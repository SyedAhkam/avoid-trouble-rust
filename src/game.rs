use bevy::{
    app::{Plugin, AppBuilder},
    ecs::system::{IntoSystem, Commands, Res, ResMut},
    asset::{Assets, AssetServer},
    sprite::ColorMaterial,
    render::{
        entity::OrthographicCameraBundle,
        color::Color
    },
    text::{
        prelude::{VerticalAlign, HorizontalAlign},
        Text,
        TextStyle,
        Text2dBundle,
        TextAlignment
    }
};

use crate::{
    player::Player,
    obstacle::Obstacle
};

pub struct Position {
    x: i32,
    y: i32
}

impl Default for Position {
    fn default() -> Position {
        Self { x: 0, y: 0 }
    }
}

struct StageCounter {
    stage: i32
}

impl Default for StageCounter {
    fn default() -> StageCounter {
        Self { stage: 1 }
    }
}

pub struct Game {
    player: Player,
    obstacles: Vec<Obstacle>
}

impl Plugin for Game {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(StageCounter::default())
            .add_startup_system(setup.system())
            .add_system(start_game.system());
    }
}

impl Game {
    pub fn new() -> Game {
        Self {
            player: Player::default(),
            obstacles: vec!(Obstacle::default())
        }
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    println!("Game setup");

    // Setup camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Load fonts
    // let font_regular = asset_server.load("fonts/OpenSans-Regular.ttf");
    
    // Draw stage counter
    // commands.spawn_bundle(Text2dBundle {
        // text: Text::with_section(
                // "test",
                // TextStyle { font: font_regular, font_size: 60., color: Color::WHITE },
                // TextAlignment { vertical: VerticalAlign::Top, horizontal: HorizontalAlign::Center }),
        // ..Default::default()
    // });
}

fn start_game() {
    // println!("start")
}
