use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::DispatcherBuilder,
};
use crate::systems::{
    CollisionSystem, KillBulletsSystem, LimitObjectsSystem, PhysicsSystem, RandomAsteroidSystem,
    ShipInputSystem,
};

pub struct MainBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MainBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(KillBulletsSystem, "kill_bullets", &[]);
        builder.add(RandomAsteroidSystem::new(), "random_asteroids", &[]);
        builder.add(
            ShipInputSystem,
            "ship_input_system",
            &["random_asteroids", "kill_bullets", "input_system"],
        );
        builder.add(PhysicsSystem, "physics_system", &["ship_input_system"]);
        builder.add(LimitObjectsSystem, "limit_objects", &["physics_system"]);
        builder.add(CollisionSystem, "collisions", &["physics_system"]);
        Ok(())
    }
}
