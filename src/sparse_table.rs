pub use crate::{Table, U64Id};

pub struct SparseTableEntry<T, ID> {
    pub id: ID,
    pub data: T,
    pub busy: bool,
}

impl<T, ID> SparseTableEntry<T, ID> {
    pub fn new(id: ID, data: T, budy: bool) -> Self {
        Self {
            id,
            data,
            busy: budy,
        }
    }
}

pub struct SparseTableAllIter<'a, T, ID> {
    iter: std::slice::Iter<'a, SparseTableEntry<T, ID>>,
}

impl<'a, T, ID> SparseTableAllIter<'a, T, ID> {
    pub fn new(iter: std::slice::Iter<'a, SparseTableEntry<T, ID>>) -> Self {
        Self { iter }
    }
}

impl<'a, T, ID> Iterator for SparseTableAllIter<'a, T, ID> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().filter(|e| e.busy).map(|e| &e.data)
    }
}

pub struct SparseTable<T, ID> {
    ids: Vec<ID>,
    data: Vec<SparseTableEntry<T, ID>>,
}

impl<T, ID> Default for SparseTable<T, ID> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, ID> SparseTable<T, ID> {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl<T, ID: std::cmp::PartialEq + std::fmt::Debug> SparseTable<T, ID> {
    #[cfg(debug_assertions)]
    fn validate_uniqueness(&self, new_id: ID) {
        for id in self.ids.iter() {
            if *id == new_id {
                panic!("Id already exists: {:?}", new_id);
            }
        }
    }
}

impl<T, ID: std::cmp::PartialEq + std::fmt::Debug + U64Id + Copy> Table<T, ID>
    for SparseTable<T, ID>
{
    fn push(&mut self, item: T, id: ID) {
        #[cfg(debug_assertions)]
        self.validate_uniqueness(id);
        self.ids.push(id);
        self.data.push(SparseTableEntry::new(id, item, true));
    }

    fn delete_by_id(&mut self, target_id: ID) {
        if let Some(data) = self.data.get_mut(target_id.get_id() as usize) {
            (*data).busy = false;
        }
    }

    fn try_get_by_id(&self, id: ID) -> Option<&T> {
        self.data
            .iter()
            .filter(|e| e.id == id)
            .last()
            .map(|e| &e.data)
    }

    fn set_by_id(&mut self, id: ID, item: T) {
        self.data[id.get_id() as usize] = SparseTableEntry::new(id, item, true);
    }

    fn all(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.data.iter().filter(|e| e.busy).map(|e| &e.data))
    }

    fn all_mut(&mut self) -> Box<dyn Iterator<Item = &mut T> + '_> {
        Box::new(self.data.iter_mut().filter(|e| e.busy).map(|e| &mut e.data))
    }

    fn all_enumerated(&self) -> Box<dyn Iterator<Item = (ID, &T)> + '_> {
        Box::new(self.data.iter().filter(|e| e.busy).map(|e| (e.id, &e.data)))
    }

    fn all_mut_enumerated(&mut self) -> Box<dyn Iterator<Item = (ID, &mut T)> + '_> {
        Box::new(
            self.data
                .iter_mut()
                .filter(|e| e.busy)
                .map(|e| (e.id, &mut e.data)),
        )
    }
}
