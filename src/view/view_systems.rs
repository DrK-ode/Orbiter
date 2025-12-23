use bevy::prelude::*;

pub fn view_setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(0., 0., 10.),
    ));
    commands.spawn((Name::new("Lighting"), DirectionalLight::default()));
}

pub fn follow_player() {}
