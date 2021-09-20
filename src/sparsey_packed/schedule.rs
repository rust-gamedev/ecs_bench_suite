use sparsey::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

fn ab(mut a: CompMut<A>, mut b: CompMut<B>) {
    (&mut a, &mut b).iter().for_each(|(mut a, mut b)| {
        std::mem::swap(&mut a.0, &mut b.0);
    })
}

fn cd(mut c: CompMut<C>, mut d: CompMut<D>) {
    (&mut c, &mut d).iter().for_each(|(mut c, mut d)| {
        std::mem::swap(&mut c.0, &mut d.0);
    })
}

fn ce(mut c: CompMut<C>, mut e: CompMut<E>) {
    (&mut c, &mut e).iter().for_each(|(mut c, mut e)| {
        std::mem::swap(&mut c.0, &mut e.0);
    })
}

pub struct Benchmark(World, Dispatcher, ThreadPool);

impl Benchmark {
    pub fn new() -> Self {
        let layout = Layout::builder()
            .add_group(<(A, B)>::group())
            .add_group(<(C, D)>::group())
            .build();

        let mut world = World::with_layout(&layout);

        let dispatcher = Dispatcher::builder()
            .add_system(ab.system())
            .add_system(cd.system())
            .add_system(ce.system())
            .build();

        dispatcher.set_up(&mut world);
        world.create_entities((0..10_000).map(|_| (A(0.0), B(0.0))));
        world.create_entities((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0))));
        world.create_entities((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));
        world.create_entities((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(dispatcher.max_concurrecy())
            .build()
            .unwrap();

        Self(world, dispatcher, thread_pool)
    }

    pub fn run(&mut self) {
        self.1.run_par(&mut self.0, &self.2).unwrap();
    }
}
