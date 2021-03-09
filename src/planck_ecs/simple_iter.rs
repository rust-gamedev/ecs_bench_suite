use cgmath::*;
use planck_ecs::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);
#[derive(Copy, Clone)]
struct Position(Vector3<f32>);
#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);
#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

fn simple_iter_system(velocity_storage: &mut Components<Velocity>, position_storage: &mut Components<Position>) -> SystemResult {
    for (velocity, mut position) in join!(&mut velocity_storage && &mut position_storage) {
        position.as_mut().unwrap().0 += velocity.unwrap().0;
    }
    Ok(())
}
pub struct Benchmark(World, System);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world.initialize::<Entities>();
        world.initialize::<Components<Transform>>();
        world.initialize::<Components<Position>>();
        world.initialize::<Components<Rotation>>();
        world.initialize::<Components<Velocity>>();
        (0..10000).for_each(|_| {
            let e = world.get_mut::<Entities>().unwrap().create();
            world.get_mut::<Components<_>>().unwrap().insert(e, Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))));
            world.get_mut::<Components<_>>().unwrap().insert(e, Position(Vector3::unit_x()));
            world.get_mut::<Components<_>>().unwrap().insert(e, Rotation(Vector3::unit_x()));
            world.get_mut::<Components<_>>().unwrap().insert(e, Velocity(Vector3::unit_x()));
        });

        Self(world, simple_iter_system.system())
    }

    pub fn run(&mut self) {
        self.1.run(&self.0).unwrap();
    }
}
