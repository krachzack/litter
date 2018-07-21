use aitios_scene::Entity;

pub trait Filter {
    /// Does its thing to mutate the given entities and make them
    /// more complex.
    fn apply(&self, scene: &mut Vec<Entity>);
}
