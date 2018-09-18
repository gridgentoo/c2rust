use indexmap::IndexSet;
use syntax::ast::{Item, ForeignItem};
use syntax::ptr::P;
use std::mem::swap;

// REVIEW: We might be able to use ItemStore in the Translation struct
// as a replacement for the uses, items, and foreign items fields

#[derive(Debug)]
pub struct ItemStore {
    pub items: Vec<P<Item>>,
    pub foreign_items: Vec<ForeignItem>,
    pub uses: IndexSet<P<Item>>,
}

impl ItemStore {
    pub fn new() -> Self {
        ItemStore {
            items: Vec::new(),
            foreign_items: Vec::new(),
            uses: IndexSet::new(),
        }
    }

    pub fn drain(&mut self) -> (Vec<P<Item>>, Vec<ForeignItem>, IndexSet<P<Item>>) {
        let mut items = Vec::new();
        let mut foreign_items = Vec::new();
        let mut uses = IndexSet::new();

        swap(&mut items, &mut self.items);
        swap(&mut foreign_items, &mut self.foreign_items);
        swap(&mut uses, &mut self.uses);

        (items, foreign_items, uses)
    }
}
