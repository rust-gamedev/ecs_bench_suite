use cgmath::*;
use sparsey::prelude::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        let mut world = World::default();
        world.register::<Transform>();
        world.register::<Position>();
        world.register::<Rotation>();
        world.register::<Velocity>();

        world.create_entities((0..10_000).map(|_| (
            Transform(Matrix4::from_scale(1.0)),
            Position(Vector3::unit_x()),
            Rotation(Vector3::unit_x()),
            Velocity(Vector3::unit_x()),
        )));
    }
}
