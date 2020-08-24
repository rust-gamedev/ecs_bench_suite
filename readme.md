# ECS Bench Suite

A suite of benchmarks designed to test and compare Rust ECS library performance across a variety of challenging circumstances.

The full benchmark report is available [here](https://rust-gamedev.github.io/ecs_bench_suite/target/criterion/report/index.html).

|                  | legion (\*)         | legion 0.2.4 | bevy     | hecs    | shipyard (\*)         | specs       |
|------------------|:-------------------:|:------------:|:--------:|:-------:|:---------------------:|:-----------:|
| simple_insert    | **0.434ms**         | 1.06ms       | 0.865ms  | 0.645ms | 2.49ms                | 2.32ms      |
| simple_iter      | **13.4us** (16.6us) | **13.4us**   | *14.4us* | 26.7us  | 110us (45.6us)        | 34.3ms      |
| frag_iter        | **0.509us**         | 1.78us       | 1.76us   | 1.79us  | 1.04us                | 1.67us      |
| heavy_compute    | *0.701ms* (0.723ms) | 4.34ms       | 1.06ms   | 1.02ms  | 0.778ms (**0.700ms**) | 0.995ms     |
| schedule         | **52.3us** (53.7us) | 151us        | 94.9us   | -       | 580us (307us)         | 244us       |
| add_remove       | 5.50ms              | 3.07ms       | -        | 18.2ms  | 2.90ms                | **0.148ms** |
| serialize_text   | **17.9ms**          | -            | -        | -       | -                     | -           |
| serialize_binary | **6.42ms**          | -            | -        | -       | -                     | -           |

(*): Per-benchmark storage optimizations. Some of these are mutually exclusive, so with and without "packing" typically represent best and worst-case performance for the ECS.

Note: Run to run variance for these benchmarks is typically 2-3%, with outliers as much as 10%. All micro-benchmarks should be taken with a grain of salt, and any benchmarks within a few percent of each other should be considered "effectively equal".

## The Benchmarks

### Simple Insert

This benchmark is designed to test the base cost of constructing entities and moving components into the ECS.

Inserts 10,000 entities, each with 4 components: `Transform(mat4x4)`, `Position(vec3)`, `Rotation(vec3)` and `Velocity(vec3)`.

### Simple Iter

This benchmark is designed to test the core overheads involved in component iteration in best-case conditions. The iteration should occur on a single CPU core.

Dataset: 10,000 entities, each with 4 components: `Transform(mat4x4)`, `Position(vec3)`, `Rotation(vec3)` and `Velocity(vec3)`.

Test: Iterate through all entities with `Position` and `Velocity`, and add velocity onto position.

### Fragmented Iter

This benchmark is designed to test how the ECS handles iteration through a fragmented dataset. The iteration should occur on a single CPU core.

Dataset: 26 component types (`A(f32)` through `Z(f32)`), each with 20 entities plus a `Data(f32)` component.

Test: Iterate through all entities with a `Data` component and double its value.

### System Scheduling

This benchmark is designed to test how efficiently the ECS can schedule multiple independent systems on a multi-core CPU. This is primarily an outer-parallelism test. Each system should execute on a single CPU core.

Dataset:

* 10,000 entities with `(A, B)` components.
* 10,000 entities with `(A, B, C)` components.
* 10,000 entities with `(A, B, C, D)` components.
* 10,000 entities with `(A, B, C, E)` components.

Test:

Three systems accessing the following components mutably, where each system swaps the values stored in each component:

* `(A, B)`
* `(C, D)`
* `(C, E)`

### Heavy Compute

This benchmark is designed to test the ECS's ability to scale when it is allowed to run a system over multiple CPU cores. This is primarily an inner-parallelism test.

Dataset: 10,000 entities with a `mat4x4` component.

Test: Iterate through all `mat4x4` components, and invert the matrix 10 times.

### Add/Remove Component

This benchmark is designed to test how quickly the ECS can add and then remove a component from an existing entity.

Dataset: 1,000 entities with a single `A` component.

Test: Iterate through all entities, adding a `B` component. Then iterate through all entities again, removing their `B` component.

### Serialize

This benchmark is designed to test how quickly the ECS and serialize and deserialize its entities in both text (RON) and binary (bincode) formats.

Dataset: 1000 entities with `Transform(mat4x4)`, `Position(vec3)`, `Rotation(vec3)` and `Velocity(vec3)` components.

Test: Serialize all entities to RON and bincode formats in-memory. Then deserialize back into the ECS. The RON and bincode formats should be separate benchmark tests.
