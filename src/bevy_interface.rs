use rand::Rng;
// use futures_lite::future;
use bevy::prelude::*;
// use bevy::tasks::{AsyncComputeTaskPool, TaskPool, Task};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::{invoke_no_args, invoke_with_args};




pub struct BevyInterfacePlugin;
impl Plugin for BevyInterfacePlugin {
    fn build(&self, app: &mut App) {
            app.add_plugins(PanOrbitCameraPlugin)

            // .insert_resource(AsyncTaskPool(*AsyncComputeTaskPool::init(TaskPool::new)))

            .add_systems(Startup, setup)
            .add_systems(Update, add_cube_system);

            // .add_systems(PostUpdate, tauri_listener);
    }
}


// #[derive(Resource)]
// struct AsyncTaskPool(AsyncComputeTaskPool);


// #[derive(Component)]
// struct BoxCount(Task<u32>);


// fn tauri_listener(
//     mut commands: Commands,
//     mut task_pool: Res<AsyncTaskPool>,
// ) {
//     let thread_pool = AsyncComputeTaskPool::get();

//     let count = thread_pool.spawn_local(async {
//         let box_count = invoke_no_args("check_box_count").await.as_f64().unwrap() as u32;
//         box_count
//     });

//     commands.spawn(BoxCount(count));
// }



#[derive(Component)]
struct BevyBox;


// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },
    BevyBox));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            transform:  Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}



fn add_cube_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // query: Query<&Transform, With<BevyBox>>,
    // target_box_count_query: Query<&BoxCount>,
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


    // // HOW IN THE WORLD CAN WE COMMUNICATE WITH TAURI?
    // // Figure out if need to add new boxes and add them if needed
    // let target_box_count_task = &target_box_count_query.iter().next().unwrap().0;

    // if target_box_count_task.is_finished() {
    //     let target_box_count: u32 = future::block_on(future::poll_once(*target_box_count_task)).unwrap();
    
    //     if target_box_count > existing_box_count {
    //         commands.spawn((PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
    //             material: materials.add(Color::rgb(0.8, 0.7, 1.0).into()),
    //             transform: Transform::from_xyz(rng.gen::<f32>() - 1.5, rng.gen::<f32>() + 0.9, rng.gen::<f32>() + 0.5),
    //             ..default()
    //             },
    //             BevyBox,
    //         ));
    //     }
    // }
}
