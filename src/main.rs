use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(set_up)
        .add_system(greet_tanks)
        .run();
}

#[derive(Component)]
struct Tank;

#[derive(Component)]
struct Name(String);

struct GreetTimer(Timer);

fn set_up(mut commands: Commands) {
    // 创建entities
    commands
        .spawn()
        .insert(Tank)
        .insert(Name("tank1".to_string()));
    commands
        .spawn()
        .insert(Tank)
        .insert(Name("tank2".to_string()));
}

fn greet_tanks(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Tank>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

fn move_tank() {
    
}
