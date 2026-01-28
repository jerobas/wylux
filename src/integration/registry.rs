use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Function '{0}' not found")]
    NotFound(String),
    #[error("Function execution error: {0}")]
    Execution(String),
}

pub type FunctionHandler = Arc<dyn Fn(Value) -> Result<Value, RegistryError> + Send + Sync>;

/// Function registry for RPC calls.
pub struct FunctionRegistry {
    functions: Arc<RwLock<HashMap<String, FunctionHandler>>>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a function handler.
    pub async fn register<F>(&self, name: String, handler: F)
    where
        F: Fn(Value) -> Result<Value, RegistryError> + Send + Sync + 'static,
    {
        let mut functions = self.functions.write().await;
        functions.insert(name, Arc::new(handler));
    }
    
    /// Call a registered function.
    pub async fn call(&self, name: &str, args: Value) -> Result<Value, RegistryError> {
        let functions = self.functions.read().await;
        let handler = functions
            .get(name)
            .ok_or_else(|| RegistryError::NotFound(name.to_string()))?;
        
        handler(args)
    }
    
    /// Check if a function is registered.
    pub async fn has_function(&self, name: &str) -> bool {
        let functions = self.functions.read().await;
        functions.contains_key(name)
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
