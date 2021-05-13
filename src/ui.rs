use bevy::{
    app::{Plugin, AppBuilder},
    ecs::{
        system::{
            IntoSystem,
            Commands,
            Res,
            Query
        },
        query::With
    },
    asset::{Assets, AssetServer},
    ui::{
        entity::{UiCameraBundle, TextBundle},
        Style,
        AlignSelf
    },
    text::{
        Text,
        TextSection,
        TextStyle,
        TextAlignment
    },
    render::color::Color,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}
};

pub struct Ui;

impl Ui {
    pub fn new() -> Ui {
        Self {}
    }
}

impl Plugin for Ui {
   fn build(&self, app: &mut AppBuilder) {
    app.add_startup_system(setup.system())
        .add_system(update_fps.system());
   }
}

struct FpsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("UI Setup");

    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());

    // Add FPS counter
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: "FPS: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                        font_size: 15.,
                        color: Color::WHITE
                    }
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                        font_size: 15.,
                        color: Color::WHITE
                    }
                }
            ],
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(FpsText);

    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text::with_section(
                "testing",
                TextStyle {
                    font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                    font_size: 15.,
                    color: Color::WHITE
                },
                TextAlignment::default()
            ),
        ..Default::default()
    });

    // Add stage counter
}

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}
