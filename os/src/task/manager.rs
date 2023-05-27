//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::{VecDeque,BinaryHeap};
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe

pub const BIG_STRIDE:usize = 0xffffffff;

///TaskManager trait
pub trait TaskManager {
    ///Creat an empty TaskManager
    fn new() -> Self;
    /// Add process back to ready queue
    fn add(&mut self,task:Arc<TaskControlBlock>);
    /// Take a process out of the ready queue
    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>>;
}

pub struct FIFOTaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}


/// A simple FIFO scheduler.
impl TaskManager for FIFOTaskManager {
    ///Creat an empty TaskManager
    fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

pub struct StrideManager{
    ready_queue: BinaryHeap<Arc<TaskControlBlock>>,
}

impl TaskManager for StrideManager{
    ///Create an empty TaskManager
    fn new() -> Self{
        Self{
            ready_queue:BinaryHeap::new(),
        }
    }

    ///Add process back to ready queue
    fn add(&mut self,task:Arc<TaskControlBlock>){
        self.ready_queue.push(task);
    }

    ///Task a process out of the ready queue
    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>>{
        let next_tcb = self.ready_queue.pop();
        next_tcb.clone().unwrap().inner_exclusive_access().add_stride();
        next_tcb
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<StrideManager> =
        unsafe { UPSafeCell::new(StrideManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
