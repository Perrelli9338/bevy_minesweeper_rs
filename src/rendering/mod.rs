pub(crate) mod tile_cube;

use bevy::{
    prelude::*,
    render::{
        render_resource::Extent3d,
        view::RenderLayers,
    },
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use crate::{
    AppState,
    resources::assets::TextureAssets
};
use std::f32::consts::PI;
use bevy::color::palettes::basic;

pub struct RenderingPlugins;

impl Plugin for RenderingPlugins {
    fn build(&self, app: &mut App){
        app.add_plugins(PanOrbitCameraPlugin)
            .add_systems(OnEnter(AppState::Playing3D), (toggle2_dcamera, Self::create).chain());
    }
}

#[derive(Component)]
struct CubeBoard;

struct TileCubeSize {
    width: f32,
    height: f32,
}

fn toggle2_dcamera(
    mut q: Query<&mut Camera>,
) {
    let mut camera = q.single_mut();
    if camera.is_active {
        camera.is_active = false;
    } else {
        camera.is_active = true;
    }
}

impl RenderingPlugins {
    fn create(mut commands: Commands,
              mut meshes: ResMut<Assets<Mesh>>,
              mut materials: ResMut<Assets<StandardMaterial>>,
              mut assets: Res<TextureAssets>,

    ) {
        let tile_size = TileCubeSize {
            width: 0.8f32,
            height: 0.8f32,
        };
        let tile_handle = meshes.add(Plane3d::default().mesh().size(tile_size.width, tile_size.height));
        let covered_handle = meshes.add(Plane3d::default().mesh().size(tile_size.width + 0.001, tile_size.height + 0.001));

        // This material has the texture that has been rendered.

        let material_covered = materials.add(StandardMaterial {
            base_color_texture: Some(assets.covered_tile.clone()),
            reflectance: 0.02,
            base_color: Color::from(basic::AQUA),
            unlit: false,
            ..default()
        });

        let material_tile = materials.add(StandardMaterial {
            base_color_texture: Some(assets.tile.clone()),
            reflectance: 0.02,
            base_color: Color::from(basic::WHITE),
            unlit: false,
            ..default()
        });

        let material_bomb = materials.add(StandardMaterial {
            base_color_texture: Some(assets.bomb.clone()),
            base_color: Color::from(basic::RED),
            reflectance: 0.02,
            unlit: false,
            ..default()
        });

        let material_flag = materials.add(StandardMaterial {
            base_color_texture: Some(assets.flag.clone()),
            base_color: Color::from(basic::RED),
            reflectance: 0.02,
            unlit: false,
            ..default()
        });

        // Main pass cube, with material containing the rendered first pass texture.
        commands.spawn((
            PbrBundle {
                mesh: cube_handle,
                material: material_handle,
                transform: Transform::from_xyz(0.0, 0.0, 1.5),
                ..default()
            },
            MainPassCube,
        ));

        // The main pass camera.
        commands.spawn((Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            pan_sensitivity: 0.0,
            zoom_upper_limit: Some(7.),
            zoom_lower_limit: Some(2.),
            ..default()
        }));
    }
}