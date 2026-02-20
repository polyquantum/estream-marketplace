//! Poll scheduler implementation.
//!
//! This module provides the polling scheduler that manages periodic data collection:
//! - Priority-based scheduling
//! - Adaptive intervals based on success/failure
//! - Rate limiting
//!
//! Implements `circuits/industrial/poll_scheduler.escir.yaml`

use crate::config::RegisterConfig;
use crate::types::*;
use crate::{IndustrialError, Result};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, warn};

/// Poll scheduler configuration.
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Maximum polls per second (rate limit)
    pub max_polls_per_second: u32,
    /// Enable adaptive scheduling
    pub adaptive_enabled: bool,
    /// Backoff factor on errors
    pub backoff_factor: f32,
    /// Maximum backoff interval in milliseconds
    pub max_backoff_interval_ms: u32,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            max_polls_per_second: 100,
            adaptive_enabled: true,
            backoff_factor: 1.5,
            max_backoff_interval_ms: 60000,
        }
    }
}

/// A scheduled poll item.
#[derive(Debug, Clone)]
pub struct PollItem {
    /// Unique poll ID
    pub poll_id: u32,
    /// Device ID
    pub device_id: String,
    /// Register name
    pub name: String,
    /// Register type
    pub register_type: RegisterType,
    /// Starting address
    pub address: u16,
    /// Number of registers
    pub count: u16,
    /// Base poll interval in milliseconds
    pub base_interval_ms: u32,
    /// Current effective interval
    pub current_interval_ms: u32,
    /// Priority (higher = more important)
    pub priority: u8,
    /// Whether enabled
    pub enabled: bool,
}

impl From<&RegisterConfig> for PollItem {
    fn from(config: &RegisterConfig) -> Self {
        static POLL_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
        
        Self {
            poll_id: POLL_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            device_id: config.device_id.clone(),
            name: config.name.clone(),
            register_type: config.register_type,
            address: config.address,
            count: config.data_type.word_count(),
            base_interval_ms: config.poll_interval_ms,
            current_interval_ms: config.poll_interval_ms,
            priority: config.priority,
            enabled: true,
        }
    }
}

/// Poll status for a single item.
#[derive(Debug, Clone, Default)]
pub struct PollStatus {
    /// Current effective interval
    pub current_interval_ms: u32,
    /// Last poll timestamp
    pub last_poll_ns: u64,
    /// Next scheduled poll timestamp
    pub next_poll_ns: u64,
    /// Total polls executed
    pub polls_total: u64,
    /// Successful polls
    pub polls_success: u64,
    /// Failed polls
    pub polls_failed: u64,
    /// Average latency in microseconds
    pub avg_latency_us: u32,
    /// Consecutive failures
    pub consecutive_failures: u16,
}

/// A poll trigger event.
#[derive(Debug, Clone)]
pub struct PollTrigger {
    /// Poll ID
    pub poll_id: u32,
    /// Device ID
    pub device_id: String,
    /// Register type
    pub register_type: RegisterType,
    /// Address
    pub address: u16,
    /// Count
    pub count: u16,
    /// Sequence number
    pub sequence_number: u64,
    /// Scheduled time
    pub scheduled_time_ns: u64,
    /// Actual trigger time
    pub actual_time_ns: u64,
}

/// Poll completion feedback.
#[derive(Debug, Clone)]
pub struct PollComplete {
    /// Poll ID
    pub poll_id: u32,
    /// Sequence number
    pub sequence_number: u64,
    /// Success status
    pub success: bool,
    /// Latency in microseconds
    pub latency_us: u32,
}

/// Entry in the scheduling heap.
#[derive(Debug, Clone, Eq, PartialEq)]
struct ScheduleEntry {
    /// Next poll time (nanoseconds)
    next_time_ns: u64,
    /// Priority (for tie-breaking)
    priority: u8,
    /// Poll ID
    poll_id: u32,
}

impl Ord for ScheduleEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Earliest time first, then highest priority
        other.next_time_ns.cmp(&self.next_time_ns)
            .then_with(|| self.priority.cmp(&other.priority))
    }
}

impl PartialOrd for ScheduleEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Poll scheduler.
pub struct PollScheduler {
    /// Configuration
    config: SchedulerConfig,
    /// Poll items
    items: RwLock<HashMap<u32, PollItem>>,
    /// Poll status
    status: RwLock<HashMap<u32, PollStatus>>,
    /// Schedule heap (protected by mutex for pop)
    schedule: tokio::sync::Mutex<BinaryHeap<ScheduleEntry>>,
    /// Sequence counter
    sequence: std::sync::atomic::AtomicU64,
    /// Trigger channel
    trigger_tx: mpsc::Sender<PollTrigger>,
    /// Trigger receiver (for the scheduler loop)
    trigger_rx: tokio::sync::Mutex<Option<mpsc::Receiver<PollTrigger>>>,
    /// Running flag
    running: std::sync::atomic::AtomicBool,
}

impl PollScheduler {
    /// Creates a new poll scheduler.
    pub fn new(config: SchedulerConfig) -> (Self, mpsc::Receiver<PollTrigger>) {
        let (trigger_tx, trigger_rx) = mpsc::channel(256);
        
        let scheduler = Self {
            config,
            items: RwLock::new(HashMap::new()),
            status: RwLock::new(HashMap::new()),
            schedule: tokio::sync::Mutex::new(BinaryHeap::new()),
            sequence: std::sync::atomic::AtomicU64::new(1),
            trigger_tx,
            trigger_rx: tokio::sync::Mutex::new(Some(trigger_rx)),
            running: std::sync::atomic::AtomicBool::new(false),
        };
        
        // Return a dummy receiver since we took ownership
        let (_, rx) = mpsc::channel(1);
        (scheduler, rx)
    }
    
    /// Creates a scheduler with external trigger channel.
    pub fn with_trigger_channel(
        config: SchedulerConfig,
        trigger_tx: mpsc::Sender<PollTrigger>,
    ) -> Self {
        Self {
            config,
            items: RwLock::new(HashMap::new()),
            status: RwLock::new(HashMap::new()),
            schedule: tokio::sync::Mutex::new(BinaryHeap::new()),
            sequence: std::sync::atomic::AtomicU64::new(1),
            trigger_tx,
            trigger_rx: tokio::sync::Mutex::new(None),
            running: std::sync::atomic::AtomicBool::new(false),
        }
    }
    
    /// Adds a poll item.
    pub async fn add_poll(&self, item: PollItem) {
        let poll_id = item.poll_id;
        let interval_ms = item.current_interval_ms;
        let priority = item.priority;
        
        // Add to items
        self.items.write().await.insert(poll_id, item);
        
        // Initialize status
        let now = timestamp_ns();
        self.status.write().await.insert(poll_id, PollStatus {
            current_interval_ms: interval_ms,
            next_poll_ns: now,
            ..Default::default()
        });
        
        // Add to schedule
        self.schedule.lock().await.push(ScheduleEntry {
            next_time_ns: now,
            priority,
            poll_id,
        });
    }
    
    /// Removes a poll item.
    pub async fn remove_poll(&self, poll_id: u32) {
        self.items.write().await.remove(&poll_id);
        self.status.write().await.remove(&poll_id);
        // Note: Entry remains in heap but will be ignored
    }
    
    /// Enables/disables a poll item.
    pub async fn set_enabled(&self, poll_id: u32, enabled: bool) {
        if let Some(item) = self.items.write().await.get_mut(&poll_id) {
            item.enabled = enabled;
        }
    }
    
    /// Reports poll completion.
    pub async fn poll_complete(&self, complete: PollComplete) {
        let mut status = self.status.write().await;
        let mut items = self.items.write().await;
        
        if let Some(s) = status.get_mut(&complete.poll_id) {
            s.polls_total += 1;
            
            if complete.success {
                s.polls_success += 1;
                s.consecutive_failures = 0;
                
                // Reset interval on success
                if self.config.adaptive_enabled {
                    if let Some(item) = items.get_mut(&complete.poll_id) {
                        item.current_interval_ms = item.base_interval_ms;
                        s.current_interval_ms = item.base_interval_ms;
                    }
                }
            } else {
                s.polls_failed += 1;
                s.consecutive_failures += 1;
                
                // Backoff on failure
                if self.config.adaptive_enabled {
                    if let Some(item) = items.get_mut(&complete.poll_id) {
                        let new_interval = (item.current_interval_ms as f32 
                            * self.config.backoff_factor) as u32;
                        item.current_interval_ms = new_interval
                            .min(self.config.max_backoff_interval_ms);
                        s.current_interval_ms = item.current_interval_ms;
                    }
                }
            }
            
            // Update latency (exponential moving average)
            s.avg_latency_us = (s.avg_latency_us * 7 + complete.latency_us) / 8;
        }
    }
    
    /// Gets status for a poll item.
    pub async fn get_status(&self, poll_id: u32) -> Option<PollStatus> {
        self.status.read().await.get(&poll_id).cloned()
    }
    
    /// Runs the scheduler loop.
    pub async fn run(&self) {
        use std::sync::atomic::Ordering;
        
        self.running.store(true, Ordering::SeqCst);
        
        // Rate limiting
        let min_interval_ns = 1_000_000_000u64 / self.config.max_polls_per_second as u64;
        let mut last_poll_ns = 0u64;
        
        while self.running.load(Ordering::SeqCst) {
            let now = timestamp_ns();
            
            // Get next scheduled poll
            let entry = {
                let mut schedule = self.schedule.lock().await;
                if let Some(entry) = schedule.peek() {
                    if entry.next_time_ns <= now {
                        schedule.pop()
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            
            if let Some(entry) = entry {
                // Check if item still exists and is enabled
                let item = self.items.read().await.get(&entry.poll_id).cloned();
                
                if let Some(item) = item {
                    if item.enabled {
                        // Rate limiting
                        if now - last_poll_ns < min_interval_ns {
                            tokio::time::sleep(std::time::Duration::from_nanos(
                                min_interval_ns - (now - last_poll_ns)
                            )).await;
                        }
                        
                        let actual_time = timestamp_ns();
                        last_poll_ns = actual_time;
                        
                        // Send trigger
                        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
                        let trigger = PollTrigger {
                            poll_id: item.poll_id,
                            device_id: item.device_id.clone(),
                            register_type: item.register_type,
                            address: item.address,
                            count: item.count,
                            sequence_number: sequence,
                            scheduled_time_ns: entry.next_time_ns,
                            actual_time_ns: actual_time,
                        };
                        
                        if self.trigger_tx.send(trigger).await.is_err() {
                            warn!("Trigger channel closed");
                            break;
                        }
                        
                        // Update status
                        if let Some(s) = self.status.write().await.get_mut(&entry.poll_id) {
                            s.last_poll_ns = actual_time;
                            s.next_poll_ns = actual_time + (item.current_interval_ms as u64 * 1_000_000);
                        }
                        
                        // Reschedule
                        let next_time = actual_time + (item.current_interval_ms as u64 * 1_000_000);
                        self.schedule.lock().await.push(ScheduleEntry {
                            next_time_ns: next_time,
                            priority: item.priority,
                            poll_id: item.poll_id,
                        });
                    }
                }
            } else {
                // No polls due, sleep a bit
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        }
    }
    
    /// Stops the scheduler.
    pub fn stop(&self) {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_schedule_entry_ordering() {
        let e1 = ScheduleEntry { next_time_ns: 100, priority: 1, poll_id: 1 };
        let e2 = ScheduleEntry { next_time_ns: 200, priority: 1, poll_id: 2 };
        let e3 = ScheduleEntry { next_time_ns: 100, priority: 2, poll_id: 3 };
        
        // Earlier time should come first
        assert!(e1 > e2);
        // Same time, higher priority first
        assert!(e3 > e1);
    }
}
