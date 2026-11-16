// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-volume-manager: volume manager component model.
//!
//! Real implementation: volumes, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Volume {
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
pub enum VolumeManagerError {
    EmptyVolumeName,
    DuplicateVolume(String),
    EmptyEventSubject,
}

impl fmt::Display for VolumeManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVolumeName => write!(f, "volume name must not be empty"),
            Self::DuplicateVolume(id) => write!(f, "duplicate volume {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for VolumeManagerError {}

pub struct VolumeManager {
    pub name: String,
    volumes: BTreeMap<String, Volume>,
    events: BTreeMap<String, Event>,
}

impl VolumeManager {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            volumes: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_volume(&mut self, item: Volume) -> Result<(), VolumeManagerError> {
        if item.name.trim().is_empty() {
            return Err(VolumeManagerError::EmptyVolumeName);
        }
        let id = item.id.clone();
        if self.volumes.contains_key(&id) {
            return Err(VolumeManagerError::DuplicateVolume(id.clone()));
        }
        self.volumes.insert(id, item);
        Ok(())
    }

    pub fn volume_count(&self) -> usize {
        self.volumes.len()
    }

    pub fn get_volume(&self, id: &str) -> Option<&Volume> {
        self.volumes.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), VolumeManagerError> {
        if event.subject.trim().is_empty() {
            return Err(VolumeManagerError::EmptyEventSubject);
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

    fn sample_volume(id: &str, name: &str) -> Volume {
        Volume {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = VolumeManager::new("VolumeManager");
        assert_eq!(m.name, "VolumeManager");
        assert_eq!(m.volume_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_volume() {
        let mut m = VolumeManager::new("VolumeManager");
        m.add_volume(sample_volume("a1", "Alpha")).unwrap();
        assert_eq!(m.volume_count(), 1);
        assert!(m.get_volume("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = VolumeManager::new("VolumeManager");
        assert!(m.add_volume(sample_volume("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = VolumeManager::new("VolumeManager");
        m.add_volume(sample_volume("x", "One")).unwrap();
        assert!(m.add_volume(sample_volume("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = VolumeManager::new("VolumeManager");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
