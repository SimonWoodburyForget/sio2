mod framerate;
mod render;
mod grid;
mod input;
mod physics;

use crate::{
    input::{ ToolState, InputState, spawn_particle, handle_input},
    render::{ GridTexture, grid_scale, grid_render },
    grid::Grid,
    framerate::display_framerate,
    physics::grid_update
};

use bevy::{
    diagnostic::{ FrameTimeDiagnosticsPlugin },
    prelude::*,
    render::texture::{ TextureFormat }
};

const WINDOW_HEIGHT : u32 = 800;
const WINDOW_WIDTH : u32 = 800;
const FIELD_WIDTH : usize = 200;
const FIELD_HEIGHT : usize = 200;
const FIELD_WIDTH_F32 : f32 = FIELD_WIDTH as f32;
const FIELD_HEIGHT_F32 : f32 = FIELD_HEIGHT as f32;

// TODO: this struct can be replaced with bevy::prelude::Color
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Pixel {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Pixel {
            r,
            g,
            b,
            a
        }
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "SiO2".to_string(),
            width: WINDOW_HEIGHT,
            height: WINDOW_WIDTH,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(grid_render.system())
        .add_system(grid_scale.system())
        .add_system(grid_update.system())
        .add_system(display_framerate.system())
        .add_system(handle_input.system())
        .add_system(spawn_particle.system())
        .run();
}

fn setup(mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>) {

    let font = asset_server
        .load("assets/fonts/FiraSans-Bold.ttf")
        .unwrap();

    let texture = Texture::new_fill(
        Vec2::new(FIELD_WIDTH_F32, FIELD_HEIGHT_F32),
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm
    );
    let th = textures.add(texture);

    commands.spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: font,
                style: TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        })
        .insert_resource(Grid::default())
        .insert_resource(ToolState::default())
        .insert_resource(InputState::default())
        .spawn(SpriteComponents {
            material: materials.add(th.into()),
            transform: Transform::from_translation_rotation_scale(
                Vec3::new(0., 0., 0.),
                Quat::identity(),
                WINDOW_WIDTH as f32 / FIELD_WIDTH_F32
            ),
            ..Default::default()
        })
        .with(GridTexture);
}