pub mod entity;
pub mod entity_manager;
pub mod entity_memory_pool;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub enum EntityTag {
    None,
    Player,
    Bullet,
    Enemy,
    Platform,
    Goal,
}
