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
struct SimpleIterSystem;

impl<'a> System<'a> for SimpleIterSystem {
    type SystemData = (WriteStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (mut velocity_storage, mut position_storage): Self::SystemData) {
        for (velocity, position) in (&mut velocity_storage, &mut position_storage).join() {
            position.0 += velocity.0;
        }
    }
}
pub struct Benchmark(World, SimpleIterSystem);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        world.register::<Transform>();
        world.register::<Position>();
        world.register::<Rotation>();
        world.register::<Velocity>();
        (0..10000).for_each(|_| {
            world
                .create_entity()
                .with(Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))))
                .with(Position(Vector3::unit_x()))
                .with(Rotation(Vector3::unit_x()))
                .with(Velocity(Vector3::unit_x()))
                .build();
        });

        Self(world, SimpleIterSystem)
    }

    pub fn run(&mut self) {
        self.1.run_now(&self.0);
    }
}
