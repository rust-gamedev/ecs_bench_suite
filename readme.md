# ECS Bench Suite

A suite of benchmarks designed to test and compare Rust ECS library performance across a variety of challenging circumstances.

The full benchmark report is available [here](https://rust-gamedev.github.io/ecs_bench_suite/target/criterion/report/index.html).

|                  | legion (\*)           | bevy       | hecs       | shipyard (\*)         | specs       |
|------------------|:----------------------|:----------:|:----------:|:---------------------:|:-----------:|
| simple_insert    | **383μs**             | 636μs      | 640μs      | 2.08ms                | 1.90ms      |
| simple_iter      | 13.2μs (**11.2μs**)   | 12.9μs     | **12.0μs** | 86.3μs (24.2μs)       | 28.8ms      |
| frag_iter        | 441ns                 | 554ns      | 452ns      | **121ns**             | 1.41μs      |
| heavy_compute    | **686μs** (687μs)     | 958μs      | 972μs      | **693μs** (693μs)     | 968μs       |
| schedule         | **54.3μs** (53.7μs)   | 80.3μs     | -          | 372μs (132μs)         | 155μs       |
| add_remove       | 4.45ms                | 6.71ms     | 7.86ms     | 237μs                 | **123μs**   |
| serialize_text   | **12.5ms**            | -          | -          | -                     | -           |
| serialize_binary | **6.50ms**            | -          | -          | -                     | -           |

(*): The values in parentheses are results where per-benchmark storage optimizations were applied. Some of these are mutually exclusive, so with and without "packing" typically represent best and worst-case performance for the ECS.

The best result for each benchmark is marked in bold text. Note that run to run variance for these benchmarks is typically 2-3%, with outliers as much as 10%. All micro-benchmarks should be taken with a grain of salt, and any benchmarks within a few percent of each other should be considered "effectively equal".

![](./target/criterion/add_remove_component/report/violin.svg)
![](./target/criterion/fragmented_iter/report/violin.svg)
![](./target/criterion/heavy_compute/report/violin.svg)
![](./target/criterion/schedule/report/violin.svg)
![](./target/criterion/simple_insert/report/violin.svg)
![](./target/criterion/simple_iter/report/violin.svg)

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
