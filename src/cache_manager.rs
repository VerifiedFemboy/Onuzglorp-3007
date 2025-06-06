use std::any::Any;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::{LogLevel, log_message};

pub struct CacheEntry {
    pub value: Box<dyn Any + Send + Sync>,
    pub expire: Option<Instant>, // For time live
}

pub struct CacheManager {
    pub cache: HashMap<String, CacheEntry>,
}

impl CacheManager {
    pub fn new() -> Self {
        CacheManager {
            cache: HashMap::new(),
        }
    }

    pub fn add<T: 'static + Send + Sync>(&mut self, key: String, value: T, time_live: Option<LiveTime>) {
        let expires = match time_live {
            Some(LiveTime::Hours(hours)) => Some(Instant::now() + Duration::from_secs(hours * 3600)),
            Some(LiveTime::Minutes(minutes)) => Some(Instant::now() + Duration::from_secs(minutes * 60)),
            None => None, // No expiration
        };
        log_message(format!("{} added to cache", &key).as_str(), LogLevel::Cache);
        self.cache.insert(
            key,
            CacheEntry {
                value: Box::new(value),
                expire: expires,
            },
        );
    }

    pub fn get<T: 'static>(&mut self, key: &str) -> Option<&T> {
        let expired = if let Some(cache) = self.cache.get(key) {
            if let Some(expire) = cache.expire {
                expire < Instant::now()
            } else {
                false // No expiration set
            }
        } else {
            return None; // Entry not found
        };

        if expired {
            log_message(format!("{} has expired. Deleting the cache", key).as_str(), LogLevel::Cache);
            self.cache.remove(key);
            return None; // Entry has expired
        }

        self.cache.get(key)?.value.downcast_ref::<T>()
    }

    pub fn get_owned<T: 'static + Copy>(&mut self, key: &str) -> Option<T> {
        let expired = if let Some(cache) = self.cache.get(key) {
            if let Some(expire) = cache.expire {
                expire < Instant::now()
            } else {
                false
            }
        } else {
            return None;
        };

        if expired {
            log_message(format!("{} has expired. Deleting the cache", key).as_str(), LogLevel::Cache);
            self.cache.remove(key);
            return None;
        }

        let val = self.cache.get(key)?.value.downcast_ref::<T>()?;

        Some(*val)
    }

    pub fn cleanup_expired(&mut self) {
        let now = Instant::now();
        self.cache
            .retain(|_, entry| entry.expire.map_or(true, |exp| now < exp));
    }

    pub fn get_all_entries<T: 'static>(&self) -> HashMap<String, &T> {
        self.cache
            .iter()
            .filter_map(|(key, entry)| {
                if let Some(value) = entry.value.downcast_ref::<T>() {
                    Some((key.clone(), value))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub enum LiveTime {
    Hours(u64),
    Minutes(u64),
}