use std::{any::TypeId, sync::Mutex};

use once_cell::sync::Lazy;
use sfml::system::Vector2;

use crate::components::{CInput, CLifetime, CTransform, Component};

use super::{entity::Entity, EntityTag};

type EntityComponentVectorTuple = (Vec<CTransform>, Vec<CLifetime>, Vec<CInput>);

pub static POOL: Lazy<Mutex<EntityMemoryPool>> =
    Lazy::new(|| Mutex::new(EntityMemoryPool::new(20000)));

pub struct EntityMemoryPool {
    entity_count: usize,
    pool: EntityComponentVectorTuple,
    tags: Vec<EntityTag>,
    active: Vec<bool>,
}

impl EntityMemoryPool {
    pub fn new(entity_count: usize) -> Self {
        let mut pool = (
            Vec::<CTransform>::with_capacity(entity_count),
            Vec::<CLifetime>::with_capacity(entity_count),
            Vec::<CInput>::with_capacity(entity_count),
        );

        let mut tags = Vec::<EntityTag>::with_capacity(entity_count);
        let mut active = Vec::<bool>::with_capacity(entity_count);

        for _ in 0..entity_count {
            pool.0.push(CTransform::new());
            pool.1.push(CLifetime::new(0.0));
            pool.2.push(CInput::new());

            tags.push(EntityTag::None);
            active.push(false);
        }

        Self {
            entity_count,
            pool,
            tags,
            active,
        }
    }

    pub fn get_component<T: Component>(&self, id: usize) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        match type_id {
            x if x == TypeId::of::<CTransform>() => {
                Some(&self.pool.0[id].as_any().downcast_ref().unwrap() as &T)
            }
            x if x == TypeId::of::<CLifetime>() => {
                Some(&self.pool.1[id].as_any().downcast_ref().unwrap() as &T)
            }
            x if x == TypeId::of::<CInput>() => {
                Some(&self.pool.2[id].as_any().downcast_ref().unwrap() as &T)
            }
            _ => None,
        }
    }

    pub fn get_component_mut<T: Component>(&mut self, id: usize) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        match type_id {
            x if x == TypeId::of::<CTransform>() => {
                Some(self.pool.0[id].as_any_mut().downcast_mut().unwrap() as &mut T)
            }
            x if x == TypeId::of::<CLifetime>() => {
                Some(self.pool.1[id].as_any_mut().downcast_mut().unwrap() as &mut T)
            }
            x if x == TypeId::of::<CInput>() => {
                Some(self.pool.2[id].as_any_mut().downcast_mut().unwrap() as &mut T)
            }
            _ => None,
        }
    }

    pub fn get_tag(&self, id: usize) -> &EntityTag {
        &self.tags[id]
    }

    pub fn get_entity_count(&self) -> usize {
        self.entity_count
    }

    pub fn add_entity(&mut self, tag: EntityTag) -> Entity {
        let id = self.get_next_available_id().unwrap();
        self.tags[id] = tag;
        self.active[id] = true;

        return Entity::new(id);
    }

    pub fn is_active(&self, id: usize) -> bool {
        self.active[id]
    }

    pub fn add_component<T: Component>(&mut self, id: usize) {
        let type_id = TypeId::of::<T>();

        match type_id {
            x if x == TypeId::of::<CTransform>() => {
                self.pool.0[id].reset();
            }
            x if x == TypeId::of::<CLifetime>() => {
                self.pool.1[id].reset();
            }
            x if x == TypeId::of::<CInput>() => {
                self.pool.2[id].reset();
            }
            _ => {}
        }
    }

    pub fn destroy_entity(&mut self, id: usize) {
        self.active[id] = false;
    }

    pub fn get_pool(&self) -> &EntityComponentVectorTuple {
        &self.pool
    }

    pub fn get_pool_mut(&mut self) -> &mut EntityComponentVectorTuple {
        &mut self.pool
    }

    fn get_next_available_id(&self) -> Option<usize> {
        for i in 0..self.entity_count {
            if !self.active[i] {
                return Some(i);
            }
        }
        None
    }
}
