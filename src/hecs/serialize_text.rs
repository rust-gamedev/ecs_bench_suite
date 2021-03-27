use hecs::{serialize::row::*, *};
use serde::{de::MapAccess, ser::SerializeMap, Deserialize, Serialize};

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Transform([f32; 16]);

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Rotation {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

struct SerContext;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum ComponentId {
    Transform,
    Position,
    Rotation,
    Velocity,
}

impl SerializeContext for SerContext {
    fn serialize_entity<S: SerializeMap>(
        &mut self,
        entity: EntityRef<'_>,
        map: &mut S,
    ) -> Result<(), S::Error> {
        try_serialize::<Transform, _, _>(&entity, &ComponentId::Transform, map)?;
        try_serialize::<Position, _, _>(&entity, &ComponentId::Position, map)?;
        try_serialize::<Rotation, _, _>(&entity, &ComponentId::Rotation, map)?;
        try_serialize::<Velocity, _, _>(&entity, &ComponentId::Velocity, map)?;
        Ok(())
    }
}

struct DeContext;

impl DeserializeContext for DeContext {
    fn deserialize_entity<'de, M>(
        &mut self,
        mut map: M,
        entity: &mut EntityBuilder,
    ) -> Result<(), M::Error>
    where
        M: MapAccess<'de>,
    {
        while let Some(key) = map.next_key()? {
            match key {
                ComponentId::Transform => {
                    entity.add::<Transform>(map.next_value()?);
                }
                ComponentId::Position => {
                    entity.add::<Position>(map.next_value()?);
                }
                ComponentId::Rotation => {
                    entity.add::<Rotation>(map.next_value()?);
                }
                ComponentId::Velocity => {
                    entity.add::<Velocity>(map.next_value()?);
                }
            }
        }
        Ok(())
    }
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        world.spawn_batch((0..1000).map(|_| {
            (
                Transform::default(),
                Position::default(),
                Rotation::default(),
                Velocity::default(),
            )
        }));

        Self(world)
    }

    pub fn run(&mut self) {
        let Self(world) = self;
        let mut encoded = Vec::new();
        serialize(
            &world,
            &mut SerContext,
            &mut ron::Serializer::new(&mut encoded, None, false).unwrap(),
        )
        .unwrap();
        deserialize(
            &mut DeContext,
            &mut ron::Deserializer::from_bytes(&encoded).unwrap(),
        )
        .unwrap();
    }
}
