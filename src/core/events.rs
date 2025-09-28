//! Event system for game-wide notifications and state changes

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Event bus for loosely coupled communication between systems
pub struct EventBus {
    /// Event listeners organized by event type
    listeners: HashMap<EventType, Vec<Box<dyn EventListener>>>,
}

/// Types of events that can occur in the game
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    /// Player moved to a new location
    PlayerMoved,
    /// Magic was cast
    MagicCast,
    /// Faction reputation changed
    FactionReputationChanged,
    /// Player learned a new theory
    TheoryLearned,
    /// Crystal degraded significantly
    CrystalDegraded,
    /// Player leveled up an attribute
    AttributeIncreased,
    /// Time advanced significantly
    TimeAdvanced,
    /// World event triggered
    WorldEvent,
}

/// A game event with associated data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Type of event
    pub event_type: EventType,
    /// Event data as JSON string
    pub data: String,
    /// When the event occurred (game time in minutes)
    pub timestamp: i32,
}

/// Trait for objects that can listen to events
pub trait EventListener: Send + Sync {
    fn handle_event(&mut self, event: &Event);
    fn get_name(&self) -> &str;
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    /// Register a listener for specific event types
    pub fn register_listener(&mut self, event_type: EventType, listener: Box<dyn EventListener>) {
        self.listeners.entry(event_type).or_insert_with(Vec::new).push(listener);
    }

    /// Publish an event to all registered listeners
    pub fn publish(&mut self, event: Event) {
        if let Some(listeners) = self.listeners.get_mut(&event.event_type) {
            for listener in listeners {
                listener.handle_event(&event);
            }
        }
    }

    /// Create and publish an event
    pub fn emit(&mut self, event_type: EventType, data: String, timestamp: i32) {
        let event = Event {
            event_type: event_type.clone(),
            data,
            timestamp,
        };
        self.publish(event);
    }

    /// Get count of listeners for an event type
    pub fn listener_count(&self, event_type: &EventType) -> usize {
        self.listeners.get(event_type).map_or(0, |v| v.len())
    }
}

impl Event {
    /// Create a new event
    pub fn new(event_type: EventType, data: String, timestamp: i32) -> Self {
        Self {
            event_type,
            data,
            timestamp,
        }
    }

    /// Deserialize event data as specific type
    pub fn get_data<T>(&self) -> Result<T, serde_json::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        serde_json::from_str(&self.data)
    }
}

// Example event listeners

/// Logs all events for debugging
pub struct EventLogger {
    name: String,
    log_entries: Vec<String>,
}

impl EventLogger {
    pub fn new() -> Self {
        Self {
            name: "EventLogger".to_string(),
            log_entries: Vec::new(),
        }
    }

    pub fn get_log(&self) -> &Vec<String> {
        &self.log_entries
    }
}

impl EventListener for EventLogger {
    fn handle_event(&mut self, event: &Event) {
        let log_entry = format!(
            "[{}] {:?}: {}",
            event.timestamp,
            event.event_type,
            event.data
        );
        self.log_entries.push(log_entry);

        // Keep only last 100 entries
        if self.log_entries.len() > 100 {
            self.log_entries.remove(0);
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_bus_creation() {
        let bus = EventBus::new();
        assert_eq!(bus.listener_count(&EventType::PlayerMoved), 0);
    }

    #[test]
    fn test_event_registration_and_publishing() {
        let mut bus = EventBus::new();
        let logger = Box::new(EventLogger::new());

        bus.register_listener(EventType::PlayerMoved, logger);
        assert_eq!(bus.listener_count(&EventType::PlayerMoved), 1);

        let event = Event::new(
            EventType::PlayerMoved,
            "test data".to_string(),
            0,
        );

        bus.publish(event);
    }

    #[test]
    fn test_event_data_serialization() {
        let data = serde_json::json!({"location": "test_room"});
        let event = Event::new(
            EventType::PlayerMoved,
            data.to_string(),
            100,
        );

        let parsed: serde_json::Value = event.get_data().unwrap();
        assert_eq!(parsed["location"], "test_room");
    }
}