use std::{collections::VecDeque, f32::consts::PI};

use bevy::prelude::*;

const DIMENSION: usize = 5;

const CUBE_LEN: f32 = 0.9;

const CENTER: f32 = (DIMENSION - 1) as f32 / 2.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera_light)
        .add_systems(Startup, setup)
        // .add_systems(Update, (rotate_camera, ))
        // .add_systems(Update, scale_camera)
        .add_event::<RotateEvent>()
        .add_event::<RotateDoneEvent>()
        .add_systems(Update, input_handler)
        .add_systems(Update, rotate_cube)
        .add_systems(Update, update_cube_position)
        .run();
}

fn setup_camera_light(mut commands: Commands) {
    //camera
    commands.spawn(Camera3dBundle {
        camera: Camera::default(),
        transform: Transform::from_xyz(8., 8., 8.).looking_at(Vec3::splat(CENTER), Vec3::Y),
        ..default()
    });

    //light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_xyz(8., 8., 8.),
        ..default()
    });
}

#[derive(Component)]
struct CubePart(Vec3);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Hello, world!");

    // // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    //     material: materials.add(Color::rgb_u8(124, 144, 255)),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });

    assert!(DIMENSION > 1);

    let x = [0, DIMENSION - 1];
    let y = [0, DIMENSION - 1];
    let z = [0, DIMENSION - 1];
    //point*8
    for x in x {
        for y in y {
            for z in z {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(CUBE_LEN, CUBE_LEN, CUBE_LEN)),
                        material: materials.add(Color::rgb_u8(255, 140, 0)),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    },
                    CubePart(Vec3::new(x as f32, y as f32, z as f32)),
                ));
            }
        }
    }

    //edge*12
    for x in 1..DIMENSION - 1 {
        for y in y {
            for z in z {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(CUBE_LEN, CUBE_LEN, CUBE_LEN)),
                        material: materials.add(Color::rgb_u8(124, 144, 255)),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    },
                    CubePart(Vec3::new(x as f32, y as f32, z as f32)),
                ));
            }
        }
    }

    for y in 1..DIMENSION - 1 {
        for x in x {
            for z in z {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(CUBE_LEN, CUBE_LEN, CUBE_LEN)),
                        material: materials.add(Color::rgb_u8(124, 144, 255)),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    },
                    CubePart(Vec3::new(x as f32, y as f32, z as f32)),
                ));
            }
        }
    }

    for z in 1..DIMENSION - 1 {
        for x in x {
            for y in y {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(CUBE_LEN, CUBE_LEN, CUBE_LEN)),
                        material: materials.add(Color::rgb_u8(124, 144, 255)),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    },
                    CubePart(Vec3::new(x as f32, y as f32, z as f32)),
                ));
            }
        }
    }

    //face*6
    for x in x {
        for y in 1..DIMENSION - 1 {
            for z in 1..DIMENSION - 1 {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(CUBE_LEN, CUBE_LEN, CUBE_LEN)),
                        material: materials.add(Color::rgb_u8(154, 205, 50)),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    },
                    CubePart(Vec3::new(x as f32, y as f32, z as f32)),
                ));
            }
        }
    }

    for y in y {
        for x in 1..DIMENSION - 1 {
            for z in 1..DIMENSION - 1 {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(CUBE_LEN, CUBE_LEN, CUBE_LEN)),
                        material: materials.add(Color::rgb_u8(154, 205, 50)),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    },
                    CubePart(Vec3::new(x as f32, y as f32, z as f32)),
                ));
            }
        }
    }

    for z in z {
        for x in 1..DIMENSION - 1 {
            for y in 1..DIMENSION - 1 {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(CUBE_LEN, CUBE_LEN, CUBE_LEN)),
                        material: materials.add(Color::rgb_u8(154, 205, 50)),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    },
                    CubePart(Vec3::new(x as f32, y as f32, z as f32)),
                ));
            }
        }
    }
}

fn rotate_camera(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::splat(CENTER),
            Quat::from_axis_angle(Vec3::Y, time.delta_seconds() * PI / 5.0),
        );
    }
}

fn scale_camera(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in &mut query {
        // transform.translate_around(point, rotation)
        transform.translation += Vec3::splat(time.delta_seconds());
        transform.look_at(Vec3::splat(CENTER), Vec3::Y);
    }
}

#[derive(Event)]
struct RotateEvent(RotateMsg);

#[derive(Debug, Clone, Copy)]
struct RotateMsg(Vec3, usize, bool);

fn input_handler(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mesh_query: Query<&Handle<Mesh>, With<CubePart>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Transform, With<CubePart>>,
    mut rotate_ev_writer: EventWriter<RotateEvent>,
    mut keys: Local<(Option<KeyCode>, Option<KeyCode>)>
) {
    if keyboard_input.just_pressed(KeyCode::KeyX) {
        keys.0 = Some(KeyCode::KeyX);
        // rotate_ev_writer.send(RotateEvent(RotateMsg(Vec3::X, 0, true)));
    }
    if keyboard_input.just_pressed(KeyCode::KeyY) {
        keys.0 = Some(KeyCode::KeyY);
        // rotate_ev_writer.send(RotateEvent(RotateMsg(Vec3::Y, 0, true)));
    }
    if keyboard_input.just_pressed(KeyCode::KeyZ) {
        keys.0 = Some(KeyCode::KeyZ);
        // rotate_ev_writer.send(RotateEvent(RotateMsg(Vec3::Z, 0, true)));
    }

    if keyboard_input.just_pressed(KeyCode::Digit0) {
        keys.1 = Some(KeyCode::Digit0);
    }

    if keyboard_input.just_pressed(KeyCode::Digit1) {
        keys.1 = Some(KeyCode::Digit1);
    }

    if keyboard_input.just_pressed(KeyCode::Digit2) {
        keys.1 = Some(KeyCode::Digit2);
    }

    if keyboard_input.just_pressed(KeyCode::Digit3) {
        keys.1 = Some(KeyCode::Digit3);
    }

    if keyboard_input.just_pressed(KeyCode::Digit4) {
        keys.1 = Some(KeyCode::Digit4);
    }

    if keys.0.is_none() && keys.1.is_some() {
        keys.1 = None;
    }

    if keys.0.is_some() && keys.1.is_some() {
        let v3 = match keys.0.unwrap() {
            KeyCode::KeyX => Vec3::X,
            KeyCode::KeyY => Vec3::Y,
            KeyCode::KeyZ => Vec3::Z,
            _ => Vec3::X,
        };

        let idx = match keys.1.unwrap() {
            KeyCode::Digit0 => 0,
            KeyCode::Digit1 => 1,
            KeyCode::Digit2 => 2,
            KeyCode::Digit3 => 3,
            KeyCode::Digit4 => 4,
            _ => 0,
        };
        rotate_ev_writer.send(RotateEvent(RotateMsg(v3, idx, true)));
        keys.0 = None;
        keys.1 = None;
    }
    // if keyboard_input.pressed(KeyCode::KeyR) {
    //     for mut transform in &mut query {
    //         transform.look_to(Vec3::NEG_Z, Vec3::Y);
    //     }
    // }
}

#[derive(Event)]
struct RotateDoneEvent(RotateMsg);

#[derive(Resource, Default)]
struct Ra(Option<(f32, f32, RotateMsg)>);

fn rotate_cube(
    time: Res<Time>,
    mut local: Local<VecDeque<RotateMsg>>,
    mut current: Local<Ra>,
    mut rotate_ev: EventReader<RotateEvent>,
    mut rotate_done_ev: EventWriter<RotateDoneEvent>,
    mut query: Query<(&mut Transform, &CubePart), With<CubePart>>,
) {
    for ev in rotate_ev.read() {
        if local.len() < 1 {
            // only 1 steps is cached
            println!("rotate: {:?}", ev.0);
            local.push_back(ev.0.clone());
        }
    }

    if let Some((done, target, msg)) = current.0 {
        let mut angle = time.delta_seconds() * PI / 2.;
        // if done > target {
        //     angle = -angle;
        // }
        if done + angle < target {
            current.0 = Some((done + angle, target, msg));
        } else {
            angle = target - done;
            current.0 = None;

            rotate_done_ev.send(RotateDoneEvent(msg));
        }

        for (mut transform, position) in &mut query {
            let mut result = false;
            if msg.0 == Vec3::X && msg.1 == position.0.x as usize {
                //weird
                result = true;
            } else if msg.0 == Vec3::Y && msg.1 == position.0.y as usize {
                result = true;
            } else if msg.0 == Vec3::Z && msg.1 == position.0.z as usize {
                result = true;
            }
            if result {
                transform.rotate_around(Vec3::splat(CENTER), Quat::from_axis_angle(msg.0, angle));
            }
        }
    }

    if current.0.is_none() {
        if let Some(msg) = local.pop_front() {
            let angle = if msg.2 { PI / 2. } else { -PI / 2. };
            current.0 = Some((0., angle, msg));
        }
    }
}

fn update_cube_position(
    mut rotate_done_ev: EventReader<RotateDoneEvent>,
    mut query: Query<&mut CubePart, With<CubePart>>,
) {
    for ev in rotate_done_ev.read() {
        let msg = ev.0;
        let axis = msg.0;
        let i = msg.1;

        for mut cubepart in &mut query {
            if msg.0 == Vec3::X && msg.1 == cubepart.0.x as usize {
                //weird
                let (y, z) = clock(
                    cubepart.0.y as isize - CENTER as isize,
                    cubepart.0.z as isize - CENTER as isize,
                );
                cubepart.0 = Vec3::new(i as f32, y as f32 + CENTER, z as f32 + CENTER);
            } else if msg.0 == Vec3::Y && msg.1 == cubepart.0.y as usize {
                let (x, z) = anti_clock(
                    cubepart.0.x as isize - CENTER as isize,
                    cubepart.0.z as isize - CENTER as isize,
                );
                cubepart.0 = Vec3::new(x as f32 + CENTER, i as f32, z as f32 + CENTER);
            } else if msg.0 == Vec3::Z && msg.1 == cubepart.0.z as usize {
                let (x, y) = clock(
                    cubepart.0.x as isize - CENTER as isize,
                    cubepart.0.y as isize - CENTER as isize,
                );
                cubepart.0 = Vec3::new( x as f32 + CENTER, y as f32 + CENTER, i as f32);
            }
        }
    }
}

fn new_position(old: &CubePart, rotate: &RotateMsg) -> CubePart {
    todo!()
}

fn clock(a: isize, b: isize) -> (isize, isize) {
    (-b, a)
}

fn anti_clock(a: isize, b: isize) -> (isize, isize) {
    (b, -a)
}
