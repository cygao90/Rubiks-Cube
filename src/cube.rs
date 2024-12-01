use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::{prelude::*, tasks::futures_lite::io::Empty};
use bevy::color::palettes::css::{self, BLACK};
use std::collections::HashMap;
use std::default;

#[derive(Component)]
pub struct Rotator;

#[derive(Clone, Copy, PartialEq)]
pub enum Layer {
    Left,
    M,
    Right,
    Up,
    E,
    Down,
    Front,
    S,
    Back,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Movement {
    layer: Layer,
    direction: Direction,
}

#[derive(Component)]
pub struct Cube {
    gap: f32,
    coord: [u32; 3],
    colors: HashMap<Face, Color>,
}

#[derive(Clone, Copy)]
pub struct Settings {
    layers: u32,
    color_up: Color,
    color_down: Color,
    color_left: Color,
    color_right: Color,
    color_front: Color,
    color_back: Color,
    color_beleved: Color,
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

impl Default for Settings {
    fn default() -> Self {
        Settings {
            layers: 3,
            color_down: Color::Srgba(css::WHITE),
            color_front: Color::Srgba(css::GREEN),
            color_up:  Color::Srgba(css::YELLOW),
            color_left: Color::Srgba(css::RED),
            color_back: Color::Srgba(css::BLUE),
            color_right: Color::Srgba(css::ORANGE),
            color_beleved: Color::Srgba(css::BLACK)
        }
    }
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

pub fn setup_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let rotator = commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Rotator,
    )).id();

    let settings = Settings::default();
    let layers = settings.layers;

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
                        transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
                        ..default()
                    },
                    cube,
                ))
                .id();

                commands.entity(rotator).push_children(&[id]);

            }
        }
    }
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
