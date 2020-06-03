use std::any::TypeId;
use std::collections::HashMap;

pub struct TypeMap<V>(HashMap<TypeId, V>);

impl<V> TypeMap<V> {
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[inline]
    pub fn get<K: 'static>(&self) -> Option<&V> {
        self.0.get(&TypeId::of::<K>())
    }

    #[inline]
    pub fn get_mut<K: 'static>(&mut self) -> Option<&mut V> {
        self.0.get_mut(&TypeId::of::<K>())
    }

    #[inline]
    pub fn get_raw(&self, ty: TypeId) -> Option<&V> {
        self.0.get(&ty)
    }

    #[inline]
    pub fn get_raw_mut(&mut self, ty: TypeId) -> Option<&mut V> {
        self.0.get_mut(&ty)
    }

    #[inline]
    pub fn insert<K: 'static>(&mut self, value: V) -> Option<V> {
        self.0.insert(TypeId::of::<K>(), value)
    }

    #[inline]
    pub fn remove<K: 'static>(&mut self) -> Option<V> {
        self.0.remove(&TypeId::of::<K>())
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::type_map::TypeMap;

    #[test]
    pub fn test_type_map_get_insert() {
        let mut map = TypeMap::<u32>::new();

        assert_eq!(map.len(), 0);
        assert_eq!(map.get::<u8>(), None);

        assert_eq!(map.insert::<u8>(32), None);
        assert_eq!(map.insert::<u8>(42), Some(32));
        assert_eq!(map.len(), 1);

        assert_eq!(map.get::<u8>(), Some(&42));
    }

    #[test]
    pub fn test_type_map_remove_clear() {
        let mut map = TypeMap::<u32>::new();
        map.insert::<u8>(42);
        map.insert::<u16>(52);

        assert_eq!(map.len(), 2);

        assert_eq!(map.remove::<u8>(), Some(42));
        assert_eq!(map.len(), 1);

        assert_eq!(map.remove::<u128>(), None);
        assert_eq!(map.len(), 1);

        map.clear();
        assert_eq!(map.len(), 0);
    }
}
