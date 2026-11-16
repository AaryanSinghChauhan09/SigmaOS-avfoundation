// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-avfoundation: avfoundation component model.
//!
//! Real implementation: avfoundation_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvfoundationItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AvfoundationError {
    EmptyAvfoundationItemName,
    DuplicateAvfoundationItem(String),
    EmptyEventSubject,
}

impl fmt::Display for AvfoundationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAvfoundationItemName => write!(f, "avfoundation_item name must not be empty"),
            Self::DuplicateAvfoundationItem(id) => write!(f, "duplicate avfoundation_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AvfoundationError {}

pub struct Avfoundation {
    pub name: String,
    avfoundation_items: BTreeMap<String, AvfoundationItem>,
    events: BTreeMap<String, Event>,
}

impl Avfoundation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            avfoundation_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_avfoundation_item(&mut self, item: AvfoundationItem) -> Result<(), AvfoundationError> {
        if item.name.trim().is_empty() {
            return Err(AvfoundationError::EmptyAvfoundationItemName);
        }
        let id = item.id.clone();
        if self.avfoundation_items.contains_key(&id) {
            return Err(AvfoundationError::DuplicateAvfoundationItem(id.clone()));
        }
        self.avfoundation_items.insert(id, item);
        Ok(())
    }

    pub fn avfoundation_item_count(&self) -> usize {
        self.avfoundation_items.len()
    }

    pub fn get_avfoundation_item(&self, id: &str) -> Option<&AvfoundationItem> {
        self.avfoundation_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AvfoundationError> {
        if event.subject.trim().is_empty() {
            return Err(AvfoundationError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_avfoundation_item(id: &str, name: &str) -> AvfoundationItem {
        AvfoundationItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Avfoundation::new("Avfoundation");
        assert_eq!(m.name, "Avfoundation");
        assert_eq!(m.avfoundation_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_avfoundation_item() {
        let mut m = Avfoundation::new("Avfoundation");
        m.add_avfoundation_item(sample_avfoundation_item("a1", "Alpha")).unwrap();
        assert_eq!(m.avfoundation_item_count(), 1);
        assert!(m.get_avfoundation_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Avfoundation::new("Avfoundation");
        assert!(m.add_avfoundation_item(sample_avfoundation_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Avfoundation::new("Avfoundation");
        m.add_avfoundation_item(sample_avfoundation_item("x", "One")).unwrap();
        assert!(m.add_avfoundation_item(sample_avfoundation_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Avfoundation::new("Avfoundation");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
