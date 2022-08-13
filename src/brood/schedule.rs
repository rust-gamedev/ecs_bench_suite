use brood::{
    entities,
    query::{filter, result, views},
    registry,
    system::{schedule, Schedule, System},
    World,
};

#[derive(Clone)]
struct A(f32);
#[derive(Clone)]
struct B(f32);
#[derive(Clone)]
struct C(f32);
#[derive(Clone)]
struct D(f32);
#[derive(Clone)]
struct E(f32);

type Registry = registry!(A, B, C, D, E);

struct AB;

impl<'a> System<'a> for AB {
    type Views = views!(&'a mut A, &'a mut B);
    type Filter = filter::None;

    fn run<R>(&mut self, query_results: result::Iter<'a, R, Self::Filter, Self::Views>)
    where
        R: brood::registry::Registry + 'a,
    {
        for result!(a, b) in query_results {
            std::mem::swap(&mut a.0, &mut b.0);
        }
    }
}

struct CD;

impl<'a> System<'a> for CD {
    type Views = views!(&'a mut C, &'a mut D);
    type Filter = filter::None;

    fn run<R>(&mut self, query_results: result::Iter<'a, R, Self::Filter, Self::Views>)
    where
        R: brood::registry::Registry + 'a,
    {
        for result!(c, d) in query_results {
            std::mem::swap(&mut c.0, &mut d.0);
        }
    }
}

struct CE;

impl<'a> System<'a> for CE {
    type Views = views!(&'a mut C, &'a mut E);
    type Filter = filter::None;

    fn run<R>(&mut self, query_results: result::Iter<'a, R, Self::Filter, Self::Views>)
    where
        R: brood::registry::Registry + 'a,
    {
        for result!(c, e) in query_results {
            std::mem::swap(&mut c.0, &mut e.0);
        }
    }
}

pub struct Benchmark(
    World<Registry>,
    Schedule<
        schedule::stages! {
            system: AB,
            system: CD,
            system: CE,
        },
    >,
);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        world.extend(entities!((A(0.0), B(0.0)); 10_000));
        world.extend(entities!((A(0.0), B(0.0), C(0.0)); 10_000));
        world.extend(entities!((A(0.0), B(0.0), C(0.0), D(0.0)); 10_000));
        world.extend(entities!((A(0.0), B(0.0), C(0.0), E(0.0)); 10_000));

        let schedule = Schedule::builder().system(AB).system(CD).system(CE).build();

        Self(world, schedule)
    }

    pub fn run(&mut self) {
        self.0.run_schedule(&mut self.1);
    }
}
