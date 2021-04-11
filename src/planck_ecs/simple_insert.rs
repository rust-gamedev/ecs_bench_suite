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

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        let mut entities = Entities::default();
        let mut comp1 = Components::<Transform>::default();
        let mut comp2 = Components::<Position>::default();
        let mut comp3 = Components::<Rotation>::default();
        let mut comp4 = Components::<Velocity>::default();

        let en = (0..10000).map(|_| entities.create()).collect::<Vec<_>>();
        en.iter().for_each(|e| {comp1.insert(*e, Transform(Matrix4::<f32>::from_scale(1.0)));});
        en.iter().for_each(|e| {comp2.insert(*e, Position(Vector3::unit_x()));});
        en.iter().for_each(|e| {comp3.insert(*e, Rotation(Vector3::unit_x()));});
        en.iter().for_each(|e| {comp4.insert(*e, Velocity(Vector3::unit_x()));});

        /*(0..10000).for_each(|_| {
            let e = entities.create();
            comp1.insert(e, Transform(Matrix4::<f32>::from_scale(1.0)));
            comp2.insert(e, Position(Vector3::unit_x()));
            comp3.insert(e, Rotation(Vector3::unit_x()));
            comp4.insert(e, Velocity(Vector3::unit_x()));
        });*/
    }
}
