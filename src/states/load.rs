use amethyst::{
    assets::{AssetStorage, Handle, JsonFormat, Loader, ProgressCounter},
    core::{
        ArcThreadPool,
        Float,
        math::Vector3,
        Transform
    },
    ecs::{Dispatcher, DispatcherBuilder},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans},
};

use specs_physics::{
        parameters::Gravity,
    register_physics_systems
};

use crate::{
    entities::{load_camera, load_camera_subject, load_marine, load_pincer},
    resources::{load_assets, AssetType, Context, Map, PrefabList},
    systems::*,
};

#[derive(Default)]
pub struct LoadState<'a, 'b> {
    progress_counter: Option<ProgressCounter>,
    map_handle: Option<Handle<Map>>,
    fixed_dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for LoadState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let thread_pool = world.res.fetch::<ArcThreadPool>().clone();

        let mut builder = DispatcherBuilder::new()
            .with_pool(thread_pool)
            .with(MarineAccelerationSystem, "marine_acceleration_system", &[])
            .with(
                AttackSystem,
                "attack_system",
                &["marine_acceleration_system"],
            )
            .with(
                CollisionSystem,
                "collision_system",
                &["marine_acceleration_system"],
            )
            .with(
                BulletCollisionSystem,
                "bullet_collision_system",
                &["collision_system"],
            )
            .with(
                BulletImpactAnimationSystem,
                "bullet_impact_animation_system",
                &["bullet_collision_system"],
            )
            .with(
                PincerCollisionSystem,
                "pincer_collision_system",
                &["collision_system"],
            )
            .with(
                PincerAnimationSystem,
                "pincer_animation_system",
                &["pincer_collision_system"],
            )
            .with(ExplosionAnimationSystem, "explosion_animation_system", &[])
            .with(ParallaxSystem, "parallax_system", &[])
            .with(
                MotionSystem,
                "motion_system",
                &["collision_system", "parallax_system"],
            )
            .with(
                MarineAnimationSystem,
                "marine_animation_system",
                &["collision_system"],
            )
            .with(AnimationControlSystem, "animation_control_system", &[])
            .with(DirectionSystem, "direction_system", &[])
            .with(MarineMotionSystem, "marine_motion_system", &[])
            .with(
                CameraMotionSystem,
                "camera_motion_system",
                &["collision_system"],
            );

        register_physics_systems::<Float, Transform>(&mut builder);
        let mut dispatcher = builder.build();
        dispatcher.setup(&mut world.res);
        self.fixed_dispatcher = Some(dispatcher);

        world
            .add_resource(Gravity::<Float>(Vector3::<Float>::new(
                0.0.into(),
                Float::from_f32(-100.),
                0.0.into(),
            )));

        world.add_resource(Context::new());

        self.progress_counter = Some(load_assets(
            world,
            vec![
                AssetType::Background,
                AssetType::Bullet,
                AssetType::BulletImpact,
                AssetType::Marine,
                AssetType::Pincer,
                AssetType::Platform,
                AssetType::SmallExplosion,
                AssetType::Truss,
            ],
        ));
        self.map_handle = {
            let loader = world.read_resource::<Loader>();
            Some(loader.load(
                "tilemaps/map.json",
                JsonFormat,
                self.progress_counter.as_mut().expect("map"),
                &world.read_resource::<AssetStorage<Map>>(),
            ))
        };

        let camera_subject = load_camera_subject(world);
        load_camera(world, camera_subject);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            // Check if all data has been loaded
            if progress_counter.is_complete() {
                // Get the map, which is loaded in the on_start function of load state.
                let map = {
                    let map_storage = &data.world.read_resource::<AssetStorage<Map>>();
                    let map_handle = &self.map_handle.take().unwrap();
                    map_storage.get(map_handle).unwrap().clone()
                };
                let ctx = data.world.read_resource::<Context>().clone();

                map.load_layers(data.world, &ctx);

                let marine_prefab_handle = {
                    let prefab_list = data.world.read_resource::<PrefabList>();
                    prefab_list.get(AssetType::Marine).unwrap().clone()
                };
                load_marine(data.world, marine_prefab_handle, &ctx);

                let pincer_prefab_handle = {
                    let prefab_list = data.world.read_resource::<PrefabList>();
                    prefab_list.get(AssetType::Pincer).unwrap().clone()
                };
                load_pincer(data.world, pincer_prefab_handle, &ctx);
                self.progress_counter = None;
            }
        }
        Trans::None
    }

    fn fixed_update(&mut self, data: StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = &mut self.fixed_dispatcher {
            dispatcher.dispatch(&data.world.res);
        }
        Trans::None
    }
}
