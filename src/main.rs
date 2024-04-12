use bevy::prelude::*;
use iyes_perf_ui::{PerfUiCompleteBundle, PerfUiPlugin};
use regex::Regex;
use std::{collections::VecDeque, f32::consts::PI};

const DIMENSION: usize = 5;
const CUBE_LEN: f32 = 0.9;
const CENTER: f32 = (DIMENSION - 1) as f32 / 2.;
const ROTATE_SPEED: f32 = PI / 2.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MyPerfUIPlugin)
        .init_resource::<History>()
        .add_systems(Startup, setup_camera_light)
        .add_systems(Startup, setup_cubes)
        // .add_systems(Update, (rotate_camera,))
        // .add_systems(Update, scale_camera)
        .add_event::<InputEvent>()
        .add_event::<RotateDoneEvent>()
        .add_event::<RotateEvent>()
        .add_systems(Update, rotate_cmd_handler)
        .add_systems(Update, (rotate_calculater, rotate_animate).chain())
        .add_systems(Update, text_input)
        .run();
}


struct MyPerfUIPlugin;

impl Plugin for MyPerfUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup_perf_ui);
    }
}

fn setup_perf_ui(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}

fn setup_camera_light(mut commands: Commands) {
    //camera
    commands.spawn(Camera3dBundle {
        camera: Camera::default(),
        transform: Transform::from_translation(Vec3::splat(DIMENSION as f32))
            .looking_at(Vec3::splat(CENTER), Vec3::Y),
        ..default()
    });

    //light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::splat(DIMENSION as f32)),
        ..default()
    });
}

#[derive(Component, Debug)]
struct CubeIndex(usize, usize, usize);

fn idx2axis(a: usize) -> f32 {
    a as f32 - CENTER
}

fn setup_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Hello, world!");

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
                        mesh: meshes.add(Cuboid::from_size(Vec3::splat(CUBE_LEN))),
                        material: materials.add(Color::rgb_u8(255, 140, 0)),
                        transform: Transform::from_xyz(idx2axis(x), idx2axis(y), idx2axis(z)),
                        ..default()
                    },
                    CubeIndex(x, y, z),
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
                        mesh: meshes.add(Cuboid::from_size(Vec3::splat(CUBE_LEN))),
                        material: materials.add(Color::rgb_u8(124, 144, 255)),
                        transform: Transform::from_xyz(idx2axis(x), idx2axis(y), idx2axis(z)),
                        ..default()
                    },
                    CubeIndex(x, y, z),
                ));
            }
        }
    }

    for y in 1..DIMENSION - 1 {
        for x in x {
            for z in z {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::from_size(Vec3::splat(CUBE_LEN))),
                        material: materials.add(Color::rgb_u8(124, 144, 255)),
                        transform: Transform::from_xyz(idx2axis(x), idx2axis(y), idx2axis(z)),
                        ..default()
                    },
                    CubeIndex(x, y, z),
                ));
            }
        }
    }

    for z in 1..DIMENSION - 1 {
        for x in x {
            for y in y {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::from_size(Vec3::splat(CUBE_LEN))),
                        material: materials.add(Color::rgb_u8(124, 144, 255)),
                        transform: Transform::from_xyz(idx2axis(x), idx2axis(y), idx2axis(z)),
                        ..default()
                    },
                    CubeIndex(x, y, z),
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
                        mesh: meshes.add(Cuboid::from_size(Vec3::splat(CUBE_LEN))),
                        material: materials.add(Color::rgb_u8(154, 205, 50)),
                        transform: Transform::from_xyz(idx2axis(x), idx2axis(y), idx2axis(z)),
                        ..default()
                    },
                    CubeIndex(x, y, z),
                ));
            }
        }
    }

    for y in y {
        for x in 1..DIMENSION - 1 {
            for z in 1..DIMENSION - 1 {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::from_size(Vec3::splat(CUBE_LEN))),
                        material: materials.add(Color::rgb_u8(154, 205, 50)),
                        transform: Transform::from_xyz(idx2axis(x), idx2axis(y), idx2axis(z)),
                        ..default()
                    },
                    CubeIndex(x, y, z),
                ));
            }
        }
    }

    for z in z {
        for x in 1..DIMENSION - 1 {
            for y in 1..DIMENSION - 1 {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::from_size(Vec3::splat(CUBE_LEN))),
                        material: materials.add(Color::rgb_u8(154, 205, 50)),
                        transform: Transform::from_xyz(idx2axis(x), idx2axis(y), idx2axis(z)),
                        ..default()
                    },
                    CubeIndex(x, y, z),
                ));
            }
        }
    }
}

fn rotate_camera(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
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

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Axis {
    #[default]
    X,
    Y,
    Z,
}

impl From<&str> for Axis {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();
        if value == "x" {
            Axis::X
        } else if value == "y" {
            Axis::Y
        } else if value == "z" {
            Axis::Z
        } else {
            Axis::X
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
struct RotateCmd {
    axis: Axis,
    index: usize,
    clockwise: bool,
}

#[derive(Event, Debug)]
struct InputEvent(Cmd);

#[derive(Event, Debug)]
struct RotateEvent(RotateCmd);

#[derive(Event)]
struct RotateDoneEvent;

#[derive(Resource)]
struct History {
    stack: VecDeque<RotateCmd>,
    cached: VecDeque<RotateCmd>,
}

impl Default for History {
    fn default() -> Self {
        Self {
            stack: VecDeque::with_capacity(64),
            cached: VecDeque::with_capacity(64),
        }
    }
}

impl History {
    fn push(&mut self, value: RotateCmd) {
        self.stack.push_back(value);
        self.cached.clear();
    }

    fn undo(&mut self) -> Option<RotateCmd> {
        self.stack.pop_back().and_then(|mut cmd| {
            self.cached.push_back(cmd);
            cmd.clockwise = !cmd.clockwise;
            Some(cmd)
        })
    }

    fn redo(&mut self) -> Option<RotateCmd> {
        self.cached.pop_back().and_then(|cmd| {
            self.stack.push_back(cmd);
            Some(cmd)
        })
    }
}

#[derive(Debug, Resource)]
struct CmdBuffer(VecDeque<RotateCmd>);

impl Default for CmdBuffer {
    fn default() -> Self {
        Self(VecDeque::with_capacity(1))
    }
}

fn rotate_cmd_handler(
    mut cmd_in_progress: Local<bool>,
    mut history: ResMut<History>,
    mut input_ev: EventReader<InputEvent>,
    mut rotate_done_ev: EventReader<RotateDoneEvent>,
    mut rotate_cal_ev: EventWriter<RotateEvent>,
) {
    for _ in rotate_done_ev.read() {
        *cmd_in_progress = false;
    }

    for ev in input_ev.read() {
        match ev.0 {
            Cmd::UnDo => {
                if let Some(cmd) = history.undo() {
                    rotate_cal_ev.send(RotateEvent(cmd));
                }
            }
            Cmd::ReDo => {
                if let Some(cmd) = history.redo() {
                    rotate_cal_ev.send(RotateEvent(cmd));
                }
            }
            Cmd::Do(cmd) => {
                if !*cmd_in_progress {
                    *cmd_in_progress = true;
                    history.push(cmd);
                    rotate_cal_ev.send(RotateEvent(cmd));
                }
            }
        }
    }
}

fn rotate_calculater(
    mut query: Query<&mut CubeIndex, With<CubeIndex>>,
    mut ev: EventReader<RotateEvent>,
) {
    for e in ev.read() {
        let axis = e.0.axis;
        let index = e.0.index;
        let is_clockwise = e.0.clockwise;
        for mut cube in &mut query {
            // print!("{:?}", cube);
            match axis {
                Axis::X => {
                    if index == cube.0 {
                        let (a, b) = if is_clockwise {
                            clockwise(cube.1, cube.2, DIMENSION)
                        } else {
                            anti_clockwise(cube.1, cube.2, DIMENSION)
                        };
                        *cube = CubeIndex(index, a, b);
                    }
                }
                Axis::Y => {
                    if index == cube.1 {
                        let (b, a) = if is_clockwise {
                            clockwise(cube.2, cube.0, DIMENSION)
                        } else {
                            anti_clockwise(cube.2, cube.0, DIMENSION)
                        };
                        *cube = CubeIndex(a, index, b);
                    }
                }
                Axis::Z => {
                    if index == cube.2 {
                        let (a, b) = if is_clockwise {
                            clockwise(cube.0, cube.1, DIMENSION)
                        } else {
                            anti_clockwise(cube.0, cube.1, DIMENSION)
                        };
                        *cube = CubeIndex(a, b, index);
                    }
                }
            };
            // println!(" -> {:?}", cube);
        }
    }
}

#[derive(Debug)]
enum Cmd {
    UnDo,
    ReDo,
    Do(RotateCmd),
}

fn text_input(
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<ButtonInput<KeyCode>>,
    mut buf: Local<String>,
    mut last_buf: Local<String>,
    mut rotate_ev: EventWriter<InputEvent>,
) {
    if kbd.just_pressed(KeyCode::Enter) {
        if buf.is_empty() && !last_buf.is_empty() {
            *buf = last_buf.clone();
        }
        println!("Text input: {:?}", buf);
        if *buf == String::from("undo") {
            rotate_ev.send(InputEvent(Cmd::UnDo));
        }
        if *buf == String::from("redo") {
            rotate_ev.send(InputEvent(Cmd::ReDo));
        }

        let re = Regex::new(r"^([x-z])([1-9][0-9]*?)('?)$").unwrap();

        for (_, [axis, index, clockwise]) in re.captures_iter(&buf).map(|c| c.extract()) {
            rotate_ev.send(InputEvent(Cmd::Do(RotateCmd {
                axis: axis.into(),
                index: index.parse::<usize>().unwrap_or(0).saturating_sub(1),
                clockwise: !clockwise.eq("'"),
            })));
        }
        *last_buf = buf.clone();
        buf.clear();
    }
    if kbd.just_pressed(KeyCode::Backspace) {
        buf.pop();
        println!("buf: {:?}", buf)
    }
    for ev in evr_char.read() {
        // ignore control (special) characters
        if !ev.char.chars().last().unwrap().is_control() {
            buf.push(ev.char.parse().unwrap());
            println!("buf: {:?}", buf)
        }
    }
}

fn rotate_animate(
    timer: Res<Time>,
    mut local: Local<Option<(Axis, usize, f32)>>,
    mut ev: EventReader<RotateEvent>,
    mut rotate_done_ev: EventWriter<RotateDoneEvent>,
    mut query: Query<(&mut Transform, &CubeIndex), With<CubeIndex>>,
) {
    let mut done = false;
    for event in ev.read() {
        let cmd = event.0;
        match local.as_mut() {
            Some((axis, index, degrees)) => {
                if *axis == cmd.axis && *index == cmd.index {
                    *degrees += if cmd.clockwise { PI / 2. } else { -PI / 2. };
                }
            }
            None => {
                let degrees = if cmd.clockwise { PI / 2. } else { -PI / 2. };
                *local = Some((cmd.axis, cmd.index, degrees));
            }
        }
    }

    let mut delta = timer.delta_seconds() * ROTATE_SPEED;
    if let Some((axis, index, degrees)) = *local {
        let abs_degrees = degrees.abs();
        if abs_degrees < delta {
            delta = abs_degrees;
            done = true;
        }

        if degrees > 0. {
            delta = -delta;
        }

        for (mut transform, idx) in &mut query {
            let (cmp_idx, axis_vec3) = match axis {
                Axis::X => (idx.0, Vec3::X),
                Axis::Y => (idx.1, Vec3::Y),
                Axis::Z => (idx.2, Vec3::Z),
            };
            if index == cmp_idx {
                transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(axis_vec3, delta));
            }
        }
    }

    match local.as_mut() {
        Some((_, _, v)) => *v += delta,
        None => {}
    }

    if done {
        *local = None;
        rotate_done_ev.send(RotateDoneEvent);
    }
}

fn clockwise(a: usize, b: usize, dim: usize) -> (usize, usize) {
    (b, dim - 1 - a)
}

fn anti_clockwise(a: usize, b: usize, dim: usize) -> (usize, usize) {
    (dim - 1 - b, a)
}
