use rand::Rng;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
// use wasm_bindgen::JsValue;

// use crate::invoke;




pub struct BevyInterfacePlugin;
impl Plugin for BevyInterfacePlugin {
    fn build(&self, app: &mut App) {
            app.add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, add_cube_system);
            // .add_systems(PostUpdate, tauri_listener);
    }
}



// How in the world can this work?.....
// Big issue with "invoke" (Tauri command call) being async and "JsValue" not being Send or something
// and not passing cleanly between Bevy and Tauri.
//
// Think cleanest approach would be to have a Bevy system/function continually
// check Tauri state for updates and then signal to other Bevy systems to handle
// those updates as needed.
// WHAT IS POSSIBLE HERE???
// fn tauri_listener(
//     commands: &mut Commands,
// ) {
//     todo!();
    
//     // Something like the below would be ideal.
//     // If could get the value from the invoke/Tauri command,
//     // could use in Bevy as a new component/entity to query for
//     // or perhaps with Bevy signals?

//     // let args = JsValue::NULL;  // not using args, but have to pass in something to invoke ?
//     // let latest_box_count = invoke("check_box_count", args).await;
//     // .....now use latest_box_count within Bevy.....
//     // commands.spawn(TauriBoxCount(latest_box_count));  <-- This may work with query in other systems?
// }

// #[derive(Component)]
// struct TauriBoxCount(u32);



#[derive(Component)]
struct BevyBox;


// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // Cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },
    BevyBox));
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform:  Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}


// Add additional cubes... ideally based on state contained in Tauri side
fn add_cube_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // query: Query<&Transform, With<BevyBox>>,
    // mut tauri_query: Query<(Entity, &mut TauriBoxCount)>,
) {
    // Spawn a second box after some random delay
    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() < 0.005 {
        commands.spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.8, 1.2, 1.0),
            ..default()
        },
        BevyBox));
    }


    // Placeholder to spawn more boxes...
    // TODO - WANT TO RUN THIS BASED ON TAURI COMMANDS/STATE
    if rng.gen::<f32>() < 0.002 {
        commands.spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
            material: materials.add(Color::rgb(0.8, 0.7, 1.0).into()),
            transform: Transform::from_xyz(rng.gen::<f32>() - 1.5, rng.gen::<f32>() + 0.9, rng.gen::<f32>() + 0.5),
            ..default()
            },
            BevyBox,
        ));
    }

    // let box_count: u32 = query.into_iter().map(|_| {1}).sum();
    // for (entity, new_box_count) in &mut tauri_query {
    //     if box_count < new_box_count.0 {
    //         commands.spawn((PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
    //             material: materials.add(Color::rgb(0.8, 0.7, 1.0).into()),
    //             transform: Transform::from_xyz(rng.gen::<f32>() - 1.5, rng.gen::<f32>() + 0.9, rng.gen::<f32>() + 0.5),
    //             ..default()
    //             },
    //             BevyBox,
    //         ));
    //     }
    //     commands.entity(entity).despawn();
    // }

}
