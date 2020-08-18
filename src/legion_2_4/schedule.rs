use legion_2_4::prelude::*;

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

fn ab_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("ab")
        .with_query(<(Write<A>, Write<B>)>::query())
        .build(|_, world, _, query| {
            for (mut a, mut b) in query.iter_mut(world) {
                std::mem::swap(&mut a.0, &mut b.0);
            }
        })
}

fn cd_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("cd")
        .with_query(<(Write<C>, Write<D>)>::query())
        .build(|_, world, _, query| {
            for (mut c, mut d) in query.iter_mut(world) {
                std::mem::swap(&mut c.0, &mut d.0);
            }
        })
}

fn ce_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("ce")
        .with_query(<(Write<C>, Write<E>)>::query())
        .build(|_, world, _, query| {
            for (mut c, mut e) in query.iter_mut(world) {
                std::mem::swap(&mut c.0, &mut e.0);
            }
        })
}

pub struct Benchmark(World, Resources, Schedule);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.insert((), (0..10000).map(|_| (A(0.0), B(0.0))));

        world.insert((), (0..10000).map(|_| (A(0.0), B(0.0), C(0.0))));

        world.insert((), (0..10000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));

        world.insert((), (0..10000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        let schedule = Schedule::builder()
            .add_system(ab_system())
            .add_system(cd_system())
            .add_system(ce_system())
            .build();

        Self(world, Resources::default(), schedule)
    }

    pub fn run(&mut self) {
        self.2.execute(&mut self.0, &mut self.1);
    }
}
