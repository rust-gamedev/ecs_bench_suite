use cgmath::*;
use legion::*;
use query::Query;
use storage::PackOptions;

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World, Query<(Write<Position>, Write<Matrix4<f32>>)>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.extend((0..1000).map(|_| {
            (
                Matrix4::<f32>::from_angle_x(Rad(1.2)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));
        world.pack(PackOptions::force());

        let query = <(Write<Position>, Write<Matrix4<f32>>)>::query();

        Self(world, query)
    }

    pub fn run(&mut self) {
        self.1.par_for_each_mut(&mut self.0, |(pos, mat)| {
            for _ in 0..100 {
                *mat = mat.invert().unwrap();
            }
            pos.0 = mat.transform_vector(pos.0);
        });
    }
}
