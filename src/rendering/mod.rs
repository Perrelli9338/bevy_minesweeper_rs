pub(crate) mod tile_cube;
pub(crate) mod FaceIndex;

use crate::rendering::FaceIndex::FaceIndex as faceindex;
use bevy_mod_picking::prelude::*;
use std::collections::{HashMap, HashSet};
use bevy::{
    prelude::*,
    rendering::tile_cube::TileCube,
    render::{
        render_resource::Extent3d,
        view::RenderLayers,
    },
    resources::{
        board::{Board, Board3D},
        tile::Tile,
        settings::GameSettings,
        events::{FaceTriggerEvent, GameLoseEvent, GameWinEvent}
    },
    color::palettes::basic,
    input::touch::TouchPhase,
    window::PrimaryWindow,
    components::{Bomb, BombNeighbor, Coordinates,
                 uncover::Uncover}
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use crate::{
    AppState,
    resources::assets::TextureAssets
};
use std::f32::consts::PI;
use bevy::color::palettes::basic;
use bevy::input::touch::TouchPhase;
use bevy::window::PrimaryWindow;
use crate::components::{Bomb, BombNeighbor, Coordinates};
use crate::components::uncover::Uncover;
use crate::rendering::tile_cube::TileCube;
use crate::resources::assets::FontAssets;
use crate::resources::board::{Board, Board3D, FlagToggle};
use crate::resources::events::{FaceTriggerEvent, GameLoseEvent, GameWinEvent};
use crate::resources::settings::GameSettings;
use crate::resources::tile::Tile;
use crate::resources::tile_map::TileMap;

pub struct RenderingPlugins;

#[derive(Component)]
pub struct CubeBoard;

struct TileCubeSize {
    width: f32,
    height: f32,
}

impl Plugin for RenderingPlugins {
    fn build(&self, app: &mut App){
        app.add_plugins(PanOrbitCameraPlugin).add_plugins(DefaultPickingPlugins)
            .add_systems(OnEnter(AppState::Playing3D), (toggle2_dcamera, Self::setup))
            .add_systems(Update, (check_input_cube, uncover_face).run_if(in_state(AppState::Playing3D)));
    }
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
    fn setup(mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             mut assets: Res<TextureAssets>,
             config: Res<GameSettings>) {
        let mut safe_start: Option<Entity> = None;
        let mut tile_cube = tile_cube::TileCube::new(3);
        let tile_size = TileCubeSize {
            width: 0.8f32,
            height: 0.8f32,
        };
        let tile_handle = meshes.add(Plane3d::default().mesh().size(tile_size.width, tile_size.height));
        let covered_handle = meshes.add(Plane3d::default().mesh().size(tile_size.width + 0.0001, tile_size.height + 0.0001));

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

        let mut covered_tiles = HashMap::with_capacity(6);
        commands.spawn((Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
                        PanOrbitCamera {
                            pan_sensitivity: 0.0,
                            zoom_upper_limit: Some(7.),
                            zoom_lower_limit: Some(2.),
                            ..default()
                        }));
        let bomb_count = 4;
        let mut tile_cube = TileCube::new(bomb_count);
        tile_cube.set_bombs(bomb_count);
        let transforms = vec![
            Transform::from_matrix(Mat4::from_rotation_translation(Quat::from_rotation_x(0.0), Vec3::new(0.0, 0.4f32, 0.0))),
            Transform::from_matrix(Mat4::from_rotation_translation(Quat::from_rotation_x(PI), Vec3::new(0.0, -0.4f32, 0.0), )),
            Transform::from_matrix(Mat4::from_rotation_translation(Quat::from_rotation_z(-PI / 2.0), Vec3::new(0.4f32, 0.0, 0.0))),
            Transform::from_matrix(Mat4::from_rotation_translation(Quat::from_rotation_z(PI / 2.0), Vec3::new(-0.4f32, 0.0, 0.0), )),
            Transform::from_matrix(Mat4::from_rotation_translation(Quat::from_rotation_x(PI / 2.0), Vec3::new(0.0, 0.0, 0.4f32), )),
            Transform::from_matrix(Mat4::from_rotation_translation(Quat::from_rotation_x(-PI / 2.0), Vec3::new(0.0, 0.0, -0.4f32)))];
        let e = commands.spawn(SpatialBundle::default()).with_children(|child| {
            Self::generate(&tile_cube, transforms, child, tile_handle, covered_handle, material_tile, material_covered, material_bomb, materials, assets,
                           &mut covered_tiles,
                               &mut safe_start);
        }).id();
        if config.easy_mode {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }
        commands.insert_resource(Board3D {
            tile_cube: tile_cube.clone(),
            covered_tiles,
            flagged_tiles: HashSet::new(),
            entity: e,
        });
    }
    #[allow(clippy::too_many_arguments)]
    fn generate(
        tile_cube: &TileCube,
        transform: Vec<Transform>,
        child: &mut ChildBuilder,
        tile_handle: Handle<Mesh>,
        covered_handle: Handle<Mesh>,
        material_tile: Handle<StandardMaterial>,
        material_covered: Handle<StandardMaterial>,
        material_bomb: Handle<StandardMaterial>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut assets: Res<TextureAssets>,
        covered_tiles: &mut HashMap<u16, Entity>,
        safe_start: &mut Option<Entity>,
    ){
        for (index, (face, transform)) in tile_cube.iter().copied().zip(transform.iter()).enumerate()
        {
            let number = faceindex {
                i: index as u16,
            };
            let mut commands = child.spawn((
                MaterialMeshBundle {
                    mesh: tile_handle.clone(),
                    material: material_tile.clone(),
                    transform: *transform,
                    ..Default::default()
                },
            ));

            commands.insert(number);

            commands.with_children(|child| {
                let e = child.spawn((
                    MaterialMeshBundle {
                        mesh: covered_handle.clone(),
                        material: material_covered.clone(),
                        transform: Transform::from_xyz(0.0, 0.0001, 0.0),
                        ..Default::default()
                    },
                    On::<Pointer<Click>>::commands_mut(move |event, commands| {
                        commands.entity(event.target).insert(CubeBoard);
                    })
                )).id();
                covered_tiles.insert(number.i, e);
                if safe_start.is_none() && face == Tile::Empty{
                    *safe_start = Some(e);
                }

            });

            match face {
                Tile::Bomb => {
                    commands.insert(Bomb);
                    commands.with_children(|child| {
                        child.spawn((
                            MaterialMeshBundle {
                                mesh: tile_handle.clone(),
                                material: material_bomb.clone(),
                                transform: Transform::from_xyz(0.0, 0.00001, 0.0),
                                ..Default::default()
                            },
                        ));
                    });
                }
                Tile::BombNeighbour(bombs_counter) => {
                    commands.insert(BombNeighbor{count: bombs_counter});
                    let (texture_bomb_neighbour, color_bomb) = match (bombs_counter) {
                        1 => (assets.bomb_neighbour_1.clone(), Color::from(basic::BLUE)),
                        2 => (assets.bomb_neighbour_2.clone(), Color::from(basic::GREEN)),
                        3 => (assets.bomb_neighbour_3.clone(), Color::from(basic::RED)),
                        4 => (assets.bomb_neighbour_4.clone(), Color::from(basic::NAVY)),
                        5 => (assets.bomb_neighbour_5.clone(), Color::from(basic::MAROON)),
                        6 => (assets.bomb_neighbour_6.clone(), Color::from(basic::AQUA)),
                        7 => (assets.bomb_neighbour_7.clone(), Color::from(basic::PURPLE)),
                        8 => (assets.bomb_neighbour_8.clone(), Color::from(basic::GRAY)),
                        _ => (assets.bomb_neighbour_1.clone(), Color::from(basic::BLUE)),
                    };
                    let material_bomb_neighbour = materials.add(StandardMaterial {
                        base_color_texture: Some(texture_bomb_neighbour),
                        base_color: color_bomb,
                        reflectance: 0.02,
                        unlit: false,
                        alpha_mode: AlphaMode::Mask(1.),
                        flip_normal_map_y: true,
                        ..default()
                    });
                    commands.with_children(|child| {
                        child.spawn((
                            MaterialMeshBundle {
                                mesh: tile_handle.clone(),
                                material: material_bomb_neighbour.clone(),
                                transform: Transform::from_xyz(0.0, 0.00001, 0.0),
                                ..Default::default()
                            },
                        ));
                    });
                }
                _ => {}
            }
            }
    }
}
fn uncover_face(
    mut commands: Commands,
    query: Query<(Entity, &Parent), With<Uncover>>,
    mut board: ResMut<Board3D>,
    parents: Query<(&faceindex, Option<&Bomb>, Option<&BombNeighbor>)>,
) {
    for (e, parent) in query.iter() {
        commands.entity(e).despawn_recursive();
        let (coordinates, bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(_e) => {
                continue;
            }
        };
        if let Some(_) = board.try_uncover_tile(*coordinates) {
            if bomb.is_some() {
                for entity in board.uncover_bomb() {
                    commands.entity(entity).insert((Uncover, Bomb));
                }
                trigger_evr.send(GameLoseEvent);
            } else if bomb_counter.is_none() {
                for entity in board.uncover_tile_neighbour(*coordinates) {
                    commands.entity(entity).insert(Uncover);
                }
            }
        }
    }
}

fn check_input_cube(
    mut commands: Commands,
    window_primary_query: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    query: Query<(Entity, &Parent), With<CubeBoard>>,
    mut board: ResMut<Board3D>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    parents: Query<&faceindex>,
) {
    if query.is_empty() {
        return;
    }
    let Ok(window) = window_primary_query.get_single() else { return };
    let mut fingers = Vec::new();
    for finger in touch_input.iter() {
        if touch_input.just_pressed(finger.id()) {
            fingers.push(finger);
        }
    }
    if mouse_input.just_pressed(MouseButton::Left) || (fingers.len() >= 0 || ingers.len() < 2) {
        for e in query.iter() {
            commands.entity(e).remove::<CubeBoard>();
            commands.entity(e).insert(Uncover);
        }
    }
    if mouse_input.just_pressed(MouseButton::Right) || fingers.len() >= 2 {
        for (e, parent) in query.iter() {
            let (coordinates) = match parents.get(parent.get()) {
                Ok(v) => v,
                Err(_e) => {
                    continue;
                }
            };
            match board.try_toggle_flag(*coordinates) {
                FlagToggle::FlagIsSet(e) => {
                    let material_flag = materials.add(StandardMaterial {
                        base_color_texture: Some(assets.flag.clone()),
                        base_color: Color::from(basic::RED),
                        reflectance: 0.02,
                        unlit: false,
                        alpha_mode: AlphaMode::Mask(1.),
                        flip_normal_map_y: true,
                        ..default()
                    });

                    let tile_size = TileCubeSize {
                        width: 0.8f32,
                        height: 0.8f32,
                    };
                    let tile_handle = meshes.add(Plane3d::default().mesh().size(tile_size.width, tile_size.height));
                    commands.entity(e).with_children(|parent| {
                            parent.spawn((
                                MaterialMeshBundle {
                                    mesh: tile_handle.clone(),
                                    material: material_flag.clone(),
                                    transform: Transform::from_xyz(0.0, 0.00001, 0.0),
                                    ..Default::default()
                                },
                                On::<Pointer<Click>>::commands_mut(move |event, commands| {
                                    commands.entity(event.target).insert(CubeBoard);
                                })
                            ));
                    });
                }
                FlagToggle::FlagIsUnset(e) => {
                    let (child, _) = match query.get(e) {
                        Ok(value) => value,
                        Err(_e) => continue,
                    };
                    commands.entity(child).despawn_recursive();
                },
                _ => (),
            }
            commands.entity(e).remove::<CubeBoard>();
        }
    }
}
