use std::collections::HashMap;

use super::{entity::Entity, entity_memory_pool::EntityMemoryPool, EntityTag};

type EntityVec = Vec<Entity>;
type EntityMap = HashMap<EntityTag, EntityVec>;

pub struct EntityManager {
    pub entities: EntityVec,
    pub entity_map: EntityMap,
    pub entities_to_add: Vec<(Entity, EntityTag)>,
}

impl EntityManager {
    pub fn new() -> Self {
        let mut entity_map = HashMap::new();
        entity_map.insert(EntityTag::Player, Vec::with_capacity(8));
        entity_map.insert(EntityTag::Bullet, Vec::with_capacity(10000));
        entity_map.insert(EntityTag::Enemy, Vec::with_capacity(10000));
        entity_map.insert(EntityTag::Platform, Vec::with_capacity(10000));
        entity_map.insert(EntityTag::Goal, Vec::with_capacity(1));

        Self {
            entities: Vec::with_capacity(20000),
            entity_map,
            entities_to_add: Vec::with_capacity(10000),
        }
    }

    pub fn add_entity(
        &mut self,
        tag: EntityTag,
        entity_memory_pool: &mut EntityMemoryPool,
    ) -> Entity {
        // Create a new entity
        let entity = entity_memory_pool.add_entity(tag);
        // Add the entity to the entities-to-add list
        self.entities_to_add.push((entity, tag));

        // Return the new entity
        return entity;
    }

    pub fn get_all_entities(&self) -> &EntityVec {
        return &self.entities;
    }

    pub fn get_entities_by_tag(&self, tag: EntityTag) -> Option<&EntityVec> {
        // Return the entities with the given tag
        return self.entity_map.get(&tag);
    }

    pub fn get_entities_by_tag_mut(&mut self, tag: EntityTag) -> Option<&mut EntityVec> {
        // Return the entities with the given tag
        return self.entity_map.get_mut(&tag);
    }

    // pub fn get_entity_by_id(&self, id: usize) -> Option<Rc<RefCell<Entity>>> {
    //     // For each entity in the main entity list
    //     for entity in self.entities.iter() {
    //         // If the entity has the given id
    //         if entity.borrow().id() == id {
    //             // Return the entity
    //             return Some(entity.clone());
    //         }
    //     }

    //     // Return None if no entity has the given id
    //     return None;
    // }

    pub fn get_entity_count_by_tag(&self, tag: EntityTag) -> usize {
        // Return the number of entities with the given tag
        return self.entity_map.get(&tag).unwrap().len();
    }

    pub fn update(&mut self, entity_memory_pool: &mut EntityMemoryPool) {
        // For each entity in the entities-to-add list
        for entity_and_tag in self.entities_to_add.drain(..) {
            // Add the entity to the main entity list
            self.entities.push(entity_and_tag.0);

            // Add the entity to the entity map
            self.entity_map
                .entry(entity_and_tag.1)
                .or_insert(Vec::new())
                .push(entity_and_tag.0);
        }

        // Remove all entities that are not alive from the main entity list
        self.entities
            .retain(|entity| entity_memory_pool.is_active(entity.id()));

        // For every pair in the entity map
        for (_, entities) in self.entity_map.iter_mut() {
            // Remove all entities that are not alive
            entities.retain(|entity| entity_memory_pool.is_active(entity.id()));
        }
    }
}
