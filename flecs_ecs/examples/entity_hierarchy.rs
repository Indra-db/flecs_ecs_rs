mod common;
use common::*;

#[derive(Debug, Component)]
struct Star;

#[derive(Debug, Component)]
struct Planet;

#[derive(Debug, Component)]
struct Moon;

fn iterate_tree(entity: EntityView, position_parent: &Position, snap: &mut Snap) {
    // Print hierarchical name of entity & the entity type
    fprintln!(
        snap,
        "{} [{:?}]",
        entity.path().unwrap(),
        entity.archetype()
    );

    // Get the position of the entity
    let pos = entity.try_get::<Position>().unwrap();

    // Calculate actual position
    let pos_actual = Position {
        x: pos.x + position_parent.x,
        y: pos.y + position_parent.y,
    };

    // Print the position
    fprintln!(snap, "{:?}", pos_actual);

    entity.for_each_child_of(|child| {
        iterate_tree(child, &pos_actual, snap);
    });
}

fn main() {
    //ignore snap in example, it's for snapshot testing
    let mut snap = Snap::setup_snapshot_test();

    let world = World::new();

    // Create a simple hierarchy.
    // Hierarchies use ECS relationships and the builtin flecs::ChildOf relationship to
    // create entities as children of other entities.

    let sun = world.entity_named(c"Sun").set(Position { x: 1.0, y: 1.0 });

    world
        .entity_named(c"Mercury")
        .set(Position { x: 1.0, y: 1.0 })
        .add::<Planet>()
        .child_of_id(sun); // Shortcut for add(flecs::ChildOf, sun)

    world
        .entity_named(c"Venus")
        .set(Position { x: 2.0, y: 2.0 })
        .add::<Planet>()
        .child_of_id(sun);

    let earth = world
        .entity_named(c"Earth")
        .set(Position { x: 3.0, y: 3.0 })
        .add::<Planet>()
        .child_of_id(sun);

    let moon = world
        .entity_named(c"Moon")
        .set(Position { x: 0.1, y: 0.1 })
        .add::<Moon>()
        .child_of_id(earth);

    // Is the Moon a child of the Earth?
    fprintln!(
        snap,
        "Is the Moon a child of the Earth? {} / {}",
        moon.has_id((flecs::ChildOf::ID, earth)), //or you can do
        moon.has_first::<flecs::ChildOf>(earth)
    );

    fprintln!(snap);

    // Do a depth-first traversal of the tree
    iterate_tree(sun, &Position { x: 0.0, y: 0.0 }, &mut snap);

    snap.test();

    // Output:
    //  Is the Moon a child of the Earth? true / true
    //  ::Sun [Position, (Identifier,Name)]
    //  Position { x: 1.0, y: 1.0 }
    //  ::Sun::Mercury [Position, Planet, (Identifier,Name), (ChildOf,Sun)]
    //  Position { x: 2.0, y: 2.0 }
    //  ::Sun::Venus [Position, Planet, (Identifier,Name), (ChildOf,Sun)]
    //  Position { x: 3.0, y: 3.0 }
    //  ::Sun::Earth [Position, Planet, (Identifier,Name), (ChildOf,Sun)]
    //  Position { x: 4.0, y: 4.0 }
    //  ::Sun::Earth::Moon [Component, Position, Sun.Earth.Moon, (Identifier,Name), (Identifier,Symbol), (ChildOf,Sun.Earth), (OnDelete,Panic)]
    //  Position { x: 4.1, y: 4.1 }
}
