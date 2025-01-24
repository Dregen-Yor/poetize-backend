use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

#[derive(Clone)]
struct Entity {
    value: Arc<Mutex<Option<Arc<dyn std::any::Any + Send + Sync>>>>,
    task: Option<Arc<Mutex<Option<JoinHandle<()>>>>>,
}

impl Entity {
    fn new(value: Arc<dyn std::any::Any + Send + Sync>, task: Option<JoinHandle<()>>) -> Self {
        Entity {
            value: Arc::new(Mutex::new(Some(value))),
            task: task.map(|t| Arc::new(Mutex::new(Some(t)))),
        }
    }

    fn get_value(&self) -> Option<Arc<dyn std::any::Any + Send + Sync>> {
        self.value.lock().unwrap().clone()
    }

    fn cancel_task(&self) {
        if let Some(task) = &self.task {
            if let Some(task) = task.lock().unwrap().take() {
                task.abort();
            }
        }
    }
}

struct PoetryCache {
    map: Arc<RwLock<HashMap<String, Entity>>>,
}

impl PoetryCache {
    fn new() -> Self {
        PoetryCache {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn put(&self, key: String, data: Arc<dyn std::any::Any + Send + Sync>, expire: u64) {
        let mut map = self.map.write().await;
        if let Some(entity) = map.get(&key) {
            entity.cancel_task();
        }

        if expire > 0 {
            let key_clone = key.clone();
            let map_clone = self.map.clone();
            let task = tokio::spawn(async move {
                sleep(Duration::from_secs(expire)).await;
                map_clone.write().await.remove(&key_clone);
            });
            map.insert(key, Entity::new(data, Some(task)));
        } else {
            map.insert(key, Entity::new(data, None));
        }
    }

    async fn get(&self, key: &str) -> Option<Arc<dyn std::any::Any + Send + Sync>> {
        let map = self.map.read().await;
        map.get(key).and_then(|entity| entity.get_value())
    }

    async fn remove(&self, key: &str) -> Option<Arc<dyn std::any::Any + Send + Sync>> {
        let mut map = self.map.write().await;
        if let Some(entity) = map.remove(key) {
            entity.cancel_task();
            entity.get_value()
        } else {
            None
        }
    }

    async fn size(&self) -> usize {
        let map = self.map.read().await;
        map.len()
    }
}