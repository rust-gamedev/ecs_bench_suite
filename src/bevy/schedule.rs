use bevy_ecs::{prelude::*, Mut, Schedule};

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

fn ab(mut a: Mut<A>, mut b: Mut<B>) {
    std::mem::swap(&mut a.0, &mut b.0);
}

fn cd(mut c: Mut<C>, mut d: Mut<D>) {
    std::mem::swap(&mut c.0, &mut d.0);
}

fn ce(mut c: Mut<C>, mut e: Mut<E>) {
    std::mem::swap(&mut c.0, &mut e.0);
}

pub struct Benchmark(World, Resources, Schedule);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        let mut schedule = Schedule::default();
        schedule.add_stage("main");
        schedule.add_system_to_stage("main", ab.system());
        schedule.add_system_to_stage("main", cd.system());
        schedule.add_system_to_stage("main", ce.system());

        Self(world, Resources::default(), schedule)
    }

    pub fn run(&mut self) {
        self.2.run(&mut self.0, &mut self.1);
    }
}
