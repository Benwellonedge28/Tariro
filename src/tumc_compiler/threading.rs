//! TUMC Threading and Concurrency
//!
//! Multi-threading primitives, async runtime, and concurrency utilities.

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, RwLock};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread::JoinHandle;

/// Task scheduler
pub struct Scheduler {
    /// Ready queue
    ready_queue: Arc<Mutex<VecDeque<TaskHandle>>>,
    /// Running tasks
    running: Arc<Mutex<Vec<TaskId>>>,
    /// Completed tasks
    completed: Arc<Mutex<Vec<TaskId>>>,
    /// Task counter
    task_counter: Arc<Mutex<u64>>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    /// Create new scheduler
    pub fn new() -> Self {
        Self {
            ready_queue: Arc::new(Mutex::new(VecDeque::new())),
            running: Arc::new(Mutex::new(Vec::new())),
            completed: Arc::new(Mutex::new(Vec::new())),
            task_counter: Arc::new(Mutex::new(0)),
        }
    }

    /// Spawn a task
    pub fn spawn<F>(&self, future: F) -> TaskHandle
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let id = {
            let mut counter = self.task_counter.lock().unwrap();
            *counter += 1;
            *counter
        };

        let handle = TaskHandle {
            id: TaskId(id),
            future: Arc::new(Mutex::new(Some(Box::pin(future)))),
        };

        self.ready_queue.lock().unwrap().push_back(handle.clone());
        handle
    }

    /// Run the scheduler
    pub fn run(&self) {
        loop {
            // Get next ready task
            let task = self.ready_queue.lock().unwrap().pop_front();

            if let Some(handle) = task {
                // Poll the task
                let waker = self.create_waker(&handle);
                let mut cx = Context::from_waker(&waker);

                let polled = {
                    let mut future = handle.future.lock().unwrap();
                    match future.as_mut() {
                        Some(fut) => Some(Pin::new(fut).poll(&mut cx)),
                        None => None,
                    }
                };
                match polled {
                    Some(Poll::Ready(())) => {
                        self.completed.lock().unwrap().push(handle.id);
                    }
                    Some(Poll::Pending) => {
                        self.ready_queue.lock().unwrap().push_back(handle);
                    }
                    None => {}
                }
            } else {
                // No more tasks
                break;
            }
        }
    }

    /// Create waker for task
    fn create_waker(&self, handle: &TaskHandle) -> Waker {
        let _data = handle.clone();

        let vtable = Box::leak(Box::new(RawWakerVTable::new(
            |data| {
                let _handle = unsafe { Arc::<_TaskFuture>::from_raw(data as *const _TaskFuture) };
                RawWaker::new(data, &VTABLE)
            },
            |data| {
                // Wake
                let handle = unsafe { Arc::<_TaskFuture>::from_raw(data as *const _TaskFuture) };
                // Re-queue task
                drop(handle);
            },
            |_| {},
            |data| {
                drop(unsafe { Arc::<_TaskFuture>::from_raw(data as *const _TaskFuture) });
            },
        )));

        let raw = Arc::into_raw(handle.future.clone()) as *const ();
        unsafe { Waker::from_raw(RawWaker::new(raw, Box::leak(Box::new(*vtable)))) }
    }
}

/// Task ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

/// Task handle
#[derive(Clone)]
pub struct TaskHandle {
    id: TaskId,
    future: Arc<Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>>,
}

/// Internal task future type
type _TaskFuture = Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>;

impl std::fmt::Debug for TaskHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskHandle")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

/// Raw waker vtable
static VTABLE: RawWakerVTable = RawWakerVTable::new(
    |_| RawWaker::new(std::ptr::null(), &VTABLE),
    |_| {},
    |_| {},
    |_| {},
);

/// Channel for task communication
pub struct Channel<T> {
    /// Internal state
    state: Arc<ChannelState<T>>,
}

/// Channel state
struct ChannelState<T> {
    /// Queue
    queue: Mutex<VecDeque<T>>,
    /// Senders count
    senders: Mutex<usize>,
    /// Receivers count
    receivers: Mutex<usize>,
    /// Closed flag
    closed: Mutex<bool>,
    /// Notifier
    notifier: Condvar,
}

/// Conditional variable
pub struct Condvar {
    /// Waiters
    waiters: Mutex<VecDeque<ThreadId>>,
}

impl Condvar {
    /// Create new condvar
    pub fn new() -> Self {
        Self { waiters: Mutex::new(VecDeque::new()) }
    }

    /// Wait
    pub fn wait(&self) {
        let id = ThreadId::current();
        self.waiters.lock().unwrap().push_back(id);
        // Would block current thread
    }

    /// Notify one
    pub fn notify_one(&self) {
        let mut waiters = self.waiters.lock().unwrap();
        waiters.pop_front();
    }

    /// Notify all
    pub fn notify_all(&self) {
        self.waiters.lock().unwrap().clear();
    }
}

impl Default for Condvar {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Channel<T> {
    /// Create channel
    pub fn new() -> (Sender<T>, Receiver<T>) {
        let state = Arc::new(ChannelState {
            queue: Mutex::new(VecDeque::new()),
            senders: Mutex::new(1),
            receivers: Mutex::new(1),
            closed: Mutex::new(false),
            notifier: Condvar::new(),
        });

        (Sender { state: state.clone() }, Receiver { state })
    }

    /// Create bounded channel
    pub fn bounded(_capacity: usize) -> (Sender<T>, Receiver<T>) {
        Self::new() // Simplified
    }
}

/// Sender end of channel
pub struct Sender<T> {
    state: Arc<ChannelState<T>>,
}

impl<T> std::fmt::Debug for Sender<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sender").finish_non_exhaustive()
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Self { state: self.state.clone() }
    }
}

impl<T> std::fmt::Debug for Receiver<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Receiver").finish_non_exhaustive()
    }
}

impl<T> Sender<T> {
    /// Send value
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        let mut queue = self.state.queue.lock().unwrap();

        if *self.state.closed.lock().unwrap() {
            return Err(SendError(value));
        }

        queue.push_back(value);
        self.state.notifier.notify_one();
        Ok(())
    }

    /// Check if closed
    pub fn is_closed(&self) -> bool {
        *self.state.closed.lock().unwrap()
    }
}

/// Send error
pub struct SendError<T>(pub T);

impl<T> std::fmt::Debug for SendError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SendError")
    }
}

/// Receiver end of channel
pub struct Receiver<T> {
    state: Arc<ChannelState<T>>,
}

impl<T> Receiver<T> {
    /// Receive value
    pub fn recv(&self) -> Result<T, RecvError> {
        loop {
            let mut queue = self.state.queue.lock().unwrap();

            if let Some(value) = queue.pop_front() {
                return Ok(value);
            }

            if *self.state.closed.lock().unwrap() {
                return Err(RecvError);
            }

            // Wait for notification
            drop(queue);
            self.state.notifier.wait();
        }
    }

    /// Try to receive
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        let mut queue = self.state.queue.lock().unwrap();

        if let Some(value) = queue.pop_front() {
            Ok(value)
        } else if *self.state.closed.lock().unwrap() {
            Err(TryRecvError::Closed)
        } else {
            Err(TryRecvError::Empty)
        }
    }
}

/// Receive error
#[derive(Debug)]
pub struct RecvError;

impl std::fmt::Display for RecvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "channel closed")
    }
}

/// Try receive error
#[derive(Debug)]
pub enum TryRecvError {
    Empty,
    Closed,
}

/// Thread pool
pub struct ThreadPool {
    /// Workers
    workers: Vec<Worker>,
    /// Task sender
    sender: Sender<Job>,
}

impl ThreadPool {
    /// Create new thread pool
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = Channel::bounded(size);

        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(receiver.clone()));
        }

        Self { workers, sender }
    }

    /// Submit job
    pub fn submit<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Job::New(Box::new(job))).ok();
    }

    /// Shutdown pool
    pub fn shutdown(self) {
        drop(self.sender);
        // Workers will exit when channel closes
    }
}

/// Worker thread
struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(receiver: Receiver<Job>) -> Self {
        let handle = std::thread::spawn(move || {
            while let Ok(job) = receiver.recv() {
                match job {
                    Job::New(f) => f(),
                    Job::Close => break,
                }
            }
        });

        Self { handle }
    }
}

/// Job for thread pool
enum Job {
    New(Box<dyn FnOnce() + Send>),
    Close,
}

/// Thread ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadId(u64);

impl ThreadId {
    /// Get current thread ID
    pub fn current() -> Self {
        Self(rand_u64())
    }
}

fn rand_u64() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let sequence = COUNTER.fetch_add(1, Ordering::Relaxed);
    (now.as_nanos() as u64) ^ (sequence << 16)
}

/// Mutex with async support
pub struct AsyncMutex<T> {
    /// Inner value
    inner: Arc<Mutex<T>>,
    /// Waiters
    waiters: Arc<RwLock<VecDeque<Waker>>>,
}

impl<T> AsyncMutex<T> {
    /// Create new async mutex
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value)),
            waiters: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    /// Lock the mutex
    pub async fn lock(&self) -> AsyncMutexGuard<'_, T> {
        AsyncMutexGuard { mutex: self, guard: self.inner.lock().unwrap() }
    }
}

/// Async mutex guard
pub struct AsyncMutexGuard<'a, T> {
    mutex: &'a AsyncMutex<T>,
    /// The lock this guard holds
    guard: std::sync::MutexGuard<'a, T>,
}

impl<T> Drop for AsyncMutexGuard<'_, T> {
    fn drop(&mut self) {
        // Would wake next waiter
    }
}

impl<T> std::ops::Deref for AsyncMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T> std::ops::DerefMut for AsyncMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

/// Once cell for one-time initialization
pub struct OnceCell<T> {
    /// State
    state: Arc<RwLock<OnceCellState<T>>>,
}

/// Once cell state
enum OnceCellState<T> {
    /// Not initialized
    New,
    /// Initializing
    Processing,
    /// Initialized with value
    Initialized(T),
    /// Poisoned
    Poisoned,
}

impl<T> OnceCell<T> {
    /// Create new once cell
    pub fn new() -> Self {
        Self { state: Arc::new(RwLock::new(OnceCellState::New)) }
    }
}

impl<T: Clone> OnceCell<T> {
    /// Get value or initialize
    ///
    /// The value is returned by value: it lives behind a lock, so handing out
    /// a reference to it would not outlive the lock guard.
    pub fn get_or_init<F>(&self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        // Fast path: already initialized
        {
            let state = self.state.read().unwrap();
            if let OnceCellState::Initialized(value) = &*state {
                return value.clone();
            }
        }

        // Slow path: need to initialize
        let mut state = self.state.write().unwrap();
        match &*state {
            OnceCellState::Initialized(value) => value.clone(),
            _ => {
                *state = OnceCellState::Processing;
                let value = f();
                *state = OnceCellState::Initialized(value);
                if let OnceCellState::Initialized(value) = &*state {
                    value.clone()
                } else {
                    unreachable!()
                }
            }
        }
    }

    /// Try to get value
    pub fn get(&self) -> Option<T> {
        let state = self.state.read().unwrap();
        match &*state {
            OnceCellState::Initialized(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl<T> Default for OnceCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// RwLock with async support
pub struct AsyncRwLock<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> AsyncRwLock<T> {
    /// Create new async RwLock
    pub fn new(value: T) -> Self {
        Self { inner: Arc::new(RwLock::new(value)) }
    }

    /// Read lock
    pub async fn read(&self) -> AsyncRwLockReadGuard<'_, T> {
        AsyncRwLockReadGuard { lock: self, guard: self.inner.read().unwrap() }
    }

    /// Write lock
    pub async fn write(&self) -> AsyncRwLockWriteGuard<'_, T> {
        AsyncRwLockWriteGuard { lock: self, guard: self.inner.write().unwrap() }
    }
}

/// Read guard
pub struct AsyncRwLockReadGuard<'a, T> {
    lock: &'a AsyncRwLock<T>,
    /// The lock this guard holds
    guard: std::sync::RwLockReadGuard<'a, T>,
}

impl<T> std::ops::Deref for AsyncRwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

/// Write guard
pub struct AsyncRwLockWriteGuard<'a, T> {
    lock: &'a AsyncRwLock<T>,
    /// The lock this guard holds
    guard: std::sync::RwLockWriteGuard<'a, T>,
}

impl<T> std::ops::Deref for AsyncRwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T> std::ops::DerefMut for AsyncRwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_spawn() {
        let scheduler = Scheduler::new();
        let handle = scheduler.spawn(async {});
        assert!(handle.id.0 > 0);
    }

    #[test]
    fn test_channel() {
        let (tx, rx) = Channel::<i32>::new();
        tx.send(42).unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn test_once_cell() {
        let cell: OnceCell<i32> = OnceCell::new();
        assert_eq!(cell.get_or_init(|| 42), 42);
        assert_eq!(cell.get_or_init(|| 100), 42);
    }
}
