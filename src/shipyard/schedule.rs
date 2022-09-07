use shipyard::*;

#[derive(Component)]
struct A(f32);

#[derive(Component)]
struct B(f32);

#[derive(Component)]
struct C(f32);

#[derive(Component)]
struct D(f32);

#[derive(Component)]
struct E(f32);

fn ab(mut a: ViewMut<A>, mut b: ViewMut<B>) {
    (&mut a, &mut b).iter().for_each(|(mut a, mut b)| {
        std::mem::swap(&mut a.0, &mut b.0);
    })
}

fn cd(mut c: ViewMut<C>, mut d: ViewMut<D>) {
    (&mut c, &mut d).iter().for_each(|(mut c, mut d)| {
        std::mem::swap(&mut c.0, &mut d.0);
    })
}

fn ce(mut c: ViewMut<C>, mut e: ViewMut<E>) {
    (&mut c, &mut e).iter().for_each(|(mut c, mut e)| {
        std::mem::swap(&mut c.0, &mut e.0);
    })
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let world = World::default();

        world.run(
            |mut entities: EntitiesViewMut, mut a: ViewMut<A>, mut b: ViewMut<B>| {
                for _ in 0..10_000 {
                    entities.add_entity((&mut a, &mut b), (A(0.0), B(0.0)));
                }
            },
        );

        world.run(
            |mut entities: EntitiesViewMut,
             mut a: ViewMut<A>,
             mut b: ViewMut<B>,
             mut c: ViewMut<C>| {
                for _ in 0..10_000 {
                    entities.add_entity((&mut a, &mut b, &mut c), (A(0.0), B(0.0), C(0.0)));
                }
            },
        );

        world.run(
            |mut entities: EntitiesViewMut,
             mut a: ViewMut<A>,
             mut b: ViewMut<B>,
             mut c: ViewMut<C>,
             mut d: ViewMut<D>| {
                for _ in 0..10_000 {
                    entities.add_entity(
                        (&mut a, &mut b, &mut c, &mut d),
                        (A(0.0), B(0.0), C(0.0), D(0.0)),
                    );
                }
            },
        );

        world.run(
            |mut entities: EntitiesViewMut,
             mut a: ViewMut<A>,
             mut b: ViewMut<B>,
             mut c: ViewMut<C>,
             mut e: ViewMut<E>| {
                for _ in 0..10_000 {
                    entities.add_entity(
                        (&mut a, &mut b, &mut c, &mut e),
                        (A(0.0), B(0.0), C(0.0), E(0.0)),
                    );
                }
            },
        );

        Workload::new("run")
            .with_system(&ab)
            .with_system(&cd)
            .with_system(&ce)
            .add_to_world(&world)
            .unwrap();

        Self(world)
    }

    pub fn run(&mut self) {
        self.0.run_workload("run").unwrap();
    }
}
