use bevy::prelude::*;

pub fn setup(mut cmd: Commands) {
    cmd.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb_u8(25, 25, 25)),
            ..default()
        },
    ));
}
