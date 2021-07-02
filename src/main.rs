use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use rand::{Rng, SeedableRng};
use rand_distr::Normal;
use rand_pcg::Pcg32;
use std::f32::consts::{FRAC_PI_2, PI};

pub struct HelloPlugin;

struct Person;
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

struct Star;

fn spawn_star(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: &Vec3,
    radius: f32,
    color: &Color,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: radius,
                subdivisions: 2,
            })),
            //material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            material: materials.add(StandardMaterial {
                base_color: *color,
                unlit: true,
                ..Default::default()
            }),
            transform: Transform::from_translation(*pos),
            ..Default::default()
        })
        .insert(Star);
}

fn spawn_galaxy(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    seed: u64,
    branch_count: u32,
    elem_count: u32,
    init_radius: f32,
    expansion_rate: f32,
    revolution_count: f32,
    depth_std_dev: f32,
    lat_offset_std_dev: f32,
) {
    const TWO_PI: f32 = 2.0 * PI;
    let systems_per_branch = elem_count / branch_count;
    let systems_per_revolution = systems_per_branch / (revolution_count as u32);
    let branch_start_angle_delta = TWO_PI / (branch_count as f32);

    let mut rng = Pcg32::seed_from_u64(seed);
    let depth_distribution = Normal::new(0.0, depth_std_dev).unwrap();
    let lateral_distribution = Normal::new(0.0, lat_offset_std_dev).unwrap();
    let colors = [
        Color::rgb(1.0, 0.0, 0.0),
        Color::rgb(0.0, 1.0, 0.0),
        Color::rgb(0.0, 0.0, 1.0),
    ];

    for branch_idx in 0..branch_count {
        let init_angle = (branch_idx as f32) * branch_start_angle_delta;
        let branch_color = &colors[(branch_idx % (colors.len() as u32)) as usize];
        for star_idx in 0..systems_per_branch {
            let revolution_ratio = (star_idx as f32) / (systems_per_revolution as f32);
            let branch_rev_angle =
                init_angle + (revolution_ratio - f32::floor(revolution_ratio)) * TWO_PI;
            let inflation = expansion_rate * revolution_ratio;
            let center_dist = init_radius + inflation;
            let perfect_pos =
                Vec3::new(f32::cos(branch_rev_angle), f32::sin(branch_rev_angle), 0.0)
                    * center_dist;
            let depth = rng.sample(depth_distribution);
            let lateral_dist = rng.sample(lateral_distribution);
            let offset = perfect_pos.normalize() * lateral_dist + Vec3::new(0.0, 0.0, depth);
            let final_pos = perfect_pos + offset;
            spawn_star(
                &mut commands,
                &mut meshes,
                &mut materials,
                &final_pos,
                0.01,
                branch_color,
            );
        }
    }
}

struct GameCameraData {
    angle: f32,
}

impl GameCameraData {
    fn new() -> GameCameraData {
        GameCameraData { angle: FRAC_PI_2 }
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_galaxy(
        &mut commands,
        &mut meshes,
        &mut materials,
        12345678, // seed: u64,
        3,        // branch_count: u32,
        200,      // elem_count: u32,
        0.1,      // init_radius: f32,
        0.5,      // expansion_rate: f32,
        3.5,      // revolution_count: f32,
        0.05,     // depth_std_dev: f32,
        0.03,     // lat_offset_std_dev: f32,
    );

    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 3.0;
    commands.spawn_bundle(camera).insert(GameCameraData::new());
}

fn update_scene(
    time: Res<Time>,
    input_data: Res<InputData>,
    mut query: Query<(&mut GameCameraData, &mut Transform)>,
) {
    // for (_, mut pbr) in query.iter_mut() {
    //     let delta_y = (f32::cos(2.0 * time.seconds_since_startup() as f32) + 1.0) * 0.5;
    //     pbr.translation.y = 0.5 + delta_y;
    // }
    for (mut cam_data, mut cam_transform) in query.iter_mut() {
        const AMPLITUDE: f32 = 5.0;
        const FREQ: f32 = 0.5;
        // let x = f32::cos(FREQ * time.seconds_since_startup() as f32 + PI * 0.5) * AMPLITUDE;
        // let z = f32::sin(FREQ * time.seconds_since_startup() as f32 + PI * 0.5) * AMPLITUDE;
        let mut delta_angle = 0.0;
        if input_data.left {
            delta_angle = -FREQ * time.delta().as_secs_f32();
        } else if input_data.right {
            delta_angle = FREQ * time.delta().as_secs_f32();
        }
        cam_data.angle += delta_angle;
        let x = f32::cos(cam_data.angle) * AMPLITUDE;
        let z = f32::sin(cam_data.angle) * AMPLITUDE;

        *cam_transform = Transform::from_xyz(x, 0.0, z).looking_at(Vec3::ZERO, Vec3::Y);
    }
}

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_scene.system())
            .add_system(update_scene.system());
    }
}

struct InputData {
    left: bool,
    right: bool,
}

impl InputData {
    fn new() -> InputData {
        InputData {
            left: false,
            right: false,
        }
    }
    fn reset(self: &mut InputData) {
        *self = InputData::new();
    }
}

fn input_system(
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
    mut input_data: ResMut<InputData>,
) {
    if keyboard_input.pressed(KeyCode::Q) {
        exit.send(AppExit);
    }

    input_data.reset();
    if keyboard_input.pressed(KeyCode::Left) {
        input_data.left = true;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        input_data.right = true;
    }
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(InputData::new());
        app.add_system(input_system.system());
    }
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(GamePlugin)
        .run();
}
