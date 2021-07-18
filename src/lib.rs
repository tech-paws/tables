pub mod linear_table;
pub mod sparse_table;

pub trait Table<T, ID> {
    fn push(&mut self, item: T, id: ID);

    fn delete_by_id(&mut self, id: ID);

    fn try_get_by_id(&self, id: ID) -> Option<&T>;

    fn get_by_id(&self, id: ID) -> &T {
        self.try_get_by_id(id).unwrap()
    }

    fn set_by_id(&mut self, id: ID, item: T);

    fn all(&self) -> Box<dyn Iterator<Item = &T> + '_>;

    fn all_enumerated(&self) -> Box<dyn Iterator<Item = (ID, &T)> + '_>;

    fn all_mut(&mut self) -> Box<dyn Iterator<Item = &mut T> + '_>;

    fn all_mut_enumerated(&mut self) -> Box<dyn Iterator<Item = (ID, &mut T)> + '_>;

    fn len(&self) -> usize;
}

pub trait U64Id {
    fn get_id(&self) -> u64;
}

pub struct TableEntry<T, ID> {
    pub id: ID,
    pub data: T,
}

impl<T, ID> TableEntry<T, ID> {
    pub fn new(id: ID, data: T) -> Self {
        Self { id, data }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
