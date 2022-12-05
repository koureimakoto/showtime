pub mod first_test {
    use bevy::prelude::{
        Query,
        Component,
        Transform,
    };

// Definição de componentes para Beny, implementações da Trait Component
#[derive(Component)]
pub struct Position {x: f32, y: f32}

// Definição de sistemas para o Beny. Funções e operações Rust.
pub fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        print!("position: {:?}", transform.translation);
    }
}

// Definição de entidade, ou entidade unica no Beny.
struct Entity(f64);

pub fn
hello_bevy() {
    println!("Hello Beny Engine")
}
 

}

pub mod components_test {
    use bevy::{prelude::{Component, Commands, Query, With, Res, Resource, ResMut}, time::{Time, Timer}};


    #[derive(Component)]
    pub struct Person;

    #[derive(Component)]
    pub struct Name(String);


    pub fn
    add_people(mut commands: Commands) {
        commands.spawn((Person, Name("Renan Calheiro de Ferro".to_string())));
        commands.spawn((Person, Name("Maria Aparecida da Costa Silva Silva".to_string())));
        commands.spawn((Person, Name("Perfumada Sofista Conservadora".to_string())));
    }

    #[derive(Resource)]
    pub struct GreetTimer(pub Timer);

    pub fn
    greet_people(
        time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>
    ) {
        if timer.0.tick(time.delta()).just_finished() {
            query.iter().for_each(
                |name| println!("hello {}!", name.0)
            );
        }
        
        
    }

}


pub mod
my_first_plugin {
    use bevy::{prelude::Plugin, time::{TimerMode, Timer}};

    use crate::components_test::{GreetTimer, add_people, greet_people};

    pub struct HelloPlugin;

    impl Plugin for HelloPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app
                .insert_resource(
                    GreetTimer(
                        Timer::from_seconds(2.0, TimerMode::Repeating))
                    )
                .add_startup_system(add_people)
                .add_system(greet_people);
        }
    }
}