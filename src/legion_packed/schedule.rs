use legion::*;
use storage::PackOptions;

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

#[system(for_each)]
fn ab(a: &mut A, b: &mut B) {
    std::mem::swap(&mut a.0, &mut b.0);
}

#[system(for_each)]
fn cd(c: &mut C, d: &mut D) {
    std::mem::swap(&mut c.0, &mut d.0);
}

#[system(for_each)]
fn ce(c: &mut C, e: &mut E) {
    std::mem::swap(&mut c.0, &mut e.0);
}

pub struct Benchmark(World, Resources, Schedule);

impl Benchmark {
    pub fn new() -> Self {
        let options = WorldOptions {
            groups: vec![<(A, B)>::to_group(), <(C, D)>::to_group()],
        };

        let mut world = World::new(options);

        world.extend((0..10000).map(|_| (A(0.0), B(0.0))));

        world.extend((0..10000).map(|_| (A(0.0), B(0.0), C(0.0))));

        world.extend((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));

        world.extend((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        world.pack(PackOptions::force());

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
