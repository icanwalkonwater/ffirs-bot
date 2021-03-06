use crate::mappers::FragmentMapper;
use crate::type_map::TypeMap;
use std::any::{Any, TypeId};

pub struct CmdManager {
    mappers: TypeMap<Box<dyn FragmentMapper>>,
}

impl CmdManager {
    pub fn new() -> Self {
        Self {
            mappers: TypeMap::new(),
        }
    }

    pub fn register_mapper<K: 'static>(mut self, mapper: Box<dyn FragmentMapper>) -> Self {
        self.mappers.insert::<K>(mapper);
        self
    }

    /// # Panics
    /// Panic if there is no mapper associated with this TypeId.
    pub fn map(&self, ty: TypeId, fragment: &str) -> Box<dyn Any> {
        self.mappers.get_raw(ty).unwrap().map(fragment)
    }

    /// # Panics
    /// Panic if there is no mapper associated with this TypeId.
    /// Also panic if the value returned by the mapper cannot be downcasted to `T`.
    pub fn map_downcast<T: 'static>(&self, fragment: &str) -> Box<T> {
        self.map(TypeId::of::<T>(), fragment)
            .downcast::<T>()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd_manager::CmdManager;
    use crate::mappers::{FromStrMapper, UserMapper};
    use serenity::model::id::UserId;

    fn create_manager() -> CmdManager {
        CmdManager::new()
            .register_mapper::<i32>(Box::new(FromStrMapper::<i32>::default()))
            .register_mapper::<UserId>(Box::new(UserMapper))
    }

    #[test]
    pub fn test_manager_register() {
        let manager = create_manager();

        assert_eq!(manager.mappers.len(), 2);
    }

    #[test]
    pub fn test_manager_map_downcast() {
        let manager = create_manager();

        assert_eq!(*manager.map_downcast::<i32>("42"), 42_i32);
        assert_eq!(*manager.map_downcast::<i32>("-42"), -42_i32);
    }

    #[test]
    pub fn test_manager_map_downcast_2() {
        let manager = create_manager();

        assert_eq!(*manager.map_downcast::<UserId>("<@1234>"), UserId(1234));
        assert_eq!(*manager.map_downcast::<UserId>("<@!1234>"), UserId(1234));
    }
}
