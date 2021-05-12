use bevy::app::Plugin;
use bevy::app::AppBuilder;
use bevy::ecs::system::{IntoSystem, Commands, Res, ResMut};
use bevy::asset::{Assets, AssetServer};
use bevy::sprite::ColorMaterial;
use bevy::render::entity::OrthographicCameraBundle;

use crate::player::Player;
use crate::obstacle::Obstacle;

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
    println!("setup");

    // Setup camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn start_game() {
    println!("start")
}
