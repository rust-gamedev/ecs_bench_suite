use cgmath::*;
use legion_2_4::prelude::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.insert(
            (),
            (0..10_000).map(|_| {
                (
                    Transform(Matrix4::from_scale(1.0)),
                    Position(Vector3::unit_x()),
                    Rotation(Vector3::unit_x()),
                    Velocity(Vector3::unit_x()),
                )
            }),
        );

        Self(world)
    }

    pub fn run(&mut self) {
        let query = <(Read<Velocity>, Write<Position>)>::query();
        query.for_each_mut(&mut self.0, |(velocity, mut position)| {
            position.0 += velocity.0;
        });
    }
}
