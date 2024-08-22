use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};

use std::f32::consts::PI;

use crate::AppState;
use crate::resources::assets::TextureAssets;

pub struct RenderingPlugins;

impl Plugin for RenderingPlugins {
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(AppState::Playing3D), Self::create);
    }
}

#[derive(Component)]
struct CubeBoard;

impl RenderingPlugins {
    fn create(mut commands: Commands,
              mut meshes: ResMut<Assets<Mesh>>,
              mut materials: ResMut<Assets<StandardMaterial>>,
              mut assets: Res<TextureAssets>,

    ) {
        let size = Extent3d {
            width: 512,
            height: 512,
            ..default()
        };

        let image_handle = assets.covered_tile.clone();

        // Light
        // NOTE: we add the light to both layers so it affects both the rendered-to-texture cube, and the cube on which we display the texture
        // Setting the layer to RenderLayers::layer(0) would cause the main view to be lit, but the rendered-to-texture cube to be unlit.
        // Setting the layer to RenderLayers::layer(1) would cause the rendered-to-texture cube to be lit, but the main view to be unlit.
        commands.spawn((
            PointLightBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
                ..default()
            },
            RenderLayers::layer(0).with(1),
        ));

        let cube_size = 4.0;
        let cube_handle = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));

        // This material has the texture that has been rendered.
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(image_handle),
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
        commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    }
}