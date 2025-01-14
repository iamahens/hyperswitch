use super::{Event, EventHandler};
use crate::services::logger;

#[derive(Clone, Debug, Default)]
pub struct EventLogger {}

impl EventHandler for EventLogger {
    fn log_event<T: Event>(&self, event: T, previous: Option<T>) {
        if let Some(prev) = previous {
            logger::info!(previous = ?serde_json::to_string(&prev).unwrap_or(r#"{ "error": "Serialization failed" }"#.to_string()), current = ?serde_json::to_string(&event).unwrap_or(r#"{ "error": "Serialization failed" }"#.to_string()), event_type =? T::event_type(), event_id =? event.key(), log_type = "event");
        } else {
            logger::info!(current = ?serde_json::to_string(&event).unwrap_or(r#"{ "error": "Serialization failed" }"#.to_string()), event_type =? T::event_type(), event_id =? event.key(), log_type = "event");
        }
    }
}
