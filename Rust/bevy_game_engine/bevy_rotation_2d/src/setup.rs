use bevy::{prelude::{
        Commands,
        Res,
        AssetServer,
        Camera2dBundle, Handle, Image, default, Transform
    }, sprite::SpriteBundle}
;

use crate::{constants::BOUNDS, player::{Data, SnapToPlayer, RotateToPlayer}};

pub fn
load(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hero_ship     : Handle<Image> = asset_server.load::<Image, &str>("textures/hero_ship.png"     );
    let rich_monster  : Handle<Image> = asset_server.load::<Image, &str>("textures/rich_monster.png"  );
    let casual_monster: Handle<Image> = asset_server.load::<Image, &str>("textures/casual_monster.png");
    
    let horizontal_margin: f32 = BOUNDS.x / 4.0;
    let vertical_margin  : f32 = BOUNDS.y / 4.0;
    
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: hero_ship,
            ..default()
        },
        Data {
            translation_speed: 500.0,
            rotation_speed   : f32::to_radians(360.0)
        }
    ));

    commands.spawn((
        SpriteBundle {
            texture  : rich_monster.clone(),
            transform: Transform::from_xyz(0.0 - horizontal_margin, 0.0, 0.0),
            ..default()
        }, 
        SnapToPlayer
    ));

    commands.spawn((
        SpriteBundle {
            texture  : rich_monster.clone(),
            transform: Transform::from_xyz(0.0 - horizontal_margin/2.0, 0.0 - vertical_margin/2.0, 0.0),
            ..default()
        }, 
        SnapToPlayer
    ));

    commands.spawn((
        SpriteBundle {
            texture  : rich_monster.clone(),
            transform: Transform::from_xyz(0.0, 0.0 - vertical_margin, 0.0),
            ..default()
        }, 
        SnapToPlayer
    ));

    commands.spawn((
        SpriteBundle {
            texture  : rich_monster,
            transform: Transform::from_xyz(0.0 + horizontal_margin/2.0, 0.0 - vertical_margin/2.0, 0.0),
            ..default()
        }, 
        SnapToPlayer
    ));

    commands.spawn((
        SpriteBundle {
            texture  : casual_monster.clone(),
            transform: Transform::from_xyz(0.0 - horizontal_margin/2.0, 0.0 + vertical_margin/2.0, 0.0),
            ..default()
        }, 
        RotateToPlayer {
            rotation_speed: f32::to_radians(45.5)
        }
    ));

    commands.spawn((
        SpriteBundle {
            texture  : casual_monster.clone(),
            transform: Transform::from_xyz(0.0 + horizontal_margin, 0.0, 0.0),
            ..default()
        }, 
        RotateToPlayer {
            rotation_speed: f32::to_radians(45.5)
        }
    ));

    commands.spawn((
        SpriteBundle {
            texture  : casual_monster.clone(),
            transform: Transform::from_xyz(0.0 + horizontal_margin/2.0, 0.0 + vertical_margin/2.0, 0.0),
            ..default()
        }, 
        RotateToPlayer {
            rotation_speed: f32::to_radians(45.5)
        }
    ));

    commands.spawn((
        SpriteBundle {
            texture  : casual_monster,
            transform: Transform::from_xyz(0.0, 0.0 + vertical_margin, 0.0),
            ..default()
        }, 
        RotateToPlayer {
            rotation_speed: f32::to_radians(45.5)
        }
    ));

}
