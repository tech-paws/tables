pub use crate::{Table, TableEntry};

pub struct LinearTable<T, ID> {
    ids: Vec<ID>,
    visible: Vec<TableEntry<T, ID>>,
    invisible: Vec<TableEntry<T, ID>>,
}

impl<T, ID> Default for LinearTable<T, ID> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, ID> LinearTable<T, ID> {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            visible: Vec::new(),
            invisible: Vec::new(),
        }
    }
}

impl<T, ID: std::cmp::PartialEq + std::fmt::Debug> LinearTable<T, ID> {
    #[cfg(debug_assertions)]
    fn validate_uniqueness(&self, new_id: ID) {
        let mut found = false;

        for id in self.ids.iter() {
            if *id == new_id {
                if found {
                    panic!("Id already exists: {:?}", new_id);
                } else {
                    found = true;
                }
            }
        }
    }
}

pub struct LinearTableAllIter<'a, T, ID> {
    iter: std::slice::Iter<'a, TableEntry<T, ID>>,
}

impl<'a, T, ID> LinearTableAllIter<'a, T, ID> {
    fn _new(iter: std::slice::Iter<'a, TableEntry<T, ID>>) -> Self {
        Self { iter }
    }
}

impl<'a, T, ID> Iterator for LinearTableAllIter<'a, T, ID> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| &e.data)
    }
}

// TODO(sysint64): write tests
impl<T, ID: std::cmp::PartialEq + std::fmt::Debug + Copy> Table<T, ID> for LinearTable<T, ID> {
    fn push(&mut self, item: T, id: ID) {
        self.ids.push(id);
        #[cfg(debug_assertions)]
        self.validate_uniqueness(id);
        self.visible.push(TableEntry::new(id, item));
    }

    fn delete_by_id(&mut self, target_id: ID) {
        self.ids.retain(|id| *id != target_id);
        self.visible.retain(|element| element.id != target_id);
        self.invisible.retain(|element| element.id != target_id);
    }

    fn get_by_id(&self, id: ID) -> Option<&T> {
        self.visible
            .iter()
            .filter(|e| e.id == id)
            .last()
            .map(|e| &e.data)
    }

    fn set_by_id(&mut self, id: ID, item: T) {
        #[cfg(debug_assertions)]
        self.validate_uniqueness(id);

        for data in self.visible.iter_mut() {
            if data.id == id {
                *data = TableEntry::new(id, item);
                break;
            }
        }
    }

    fn all(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.visible.iter().map(|e| &e.data))
    }

    fn all_mut(&mut self) -> Box<dyn Iterator<Item = &mut T> + '_> {
        Box::new(self.visible.iter_mut().map(|e| &mut e.data))
    }
}
