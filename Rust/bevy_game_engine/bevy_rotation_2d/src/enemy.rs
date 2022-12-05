use bevy::{prelude::{Query, Transform, Without, With, Quat, Vec3, Vec2}, math::Vec3Swizzles};

use crate::{player::{SnapToPlayer, Data, RotateToPlayer}, constants::TIME_STEP};

pub fn
snap_to_player(
    mut enemy_query: Query<&mut Transform, (With<SnapToPlayer>, Without<Data>)>,
    player_query: Query<&Transform, With<Data>> 
) {
    let hero_ship_transform = player_query.single();
    let hero_ship_translation = hero_ship_transform.translation.xy();

    for mut enemy_transform in &mut enemy_query {
        let enemy_translation = enemy_transform.translation.xy();
        let to_player = (hero_ship_translation - enemy_translation).normalize();

        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));
        enemy_transform.rotation = rotate_to_player;
    }
}

pub fn
rotate_to_player_system(
    mut enemy_query: Query<(&RotateToPlayer, &mut Transform), Without<Data>>,
    player_query: Query<&Transform, With<Data>>,
) {
    let hero_ship_transform = player_query.single();
    let hero_ship_translation = hero_ship_transform.translation.xy();

    for (config, mut enemy_transform) in &mut enemy_query {
        let enemy_translation = enemy_transform.translation.xy();
        let to_player = (hero_ship_translation - enemy_translation).normalize();

        let forward_dot_player = (enemy_transform.rotation * Vec3::Y).xy().dot(to_player);

        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

        let right_dot_player = (enemy_transform.rotation * Vec3::X).xy().dot(to_player);
        let rotation_sign = -f32::copysign(1.0, right_dot_player);


        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos(); 

        let rotation_angle = rotation_sign * (config.rotation_speed * TIME_STEP).min(max_angle);

        enemy_transform.rotate_z(rotation_angle);
    }
}
