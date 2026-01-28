use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type EventCallback = Arc<dyn Fn(Value) + Send + Sync>;

/// Event bus for pub/sub messaging.
pub struct EventBus {
    listeners: Arc<RwLock<HashMap<String, Vec<EventCallback>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register an event listener.
    pub async fn on<F>(&self, event_name: String, callback: F)
    where
        F: Fn(Value) + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.write().await;
        listeners
            .entry(event_name)
            .or_insert_with(Vec::new)
            .push(Arc::new(callback));
    }
    
    /// Emit an event to all listeners.
    pub async fn emit(&self, event_name: &str, data: Value) {
        let listeners = self.listeners.read().await;
        if let Some(callbacks) = listeners.get(event_name) {
            for callback in callbacks {
                callback(data.clone());
            }
        }
    }
    
    /// Remove all listeners for an event.
    pub async fn remove_listeners(&self, event_name: &str) {
        let mut listeners = self.listeners.write().await;
        listeners.remove(event_name);
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
