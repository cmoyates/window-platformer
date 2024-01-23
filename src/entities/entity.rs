use crate::components::Component;

use super::entity_memory_pool::EntityMemoryPool;

#[derive(Clone, Copy)]
pub struct Entity {
    id: usize,
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn id(&self) -> usize {
        return self.id;
    }

    pub fn get_component_mut<'a, T: Component>(
        &'a mut self,
        entity_memory_pool: &'a mut EntityMemoryPool,
    ) -> Option<&'a mut T> {
        return entity_memory_pool.get_component_mut::<T>(self.id);
    }

    pub fn get_component<'a, T: Component>(
        &'a self,
        entity_memory_pool: &'a EntityMemoryPool,
    ) -> Option<&'a T> {
        return entity_memory_pool.get_component::<T>(self.id);
    }

    // pub fn has_component<T: Component>(&self, entity_memory_pool: &EntityMemoryPool) -> bool {
    //     return entity_memory_pool.has_component::<T>(self.id);
    // }

    pub fn destroy(&mut self, entity_memory_pool: &mut EntityMemoryPool) {
        entity_memory_pool.destroy_entity(self.id);
    }

    pub fn add_component<T: Component>(&mut self, entity_memory_pool: &mut EntityMemoryPool) {
        entity_memory_pool.add_component::<T>(self.id);
    }
}
