use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_openxr::add_xr_plugins;
use bevy_vr_controller::{
    animation::defaults::default_character_animations, player::PlayerSettings, VrControllerPlugin,
};
use bevy_xr_utils::{tracking_utils::TrackingUtilitiesPlugin, xr_utils_actions::XRUtilsActionsPlugin};

fn main() {
    App::new()
        .add_plugins((
            add_xr_plugins(DefaultPlugins),
            XRUtilsActionsPlugin,
            PhysicsDebugPlugin::default(),
            PhysicsPlugins::default(),
            VrControllerPlugin,
            TrackingUtilitiesPlugin,
        ))
        .add_systems(Startup, (setup_scene, setup_player))
        .run();
}

const GROUND_SIZE: f32 = 15.0;
const GROUND_THICKNESS: f32 = 0.2;

fn setup_scene(
    mut ambient: ResMut<AmbientLight>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    ambient.brightness = 100.0;
    ambient.color = Color::linear_rgb(0.95, 0.95, 1.0);

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(4.5, 10.0, -7.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(
                GROUND_SIZE,
                GROUND_THICKNESS,
                GROUND_SIZE,
            ))),
            material: materials.add(StandardMaterial::default()),
            transform: Transform::from_xyz(0.0, -GROUND_THICKNESS / 2.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(GROUND_SIZE, GROUND_THICKNESS, GROUND_SIZE),
    ));
}

fn setup_player(asset_server: Res<AssetServer>, mut commands: Commands) {
    PlayerSettings {
        animations: Some(default_character_animations(&asset_server)),
        vrm: Some(asset_server.load("alicia.vrm")),
        void_level: Some(-20.0),
        spawn: Vec3::new(0.0, 3.0, 0.0),
        ..default()
    }
    .spawn(&mut commands);
}
