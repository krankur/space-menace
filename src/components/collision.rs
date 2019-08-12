use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage, NullStorage},
};

use crate::components::BoundingBox;

#[derive(Copy, Clone, Default)]
pub struct Boundary {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Boundary {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

pub struct CollideeDetails {
    pub name: String,
    pub velocity: Vector2<f32>,
    pub bounding_box: BoundingBox,
    pub correction: f32,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct CollideeNew {
    pub horizontal: Option<CollideeDetails>,
    pub vertical: Option<CollideeDetails>,
}

impl Default for CollideeNew {
    fn default() -> Self {
        Self {
            horizontal: None,
            vertical: None,
        }
    }
}

impl CollideeNew {
    pub fn set_collidee_details(
        &mut self,
        name: String,
        bb_a: &BoundingBox,
        bb_b: &BoundingBox,
        velocity_a: Vector2<f32>,
        velocity_b: Vector2<f32>,
    ) {
        let correction;
        let speed_sum = Vector2::new((velocity_a.x - velocity_b.x).abs(), (velocity_a.y - velocity_b.y).abs());
        let speed_ratio = Vector2::new(velocity_a.x / speed_sum.x, velocity_a.y / speed_sum.y);
        let min_safe_distance = Vector2::new(bb_a.half_size.x + bb_b.half_size.x, bb_a.half_size.y + bb_b.half_size.y);
        let overlap = Vector2::new(min_safe_distance.x - (bb_a.position.x - bb_b.position.x).abs(), min_safe_distance.y - (bb_a.position.y - bb_b.position.y).abs());

        println!("bb_a.old_position.y = {}", bb_a.old_position.y);
        println!("bb_b.old_position.y = {}", bb_b.old_position.y);
        // TODO: Reuse is_overlapping_with logic?
        let x_overlapped = (bb_a.old_position.x - bb_b.old_position.x).abs() < bb_a.half_size.x + bb_b.half_size.x;
        let y_overlapped = (bb_a.old_position.y - bb_b.old_position.y).abs() < bb_a.half_size.y + bb_b.half_size.y;

        println!("x_overlapped = {}", x_overlapped);
        println!("y_overlapped = {}", y_overlapped);
        if !x_overlapped && y_overlapped || (!x_overlapped && !y_overlapped && overlap.x.abs() <= overlap.y.abs()) {
            println!("overlap.x = {}", overlap.x);
            println!("speed_ratio.x = {}", speed_ratio.x);
            correction = overlap.x * speed_ratio.x;
            self.horizontal = Some(CollideeDetails {
                name,
                velocity: velocity_b,
                bounding_box: bb_b.clone(),
                correction,
            });
        } else {
            println!("overlap.y = {}", overlap.y);
            println!("speed_ratio.y = {}", speed_ratio.y);
            correction = overlap.y * speed_ratio.y;
            self.vertical = Some(CollideeDetails {
                name,
                velocity: velocity_b,
                bounding_box: bb_b.clone(),
                correction,
            });
        }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct ColliderNew;