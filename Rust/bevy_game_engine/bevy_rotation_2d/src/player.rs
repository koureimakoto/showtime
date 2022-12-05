use bevy::prelude::Component;

type Speed = f32;

#[derive(Component)]
pub struct Data {
    pub translation_speed: Speed,
    pub rotation_speed: Speed,
}

#[derive(Component)]
pub struct SnapToPlayer;

#[derive(Component)]
pub struct RotateToPlayer {
    pub rotation_speed: Speed
}
