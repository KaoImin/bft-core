use crate::algorithm::Step;

use std::cmp::{Ord, Ordering, PartialOrd};
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use min_max_heap::MinMaxHeap;

/// Timer infomation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct TimeoutInfo {
    /// A timeval of a timer.
    pub(crate) timeval: Instant,
    /// The height of the timer.
    pub(crate) height: u64,
    /// The round of the timer.
    pub(crate) round: u64,
    /// The step of the timer.
    pub(crate) step: Step,
}

impl PartialOrd for TimeoutInfo {
    fn partial_cmp(&self, other: &TimeoutInfo) -> Option<Ordering> {
        self.timeval.partial_cmp(&other.timeval)
    }
}

impl Ord for TimeoutInfo {
    fn cmp(&self, other: &TimeoutInfo) -> Ordering {
        self.timeval.cmp(&other.timeval)
    }
}

/// Sender and receiver of a timeout infomation channel.
pub(crate) struct WaitTimer {
    timer_seter: Receiver<TimeoutInfo>,
    timer_notify: Sender<TimeoutInfo>,
}

impl WaitTimer {
    /// A function to create a new timeout infomation channel.
    pub(crate) fn new(ts: Sender<TimeoutInfo>, rs: Receiver<TimeoutInfo>) -> WaitTimer {
        WaitTimer {
            timer_notify: ts,
            timer_seter: rs,
        }
    }

    /// A function to start a timer.
    pub(crate) fn start(&self) {
        let mut timer_heap = MinMaxHeap::<TimeoutInfo>::new();

        loop {
            // take the peek of the min-heap-timer sub now as the sleep time otherwise set timeout as 100
            let timeout = if !timer_heap.is_empty() {
                let peek_min_interval = timer_heap.peek_min().unwrap().timeval;
                let now = Instant::now();
                if peek_min_interval > now {
                    peek_min_interval - now
                } else {
                    Duration::new(0, 0)
                }
            } else {
                Duration::from_secs(100)
            };

            let set_time = self.timer_seter.recv_timeout(timeout);

            // put the timeval into a timerheap
            // put the TimeoutInfo into a hashmap, K: timeval  V: TimeoutInfo
            if set_time.is_ok() {
                let time_out = set_time.unwrap();
                timer_heap.push(time_out);
            }

            if !timer_heap.is_empty() {
                let now = Instant::now();

                // if some timers are set as the same time, send timeout messages and pop them
                while !timer_heap.is_empty()
                    && now >= timer_heap.peek_min().cloned().unwrap().timeval
                {
                    self.timer_notify
                        .send(timer_heap.pop_min().unwrap())
                        .unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;

    impl TimeoutInfo {
        fn new(t: u64, h: u64, r: u64) -> Self {
            let timeval: Instant = Instant::now() + Duration::from_millis(t);
            TimeoutInfo {
                timeval,
                height: h,
                round: r,
                step: Step::default(),
            }
        }
    }

    fn gen_timeoutinfo() -> Vec<TimeoutInfo> {
        let mut res = Vec::new();
        res.push(TimeoutInfo::new(150, 0, 0));
        res.push(TimeoutInfo::new(180, 2, 1));
        res.push(TimeoutInfo::new(50, 3, 6));
        res
    }

    #[test]
    fn test_timer_heap() {
        let (s_1, r_1) = unbounded();
        let (s_2, r_2) = unbounded();

        ::std::thread::spawn(move || {
            let timer = WaitTimer::new(s_2, r_1);
            timer.start();
        });

        let infos = gen_timeoutinfo();
        for ti in infos.clone().into_iter() {
            s_1.send(ti).unwrap();
        }
        let now = Instant::now();

        assert_eq!(infos[2], r_2.recv().unwrap());
        println!("{:?}", Instant::now() - now);
        assert_eq!(infos[0], r_2.recv().unwrap());
        println!("{:?}", Instant::now() - now);
        assert_eq!(infos[1], r_2.recv().unwrap());
        println!("{:?}", Instant::now() - now);
    }
}
