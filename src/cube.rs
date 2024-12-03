use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::prelude::*;
use std::collections::HashMap;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::backends::raycast::RaycastPickable;

use crate::{actions, Settings};
use bevy::color::palettes::css;

#[derive(Component)]
pub struct Rotator;

#[derive(Component)]
pub struct RotateX;

#[derive(Component)]
pub struct RotateY;

#[derive(Component)]
pub struct RotateZ;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    // facing the negative direction
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RotateAxis {
    X, Y, Z
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Movement {
    pub axis: RotateAxis,
    pub layer: u32,
    pub direction: Direction,
}

#[derive(Component)]
pub struct Cube {
    pub gap: f32,
    pub coord: [u32; 3],
    colors: HashMap<Face, Color>,
}

#[derive(Resource)]
pub struct CubeInfo {
    pub cubes: Vec<Entity>,
    pub x: Option<Entity>,
    pub y: Option<Entity>,
    pub z: Option<Entity>,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Face {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    FRONT,
    BACK,
    BEVELED,
}

impl Default for Cube {
    fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert(Face::LEFT, Color::BLACK);
        colors.insert(Face::RIGHT, Color::BLACK);
        colors.insert(Face::UP, Color::BLACK);
        colors.insert(Face::DOWN, Color::BLACK);
        colors.insert(Face::FRONT, Color::BLACK);
        colors.insert(Face::BACK, Color::BLACK); 
        colors.insert(Face::BEVELED, Color::BLACK);

        Cube {
            gap: 0.1,
            coord: [0; 3],
            colors: colors,
        }
    }
}

impl Cube {
    fn set_colors(&mut self, setting: &Settings) {
        if self.down_dace(setting) {
            self.colors.insert(Face::DOWN, setting.color_down);
        }
        if (self.up_face(setting)) {
            self.colors.insert(Face::UP, setting.color_up);
        }
        if (self.left_face(setting)) {
            self.colors.insert(Face::LEFT, setting.color_left);
        }
        if (self.right_face(setting)) {
            self.colors.insert(Face::RIGHT, setting.color_right);
        }
        if (self.back_face(setting)) {
            self.colors.insert(Face::BACK, setting.color_back);
        }
        if (self.front_face(setting)) {
            self.colors.insert(Face::FRONT, setting.color_front);
        }
        self.colors.insert(Face::BEVELED, setting.color_beleved);
    }

    #[allow(unused)]
    fn down_dace(&self, setting: &Settings) -> bool { self.coord[1] == 0 }
    fn up_face(&self, setting: &Settings) -> bool { self.coord[1] == setting.layers - 1 }
    #[allow(unused)]
    fn left_face(&self, setting: &Settings) -> bool { self.coord[0] == 0 }
    fn right_face(&self, setting: &Settings) -> bool { self.coord[0] == setting.layers - 1 }
    #[allow(unused)]
    fn back_face(&self, setting: &Settings) -> bool { self.coord[2] == 0 }
    fn front_face(&self, setting: &Settings) -> bool { self.coord[2] == setting.layers - 1 }
}

impl Default for CubeInfo {
    fn default() -> Self {
        CubeInfo {
            cubes: Vec::new(),
            x: None,
            y: None,
            z: None,
        }
    }
}

pub fn setup_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cube_info: ResMut<CubeInfo>,
    settings: Res<Settings>,
) {
    let rotator = commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Rotator,
    )).id();

    let layers = settings.layers;
    let center = layers as f32 / 2.0;

    for x in 0..layers {
        for y in 0..layers {
            for z in 0..layers {

                let mut cube = Cube::default();
                cube.coord = [x, y, z];
                cube.set_colors(&settings);

                let id = commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(create_mesh(&cube)),
                        material: materials.add(StandardMaterial::default()),
                        transform: Transform::from_xyz(
                            x as f32 - center + 0.5,
                            y as f32 - center + 0.5,
                            z as f32 - center + 0.5,
                        ),
                        ..default()
                    },
                    PickableBundle::default(),
                    RaycastPickable::default(),
                    On::<Pointer<DragStart>>::run(actions::handle_drag_start),
                    On::<Pointer<Move>>::run(actions::handle_drag_move),
                    On::<Pointer<DragEnd>>::run(actions::handle_drag_end),
                    HIGHLIGHT_TINT,
                    cube
                ))
                .id();

                commands.entity(rotator).push_children(&[id]);
                cube_info.cubes.push(id);

            }
        }
    }

    commands.entity(rotator).with_children(|parent| {
        let x = parent.spawn((
            TransformBundle {
                local: Transform::from_xyz(1.0, 0.0, 0.0),
                ..default()
            },
            RotateX
        )).id();
        let y = parent.spawn((
            TransformBundle {
                local: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
            },
            RotateY
        )).id();
        let z = parent.spawn((
            TransformBundle {
                local: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
            RotateZ
        )).id();

        cube_info.x = Some(x);
        cube_info.y = Some(y);
        cube_info.z = Some(z);
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Capsule3d {
            radius: 0.05,
            half_length: 10.0,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::Srgba(css::RED),
            ..Default::default()
        }),
        transform: Transform {
            translation: Vec3::new(2.5, 0.0, 0.0),
            rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2), // Rotate along Z-axis
            ..Default::default()
        },
        ..Default::default()
    });

    // Y-Axis (Green)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Capsule3d {
            radius: 0.05,
            half_length: 10.0,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::Srgba(css::GREEN),
            ..Default::default()
        }),
        transform: Transform {
            translation: Vec3::new(0.0, 2.5, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Z-Axis (Blue)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Capsule3d {
            radius: 0.05,
            half_length: 10.0,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::Srgba(css::BLUE),
            ..Default::default()
        }),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 2.5),
            rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2), // Rotate along X-axis
            ..Default::default()
        },
        ..Default::default()
    });
}

fn create_mesh(cube: &Cube) -> Mesh {

    let c = (1.0 - cube.gap) / 2.0;
    let c1 = 0.5;

    let mut vertices = vec![
        // top (facing towards +y)
        [-c, c1, -c], // vertex with index 0
        [c, c1, -c], // vertex with index 1
        [c, c1, c], // etc. until 23
        [-c, c1, c],
        // bottom   (-y)
        [-c, -c1, -c],
        [c, -c1, -c],
        [c, -c1, c],
        [-c, -c1, c],
        // right    (+x)
        [c1, -c, -c],
        [c1, -c, c],
        [c1, c, c], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
        [c1, c, -c],
        // left     (-x)
        [-c1, -c, -c],
        [-c1, -c, c],
        [-c1, c, c],
        [-c1, c, -c],
        // back     (+z) front
        [-c, -c, c1],
        [-c, c, c1],
        [c, c, c1],
        [c, -c, c1],
        // forward  (-z) back
        [-c, -c, -c1],
        [-c, c, -c1],
        [c, c, -c1],
        [c, -c, -c1],
    ];

    let color_up = cube.colors.get(&Face::UP).unwrap().to_linear().to_f32_array();
    let color_down = cube.colors.get(&Face::DOWN).unwrap().to_linear().to_f32_array();
    let color_right = cube.colors.get(&Face::RIGHT).unwrap().to_linear().to_f32_array();
    let color_left = cube.colors.get(&Face::LEFT).unwrap().to_linear().to_f32_array();
    let color_front = cube.colors.get(&Face::FRONT).unwrap().to_linear().to_f32_array();
    let color_back = cube.colors.get(&Face::BACK).unwrap().to_linear().to_f32_array();
    let color_beveled = cube.colors.get(&Face::BEVELED).unwrap().to_linear().to_f32_array();

    let mut colors = vec![
        color_up,
        color_up,
        color_up,
        color_up,

        color_down,
        color_down,
        color_down,
        color_down,

        color_right,
        color_right,
        color_right,
        color_right,

        color_left,
        color_left,
        color_left,
        color_left,

        color_front,
        color_front,
        color_front,
        color_front,

        color_back,
        color_back,
        color_back,
        color_back,
    ];

    let mut normals = vec![
        // Normals for the top side (towards +y)
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        // Normals for the bottom side (towards -y)
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        // Normals for the right side (towards +x)
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        // Normals for the left side (towards -x)
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        // Normals for the back side (towards +z)
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        // Normals for the forward side (towards -z)
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
    ];

    let mut indices: Vec<u32> = vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ];

    let beveled_corners: Vec<[u32; 3]> = vec![
        [0, 21, 15], // 24 25 26
        [1, 11, 22], // 27 28 29
        [2, 18, 10], // 30 31 32
        [3, 14, 17], // 33 34 35
        [4, 12, 20], // 36 37 38
        [5, 23, 8], // 39 40 41
        [6, 9, 19], // 42 43 44
        [7, 16, 13], // 45 46 47
    ];
    for v in beveled_corners {
        let corner_vertices: Vec<[f32; 3]> = v.iter().map(|&idx| vertices[idx as usize]).collect();
        let normals_: Vec<[f32; 3]> = v.iter().map(|&idx| normals[idx as usize]).collect();
        let corner_colors = vec![color_beveled; 3];
        let len: u32 = vertices.len() as u32;
        indices.extend_from_slice(&[len, len + 1, len + 2]);
        vertices.extend_from_slice(&corner_vertices);
        normals.extend_from_slice(&normals_);
        colors.extend_from_slice(&corner_colors);
    }

    let beveled_edges = vec![
        [24, 27, 29, 25],
        [27, 30, 32, 28],
        [24, 26, 34, 33],
        [30, 33, 35, 31],
        [36, 38, 40, 39],
        [42, 44, 46, 45],
        [36, 45, 47, 37],
        [43, 42, 39, 41],
        [38, 37, 26, 25],
        [28, 41, 40, 29],
        [31, 44, 43, 32],
        [34, 47, 46, 35],
    ];

    for edge in beveled_edges {
        let t1: Vec<u32> = [0, 1, 2].iter().map(|&idx| edge[idx]).collect();
        let t2: Vec<u32> = [0, 2, 3].iter().map(|&idx| edge[idx]).collect();
        indices.extend_from_slice(&t1);
        indices.extend_from_slice(&t2);
    }

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_indices(Indices::U32(indices))
}


const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl
            .base_color
            .mix(&Color::srgba(0.0, 0.0, 0.0, 0.8), 0.3),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl
            .base_color
            .mix(&Color::srgba(0.0, 0.0, 0.0, 0.8), 0.4),
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color,
        ..matl.to_owned()
    })),
};