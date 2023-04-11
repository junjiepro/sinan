use sinan::prelude::*;

fn main() {
    App::new()
        .bevy_app
        .add_system(hello_world_system)
        .run();
}

fn hello_world_system() {
    println!("hello world");
}
