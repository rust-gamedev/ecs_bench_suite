use specs::prelude::*;
use specs_derive::*;
#[derive(Component)]
#[storage(VecStorage)]
struct A(f32);
#[derive(Component)]
#[storage(VecStorage)]
struct B(f32);
#[derive(Component)]
#[storage(VecStorage)]
struct C(f32);
#[derive(Component)]
#[storage(VecStorage)]
struct D(f32);
#[derive(Component)]
#[storage(VecStorage)]
struct E(f32);

struct ABSystem;

impl<'a> System<'a> for ABSystem {
    type SystemData = (WriteStorage<'a, A>, WriteStorage<'a, B>);

    fn run(&mut self, (mut a_store, mut b_store): Self::SystemData) {
        for (a, b) in (&mut a_store, &mut b_store).join() {
            std::mem::swap(&mut a.0, &mut b.0);
        }
    }
}

struct CDSystem;

impl<'a> System<'a> for CDSystem {
    type SystemData = (WriteStorage<'a, C>, WriteStorage<'a, D>);

    fn run(&mut self, (mut c_store, mut d_store): Self::SystemData) {
        for (c, d) in (&mut c_store, &mut d_store).join() {
            std::mem::swap(&mut c.0, &mut d.0);
        }
    }
}
struct CESystem;

impl<'a> System<'a> for CESystem {
    type SystemData = (WriteStorage<'a, C>, WriteStorage<'a, E>);

    fn run(&mut self, (mut c_store, mut e_store): Self::SystemData) {
        for (c, e) in (&mut c_store, &mut e_store).join() {
            std::mem::swap(&mut c.0, &mut e.0);
        }
    }
}

pub struct Benchmark<'a>(World, Dispatcher<'a, 'a>);

impl Benchmark<'_> {
    pub fn new() -> Self {
        let mut world = World::new();
        world.register::<A>();
        world.register::<B>();
        world.register::<C>();
        world.register::<D>();
        world.register::<E>();
        (0..10000).for_each(|_| {
            world.create_entity().with(A(0.0)).build();
        });
        (0..10000).for_each(|_| {
            world.create_entity().with(A(0.0)).with(B(0.0)).build();
        });
        (0..10000).for_each(|_| {
            world
                .create_entity()
                .with(A(0.0))
                .with(B(0.0))
                .with(C(0.0))
                .build();
        });
        (0..10000).for_each(|_| {
            world
                .create_entity()
                .with(A(0.0))
                .with(B(0.0))
                .with(C(0.0))
                .with(D(0.0))
                .build();
        });
        (0..10000).for_each(|_| {
            world
                .create_entity()
                .with(A(0.0))
                .with(B(0.0))
                .with(C(0.0))
                .with(E(0.0))
                .build();
        });

        let dispatcher = DispatcherBuilder::new()
            .with(ABSystem, "ab", &[])
            .with(CDSystem, "cd", &[])
            .with(CESystem, "ce", &[])
            .build();

        Self(world, dispatcher)
    }

    pub fn run(&mut self) {
        self.1.dispatch_par(&self.0)
    }
}
