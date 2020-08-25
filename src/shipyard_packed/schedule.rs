use shipyard::*;

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

fn ab(mut a: ViewMut<A>, mut b: ViewMut<B>) {
    (&mut a, &mut b).iter().for_each(|(a, b)| {
        std::mem::swap(&mut a.0, &mut b.0);
    })
}

fn cd(mut c: ViewMut<C>, mut d: ViewMut<D>) {
    (&mut c, &mut d).iter().for_each(|(c, d)| {
        std::mem::swap(&mut c.0, &mut d.0);
    })
}

fn ce(mut c: ViewMut<C>, mut e: ViewMut<E>) {
    (&mut c, &mut e).iter().for_each(|(c, e)| {
        std::mem::swap(&mut c.0, &mut e.0);
    })
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let world = World::default();

        world.run(
            |mut a: ViewMut<A>,
             mut b: ViewMut<B>,
             mut c: ViewMut<C>,
             mut d: ViewMut<D>,
             mut e: ViewMut<E>| {
                (&mut a, &mut b).tight_pack();
                (&mut c, &mut d).tight_pack();

                (&mut e, &mut c).loose_pack();
            },
        );

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

        world
            .add_workload("run")
            .with_system(system!(ab))
            .with_system(system!(cd))
            .with_system(system!(ce))
            .build();

        Self(world)
    }

    pub fn run(&mut self) {
        self.0.run_workload("run");
    }
}
