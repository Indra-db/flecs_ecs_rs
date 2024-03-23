mod common;
use common::*;

// This example shows how to use union relationships. Union relationships behave
// much like exclusive relationships in that entities can have only one instance
// and that adding an instance removes the previous instance.
//
// What makes union relationships stand out is that changing the relationship
// target doesn't change the archetype of an entity. This allows for quick
// switching of tags, which can be useful when encoding state machines in ECS.
//
// There is a tradeoff, and that is that because a single archetype can contain
// entities with multiple targets, queries need to do a bit of extra work to
// only return the requested target.
//
// This code uses enumeration relationships. See the enum_relations example for
// more details.

#[derive(Clone, Component, Debug, Default, PartialEq)]
#[repr(C)]
enum Movement {
    #[default]
    Walking,
    Running,
}

#[derive(Clone, Component, Debug, Default, PartialEq)]
#[repr(C)]
enum Direction {
    #[default]
    Front,
    Back,
    Left,
    Right,
}
fn main() {
    let world = World::new();

    // Register Movement and Direction as union relationships. This ensures that
    // an entity can only have one Movement and one Direction.
    world.component::<Movement>().add_id(ECS_UNION);
    world.component::<Direction>().add_id(ECS_UNION);

    // Create a query that subscribes for all entities that have a Direction
    // and that are walking.
    // with<T>() requests no data by default, so we must specify what we want.
    // in() requests Read-Only
    let q = world
        .query_builder::<()>()
        .with_enum(Movement::Walking)
        .in_()
        .with_enum_wildcard::<Direction>()
        .in_()
        .build();

    // Create a few entities with various state combinations
    world
        .new_entity_named(c"e1")
        .add_enum_constant(Movement::Walking)
        .add_enum_constant(Direction::Front);

    world
        .new_entity_named(c"e2")
        .add_enum_constant(Movement::Running)
        .add_enum_constant(Direction::Left);

    let e3 = world
        .new_entity_named(c"e3")
        .add_enum_constant(Movement::Running)
        .add_enum_constant(Direction::Back);

    // Add Walking to e3. This will remove the Running case
    e3.add_enum_constant(Movement::Walking);

    // Iterate the query
    q.iter_only(|it| {
        // Get the column with direction states. This is stored as an array
        // with identifiers to the individual states
        let movement = unsafe { it.get_field_data::<Movement>(1) };
        let direction = unsafe { it.get_field_data::<Direction>(2) };

        for i in 0..it.count() {
            println!(
                "{}: Movement: {:?}, Direction: {:?}",
                it.get_entity(i).get_name(),
                movement[i],
                direction[i]
            );
        }
    });

    // Output:
    //   e3: Movement: Walking, Direction: Back
    //   e1: Movement: Walking, Direction: Front
}
