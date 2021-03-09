use planck_ecs::*;
struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

fn ab_system(a_store: &mut Components<A>, b_store: &mut Components<B>) -> SystemResult {
    for (a, b) in join!(&mut a_store && &mut b_store) {
        std::mem::swap(&mut a.unwrap().0, &mut b.unwrap().0);
    }
    Ok(())
}

fn cd_system(c_store: &mut Components<C>, d_store: &mut Components<D>) -> SystemResult {
    for (c, d) in join!(&mut c_store && &mut d_store) {
        std::mem::swap(&mut c.unwrap().0, &mut d.unwrap().0);
    }
    Ok(())
}

fn ce_system(c_store: &mut Components<C>, e_store: &mut Components<E>) -> SystemResult {
    for (c, e) in join!(&mut c_store && &mut e_store) {
        std::mem::swap(&mut c.unwrap().0, &mut e.unwrap().0);
    }
    Ok(())
}

pub struct Benchmark(World, Dispatcher);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world.initialize::<Entities>();
        world.initialize::<Components<A>>();
        world.initialize::<Components<B>>();
        world.initialize::<Components<C>>();
        world.initialize::<Components<D>>();
        world.initialize::<Components<E>>();
        (0..10000).for_each(|_| {
            let e = world.get_mut::<Entities>().unwrap().create();
            world.get_mut::<Components<_>>().unwrap().insert(e, A(0.0));
        });
        (0..10000).for_each(|_| {
            let e = world.get_mut::<Entities>().unwrap().create();
            world.get_mut::<Components<_>>().unwrap().insert(e, A(0.0));
            world.get_mut::<Components<_>>().unwrap().insert(e, B(0.0));
        });
        (0..10000).for_each(|_| {
            let e = world.get_mut::<Entities>().unwrap().create();
            world.get_mut::<Components<_>>().unwrap().insert(e, A(0.0));
            world.get_mut::<Components<_>>().unwrap().insert(e, B(0.0));
            world.get_mut::<Components<_>>().unwrap().insert(e, C(0.0));
        });
        (0..10000).for_each(|_| {
            let e = world.get_mut::<Entities>().unwrap().create();
            world.get_mut::<Components<_>>().unwrap().insert(e, A(0.0));
            world.get_mut::<Components<_>>().unwrap().insert(e, B(0.0));
            world.get_mut::<Components<_>>().unwrap().insert(e, C(0.0));
            world.get_mut::<Components<_>>().unwrap().insert(e, D(0.0));
        });
        (0..10000).for_each(|_| {
            let e = world.get_mut::<Entities>().unwrap().create();
            world.get_mut::<Components<_>>().unwrap().insert(e, A(0.0));
            world.get_mut::<Components<_>>().unwrap().insert(e, B(0.0));
            world.get_mut::<Components<_>>().unwrap().insert(e, C(0.0));
            world.get_mut::<Components<_>>().unwrap().insert(e, E(0.0));
        });

        let dispatcher = DispatcherBuilder::new()
            .add(ab_system)
            .add(cd_system)
            .add(ce_system)
            .build(&mut world);

        Self(world, dispatcher)
    }

    pub fn run(&mut self) {
        self.1.run_par(&self.0).unwrap();
    }
}
