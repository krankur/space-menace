use amethyst::{
    assets::{Handle, Prefab},
    core::{
        Float,
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    ecs::prelude::World,
    prelude::Builder,
    renderer::transparent::Transparent,
};

use specs_physics::{
    colliders::Shape,
    nphysics::{algebra::Velocity3, object::BodyStatus},
    PhysicsBody, PhysicsBodyBuilder, PhysicsColliderBuilder,
};

use crate::{
    components::{
        Animation, AnimationId, AnimationPrefabData, BoundingRect, Collidee, Collider, Direction,
        Directions, Marine, Motion, TwoDimObject,
    },
    resources::Context,
};

pub fn load_marine(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>, ctx: &Context) {
    let mut transform = Transform::default();
    let scale = ctx.scale;
    transform.set_scale(Vector3::new(scale, scale, scale));
    transform.set_translation_z(-10.);

    let mut two_dim_object = TwoDimObject::new(32. * scale, 36. * scale);
    two_dim_object.set_position(384., 176.);
    two_dim_object.update_transform_position(&mut transform);

    let shape = Shape::<Float>::Circle((16. * scale).into());

    world
        .create_entity()
        .with(Marine::new())
        .named("Marine")
        // .with(two_dim_object)
        // .with(Collider::new(
        //     Vector2::new(384., 176.),
        //     BoundingRect::new(ctx.x_correction, ctx.map_width, 352., 0.),
        // ))
        .with(PhysicsBodyBuilder::<Float>::from(BodyStatus::Dynamic)
            .gravity_enabled(true)
            .velocity(Velocity3::linear(0.0.into(), 0.0.into(), 0.0.into()))
            .build())
        .with(PhysicsColliderBuilder::<Float>::from(shape)
            .build())
        // .with(Collidee::default())
        .with(transform)
        .with(Motion::new())
        .with(Animation::new(
            AnimationId::Idle,
            vec![
                AnimationId::Die,
                AnimationId::Idle,
                AnimationId::Jump,
                AnimationId::Move,
                AnimationId::Shoot,
            ],
        ))
        .with(prefab)
        .with(Direction::new(
            Directions::Right,
            Directions::Neutral,
            Directions::Right,
            Directions::Neutral,
        ))
        // .with(Transparent) // Necessary for ordered layering
        .build();
}
