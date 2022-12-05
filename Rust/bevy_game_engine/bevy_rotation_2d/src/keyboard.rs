use bevy::{prelude::{KeyCode, Input, Res, Transform, Query, Vec3}};

use crate::{player, constants::{TIME_STEP, BOUNDS}};


fn process_rotation_input(input: &Res<Input<KeyCode>>) -> f32 {
    let mut buffer: f32 = 0.0;
    
    if input.pressed(KeyCode::Up) {
        buffer += 1.0;
    }
    
    if input.pressed(KeyCode::Down) {
        buffer -= 1.0;
    }

    buffer
}

fn process_translate_input(input: &Res<Input<KeyCode>>) -> f32 {
    if input.pressed(KeyCode::Space) {
        1.0
    } else {
        0.0
    }
}

pub fn
load(input: Res<Input<KeyCode>>, mut query: Query<(&player::Data, &mut Transform)>) {
    let (hero_ship, mut transform) = query.single_mut();

    let translation_factor = process_rotation_input(&input);
    let rotation_factor    = process_translate_input(&input);

    transform.rotate_z(rotation_factor * hero_ship.rotation_speed * TIME_STEP);

    let translation_direction: Vec3 = transform.rotation * Vec3::Y;
    let translation_distance : f32  = translation_factor * hero_ship.translation_speed * TIME_STEP;
    let translation_delta    : Vec3 = translation_direction * translation_distance;

    // Preserva a operação
    transform.translation += translation_delta;

    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);

}
