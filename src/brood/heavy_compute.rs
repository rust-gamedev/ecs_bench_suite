use brood::{
    entities,
    query::{filter, result, views},
    registry, World,
};
use cgmath::{Matrix4, Rad, SquareMatrix, Transform, Vector3};
use rayon::iter::ParallelIterator;

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

type Registry = registry!(Position, Rotation, Velocity, Matrix4<f32>);

pub struct Benchmark(World<Registry>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        world.extend(entities!((
            Matrix4::<f32>::from_angle_x(Rad(1.2)),
            Position(Vector3::unit_x()),
            Rotation(Vector3::unit_x()),
            Velocity(Vector3::unit_x())
        ); 1_000));

        Self(world)
    }

    pub fn run(&mut self) {
        self.0
            .par_query::<views!(&mut Position, &mut Matrix4<f32>), filter::None>()
            .for_each(|result!(position, matrix)| {
                for _ in 0..100 {
                    *matrix = matrix.invert().unwrap();
                }
                position.0 = matrix.transform_vector(position.0);
            });
    }
}
