use bevy_ecs::{prelude::*, schedule::Schedule};

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

fn ab(mut query: Query<(&mut A, &mut B)>) {
    for (mut a, mut b) in query.iter_mut() {
        std::mem::swap(&mut a.0, &mut b.0);
    }
}

fn cd(mut query: Query<(&mut C, &mut D)>) {
    for (mut c, mut d) in query.iter_mut() {
        std::mem::swap(&mut c.0, &mut d.0);
    }
}

fn ce(mut query: Query<(&mut C, &mut E)>) {
    for (mut c, mut e) in query.iter_mut() {
        std::mem::swap(&mut c.0, &mut e.0);
    }
}

pub struct Benchmark(World, Schedule);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        let mut schedule = Schedule::default();
        schedule.add_stage("main", SystemStage::parallel());
        schedule.add_system_to_stage("main", ab.system());
        schedule.add_system_to_stage("main", cd.system());
        schedule.add_system_to_stage("main", ce.system());

        Self(world, schedule)
    }

    pub fn run(&mut self) {
        self.1.run(&mut self.0);
    }
}
