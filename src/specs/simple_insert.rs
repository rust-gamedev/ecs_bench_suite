use cgmath::*;
use specs::prelude::*;
use specs_derive::*;

#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct Transform(Matrix4<f32>);
#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct Velocity(Vector3<f32>);

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        let mut world = World::new();
        world.register::<Transform>();
        world.register::<Position>();
        world.register::<Rotation>();
        world.register::<Velocity>();
        (0..10000).for_each(|_| {
            world
                .create_entity()
                .with(Transform(Matrix4::<f32>::from_scale(1.0)))
                .with(Position(Vector3::unit_x()))
                .with(Rotation(Vector3::unit_x()))
                .with(Velocity(Vector3::unit_x()))
                .build();
        });
    }
}
