use bevy::prelude::*;

pub fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(6.0, 6.0, 6.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}