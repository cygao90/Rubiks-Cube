use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::{prelude::*, tasks::futures_lite::io::Empty};
use bevy::color::palettes::css;
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
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Face {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    FRONT,
    BACK
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
            color_right: Color::Srgba(css::ORANGE)
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

        Cube {
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
    let ind = vec![
        // top (facing towards +y)
        [-0.45, 0.45, -0.45], // vertex with index 0
        [0.45, 0.45, -0.45], // vertex with index 1
        [0.45, 0.45, 0.45], // etc. until 23
        [-0.45, 0.45, 0.45],
        // bottom   (-y)
        [-0.45, -0.45, -0.45],
        [0.45, -0.45, -0.45],
        [0.45, -0.45, 0.45],
        [-0.45, -0.45, 0.45],
        // right    (+x)
        [0.45, -0.45, -0.45],
        [0.45, -0.45, 0.45],
        [0.45, 0.45, 0.45], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
        [0.45, 0.45, -0.45],
        // left     (-x)
        [-0.45, -0.45, -0.45],
        [-0.45, -0.45, 0.45],
        [-0.45, 0.45, 0.45],
        [-0.45, 0.45, -0.45],
        // back     (+z)
        [-0.45, -0.45, 0.45],
        [-0.45, 0.45, 0.45],
        [0.45, 0.45, 0.45],
        [0.45, -0.45, 0.45],
        // forward  (-z)
        [-0.45, -0.45, -0.45],
        [-0.45, 0.45, -0.45],
        [0.45, 0.45, -0.45],
        [0.45, -0.45, -0.45],
    ];

    let color_up = cube.colors.get(&Face::UP).unwrap().to_linear().to_f32_array();
    let color_down = cube.colors.get(&Face::DOWN).unwrap().to_linear().to_f32_array();
    let color_right = cube.colors.get(&Face::RIGHT).unwrap().to_linear().to_f32_array();
    let color_left = cube.colors.get(&Face::LEFT).unwrap().to_linear().to_f32_array();
    let color_front = cube.colors.get(&Face::FRONT).unwrap().to_linear().to_f32_array();
    let color_back = cube.colors.get(&Face::BACK).unwrap().to_linear().to_f32_array();

    let colors = vec![
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

    let normals = vec![
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

    let indices = Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ]);

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, ind)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_indices(indices)
}
