use std::collections::{hash_map, HashMap};

use crate::utils::geometry::XY;

#[derive(Debug)]
pub struct SparseGrid<T> {
    items: HashMap<XY, T>,
    filler: Option<T>,
    lower_corner: XY,
    upper_corner: XY,
}

impl<T> SparseGrid<T> {
    pub fn get_lower_corner(&self) -> &XY {
        &self.lower_corner
    }

    pub fn get_upper_corner(&self) -> &XY {
        &self.upper_corner
    }

    pub fn get(&self, at: &XY) -> Option<&T> {
        self.items.get(at).or(self.filler.as_ref())
    }

    pub fn insert(&mut self, at: XY, value: T) -> Option<T> {
        self.lower_corner.update_min(&at);
        self.upper_corner.update_max(&at);
        self.items.insert(at, value)
    }

    pub fn new(filler: Option<T>) -> Self {
        SparseGrid {
            items: HashMap::new(),
            filler,
            lower_corner: XY(0, 0),
            upper_corner: XY(0, 0),
        }
    }

    pub fn iter(&self) -> hash_map::Iter<XY, T> {
        self.items.iter()
    }
}

impl<T> IntoIterator for SparseGrid<T> {
    type Item = (XY, T);
    type IntoIter = hash_map::IntoIter<XY, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
