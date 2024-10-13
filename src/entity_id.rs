use std::sync::atomic::{self, AtomicI32};

/// The current Entity ID. Incremented by one every time there is a new entity.
///
/// This value should rarely be accessed directly, and definitely never updated.
pub static ENTITY_ID: EntityID = EntityID::new(0);

pub struct EntityID {
    id: AtomicI32
}

impl EntityID {
    /// Create a new entity ID
    const fn new(id: i32) -> Self {
        Self {
            id: AtomicI32::new(id)
        }
    }

    /// Get a new Entity ID and increment the global value by 1.
    #[inline]
    pub fn get(&self) -> i32 {
        let eid = self.id.load(atomic::Ordering::Relaxed);
        self.id.store(eid + 1, atomic::Ordering::Relaxed);

        eid
    }
}
