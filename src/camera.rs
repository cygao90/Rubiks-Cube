use bevy::{input::{mouse::{MouseButtonInput, MouseMotion}, ButtonState}, prelude::*, window::CursorGrabMode};

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

pub fn handle_view(
    mut windows: Query<&mut Window>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera: Query<&mut Transform, With<Camera>>
) {
    let mut window = windows.single_mut();

    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Right {
            if event.state == ButtonState::Pressed {
                info!("right button pressed");
                window.cursor.visible = false;
                window.cursor.grab_mode = CursorGrabMode::Locked;
            } else if event.state == ButtonState::Released {
                info!("right button released");
                window.cursor.visible = true;
                window.cursor.grab_mode = CursorGrabMode::None;
            }
        }
    }

    for event in mouse_motion_events.read() {
        if window.cursor.grab_mode == CursorGrabMode::Locked {
            info!("{:?}", event);
            process_rotation(&mut camera.single_mut(), &event.delta);
        }
    }
    mouse_motion_events.clear();
}

fn process_rotation(camera: &mut Transform, delta: &Vec2) {
    let c = 0.005;
    if delta.x.abs() <= delta.y.abs() {
        let quat_y = Quat::from_euler(
            EulerRot::XYZ,
            -delta.y * c * camera.translation.z,
            0.0,
            delta.y * c * camera.translation.x,
        );
        camera.rotate_around(Vec3::ZERO, quat_y);
    } else {
        let quat_x = Quat::from_euler(
            EulerRot::XYZ,
            0.0,
            delta.x * c,
            0.0
        );
        camera.rotate_around(Vec3::ZERO, quat_x);
    }
}