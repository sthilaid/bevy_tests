use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use rand::{Rng, SeedableRng};
use rand_distr::Normal;
use rand_pcg::Pcg32;
use std::f32::consts::PI;

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
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: &Vec3,
    radius: f32,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: radius,
                subdivisions: 2,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
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
    branchCount: u32,
    elemCount: u32,
    initRadius: f32,
    expansionRate: f32,
    revCount: f32,
    depthStdDev: f32,
    latOffsetStdDev: f32,
) {
    let systemsPerBranch = elemCount / branchCount;
    let systemsPerRevolution = systemsPerBranch / (revCount as u32);
    let branchStartAngleDelta = 2.0 * PI / (branchCount as f32);

    let mut rng = Pcg32::seed_from_u64(seed);
    let depth_distribution = Normal::new(0.0, depthStdDev).unwrap();
    let lateral_distribution = Normal::new(0.0, latOffsetStdDev).unwrap();

    for branchIdx in 0..branchCount {
        let initAngle = (branchIdx as f32) * branchStartAngleDelta;
        for starIdx in 0..systemsPerBranch {
            let revolutionRatio = (starIdx as f32) / (systemsPerRevolution as f32);
            let branchRevAngle =
                initAngle + (revolutionRatio - f32::floor(revolutionRatio)) * 2.0 * PI;
            let inflation = initAngle + expansionRate * revolutionRatio;
            let centerDistance = initRadius + inflation;
            let perfectPos =
                Vec3::new(f32::cos(branchRevAngle), f32::sin(branchRevAngle), 0.0) * centerDistance;
            let depth = rng.sample(depth_distribution);
            let lateralDist = rng.sample(lateral_distribution);
            let offset = perfectPos.normalize() * lateralDist + Vec3::new(0.0, 0.0, depth);
            let final_pos = perfectPos + offset;
            spawn_star(&mut commands, &mut meshes, &mut materials, &final_pos, 0.01);
        }
    }
}

// def generateSpiralGalaxy(seed, branchCount, elemCount, initRadius, expansionRate, revCount, depthStdDev, latOffsetStdDev):
//     rng = np.random.RandomState(seed)
//     systemsPerRevolution    = int(elemCount / revCount)
//     branchStartAngleDelta   = 2 * np.pi / branchCount
//     branches                = []
//     for branchIdx in range(0, branchCount):
//         initAngle           = branchIdx * branchStartAngleDelta
//         branch              = []
//         for star in range(0, elemCount):
//             revolutionRatio = star / systemsPerRevolution
//             branchRevAngle  = initAngle + (revolutionRatio - np.floor(revolutionRatio)) * 2.0 * np.pi
//             inflation       = initRadius + expansionRate * revolutionRatio
//             centerDistance  = initRadius + inflation
//             perfectPos      = np.multiply([np.cos(branchRevAngle), np.sin(branchRevAngle), 0.0], centerDistance)
//             depth           = rng.normal(0.0, depthStdDev)
//             lateralDist     = rng.normal(0.0, latOffsetStdDev)
//             offset          = np.multiply(normalize(perfectPos), lateralDist) + [0,0,depth]
//             starSys         = StarSystem(np.add(perfectPos, offset), 10, StarSystemTypeDB.random(rng))
//             branch          = branch + [starSys]
//         branches = branches + branch
//     return branches

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
        3,        // branchCount: u32,
        30,       // elemCount: u32,
        1.0,      // initRadius: f32,
        0.5,      // expansionRate: f32,
        1.5,      // revCount: f32,
        0.3,      // depthStdDev: f32,
        0.05,     // latOffsetStdDev: f32,
    );
    // spawn_star(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     &Vec3::new(0.2, 0.5, -1.0),
    // );

    // light
    // commands.spawn_bundle(LightBundle {
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..Default::default()
    // });

    commands.spawn_bundle(OrthographicCameraBundle::new_3d());
}

fn update_scene(time: Res<Time>, mut query: Query<(&Star, &mut Transform)>) {
    // for (_, mut pbr) in query.iter_mut() {
    //     let delta_y = (f32::cos(2.0 * time.seconds_since_startup() as f32) + 1.0) * 0.5;
    //     pbr.translation.y = 0.5 + delta_y;
    // }
}

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_scene.system())
            .add_system(update_scene.system());
    }
}

fn input_system(mut exit: EventWriter<AppExit>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(input_system.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(GamePlugin)
        .run();
}
