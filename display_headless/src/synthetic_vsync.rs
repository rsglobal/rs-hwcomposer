/*
 * SPDX-License-Identifier: Apache-2.0
 */

use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

pub(crate) trait IVsyncCallback: Send + Sync {
    fn on_vsync(&self, timestamp: u64, duration: Duration);
}

pub(crate) struct SyntheticVsync {
    shared: Arc<Shared>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl SyntheticVsync {
    pub(crate) fn new(callback: Arc<dyn IVsyncCallback>) -> Self {
        let shared = Arc::new(Shared {
            task_queue: Mutex::new(VecDeque::new()),
            cv: Condvar::new(),
            callback,
        });

        SyntheticVsync {
            shared: shared.clone(),
            thread: Some(std::thread::spawn(|| thread_fn(shared))),
        }
    }

    pub(crate) fn set_period(&self, period: Option<Duration>) {
        self.shared.task_queue.lock().unwrap().push_back(Task::NewPeriod(period));
        self.shared.cv.notify_all();
    }
}

impl Drop for SyntheticVsync {
    fn drop(&mut self) {
        self.shared.task_queue.lock().unwrap().push_back(Task::Exit);
        self.shared.cv.notify_all();
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap_or_default();
        }
    }
}

enum Task {
    NewPeriod(Option<Duration>),
    Exit,
}

struct Shared {
    pub(crate) task_queue: Mutex<VecDeque<Task>>,
    pub(crate) cv: Condvar,
    pub(crate) callback: Arc<dyn IVsyncCallback>,
}

fn thread_fn(shared: Arc<Shared>) {
    log::info!("Vsync thread started");

    let mut period: Option<Duration> = None;
    let mut send_event: Option<(u64, Duration)> = None;

    loop {
        if let Some((ts, period)) = send_event.take() {
            shared.callback.on_vsync(ts, period);
        }

        let mut task_queue = shared.task_queue.lock().unwrap();
        if task_queue.is_empty() {
            if let Some(period) = &period {
                let now = now_monotonic_ns();
                let period_ns = period.as_nanos() as u64;
                let next_period_ns = ((now / period_ns) + 1) * period_ns;
                let sleep_for = Duration::from_nanos(next_period_ns - now);

                let res = shared.cv.wait_timeout(task_queue, sleep_for).unwrap();
                if res.1.timed_out() {
                    send_event = Some((next_period_ns, *period));
                }
                continue;
            }

            // wait for a task indefinitely
            let _unused = shared.cv.wait(task_queue).unwrap();
            continue;
        }

        while let Some(task) = task_queue.pop_front() {
            match task {
                Task::NewPeriod(p) => period = p,
                Task::Exit => {
                    log::info!("Vsync thread exited");
                    return;
                }
            }
        }
    }
}

fn now_monotonic_ns() -> u64 {
    let mut time = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    let ret = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC_RAW, &mut time) };
    assert!(ret == 0);
    time.tv_sec as u64 * 1000000000 + time.tv_nsec as u64
}
