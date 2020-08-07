// Copyright 2016 Amanieu d'Antras
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use core::cell::Cell;
use instant::Instant;

use std::thread;
use std::sync::{ SgxThreadMutex, SgxThreadCondvar };
// x32 Linux uses a non-standard type for tv_nsec in timespec.
// See https://sourceware.org/bugzilla/show_bug.cgi?id=16437
// #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
// #[allow(non_camel_case_types)]
// type tv_nsec_t = i64;
// #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
// #[allow(non_camel_case_types)]
// type tv_nsec_t = libc::c_long;

// Helper type for putting a thread to sleep until some other thread wakes it up
pub struct ThreadParker {
    should_park: Cell<bool>,
    mutex: SgxThreadMutex,
    condvar: SgxThreadCondvar,
    initialized: Cell<bool>,
}

impl super::ThreadParkerT for ThreadParker {
    type UnparkHandle = UnparkHandle;

    const IS_CHEAP_TO_CONSTRUCT: bool = false;

    #[inline]
    fn new() -> ThreadParker {
        ThreadParker {
            should_park: Cell::new(false),
            mutex: SgxThreadMutex::new(),
            condvar: SgxThreadCondvar::new(),
            initialized: Cell::new(false),
        }
    }

    #[inline]
    unsafe fn prepare_park(&self) {
        self.should_park.set(true);
        if !self.initialized.get() {
            self.init();
            self.initialized.set(true);
        }
    }

    #[inline]
    unsafe fn timed_out(&self) -> bool {
        // We need to grab the mutex here because another thread may be
        // concurrently executing UnparkHandle::unpark, which is done without
        // holding the queue lock.
        let _r = self.mutex.lock();
        let should_park = self.should_park.get();
        let _r = self.mutex.unlock();
        should_park
    }

    #[inline]
    unsafe fn park(&self) {
        let _r = self.mutex.lock();
        while self.should_park.get() {
            let _r = self.condvar.wait(&self.mutex);
        }
        let _r = self.mutex.unlock();
    }

    #[inline]
    unsafe fn park_until(&self, timeout: Instant) -> bool {
        let _r = self.mutex.lock();
        while self.should_park.get() {
            let now = Instant::now();
            if timeout <= now {
                let _r = self.mutex.unlock();
                return false;
            }
            if timeout.elapsed().as_secs() > sgx_libc::time_t::max_value() as u64 {
                // Timeout calculation overflowed, just sleep indefinitely
                let _r = self.condvar.wait(&self.mutex);
            } else {
                match timeout.checked_sub(now.elapsed()) {
                    Some(tm) => {
                        let _r = self.condvar.wait_timeout(&self.mutex, tm.elapsed());
                    },
                    None => {
                        let _r = self.mutex.unlock();
                        return false;
                    }
                };
            }
        }
        let _r = self.mutex.unlock();
        true
    }

    #[inline]
    unsafe fn unpark_lock(&self) -> UnparkHandle {
        let _r = self.mutex.unlock();
        UnparkHandle {
            thread_parker: self,
        }
    }
}

impl ThreadParker {
    //// Initializes the condvar to use CLOCK_MONOTONIC instead of CLOCK_REALTIME.
    //#[cfg(any(target_os = "macos", target_os = "ios", target_os = "android"))]
    #[inline]
    unsafe fn init(&self) {}

    //// Initializes the condvar to use CLOCK_MONOTONIC instead of CLOCK_REALTIME.
    // #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "android")))]
    // #[inline]
    // unsafe fn init(&self) {
    //     let mut attr = MaybeUninit::<libc::pthread_condattr_t>::uninit();
    //     let r = libc::pthread_condattr_init(attr.as_mut_ptr());
    //     debug_assert_eq!(r, 0);
    //     let r = libc::pthread_condattr_setclock(attr.as_mut_ptr(), libc::CLOCK_MONOTONIC);
    //     debug_assert_eq!(r, 0);
    //     let r = libc::pthread_cond_init(self.condvar.get(), attr.as_ptr());
    //     debug_assert_eq!(r, 0);
    //     let r = libc::pthread_condattr_destroy(attr.as_mut_ptr());
    //     debug_assert_eq!(r, 0);
    // }
}

impl Drop for ThreadParker {
    #[inline]
    fn drop(&mut self) {
        // On DragonFly pthread_mutex_destroy() returns EINVAL if called on a
        // mutex that was just initialized with libc::PTHREAD_MUTEX_INITIALIZER.
        // Once it is used (locked/unlocked) or pthread_mutex_init() is called,
        // this behaviour no longer occurs. The same applies to condvars.
        unsafe {
            let _r = self.mutex.destroy();
            let _r = self.condvar.destroy();
        }
            
    }
}

pub struct UnparkHandle {
    thread_parker: *const ThreadParker,
}

impl super::UnparkHandleT for UnparkHandle {
    #[inline]
    unsafe fn unpark(self) {
        (*self.thread_parker).should_park.set(false);

        // We notify while holding the lock here to avoid races with the target
        // thread. In particular, the thread could exit after we unlock the
        // mutex, which would make the condvar access invalid memory.
        let _r = (*self.thread_parker).condvar.signal();
        let _r = (*self.thread_parker).mutex.unlock();
    }
}

// Returns the current time on the clock used by pthread_cond_t as a timespec.

// #[cfg(not(any(target_os = "macos", target_os = "ios")))]
// #[inline]
// fn timespec_now() -> libc::timespec {
//     let mut now = MaybeUninit::<libc::timespec>::uninit();
//     let clock = if cfg!(target_os = "android") {
//         // Android doesn't support pthread_condattr_setclock, so we need to
//         // specify the timeout in CLOCK_REALTIME.
//         libc::CLOCK_REALTIME
//     } else {
//         libc::CLOCK_MONOTONIC
//     };
//     let r = unsafe { libc::clock_gettime(clock, now.as_mut_ptr()) };
//     debug_assert_eq!(r, 0);
//     // SAFETY: We know `libc::clock_gettime` has initialized the value.
//     unsafe { now.assume_init() }
// }

// // Converts a relative timeout into an absolute timeout in the clock used by
// // pthread_cond_t.
// #[inline]
// fn timeout_to_timespec(timeout: Duration) -> Option<libc::timespec> {
//     // Handle overflows early on
//     if timeout.as_secs() > libc::time_t::max_value() as u64 {
//         return None;
//     }

//     let now = timespec_now();
//     let mut nsec = now.tv_nsec + timeout.subsec_nanos() as tv_nsec_t;
//     let mut sec = now.tv_sec.checked_add(timeout.as_secs() as libc::time_t);
//     if nsec >= 1_000_000_000 {
//         nsec -= 1_000_000_000;
//         sec = sec.and_then(|sec| sec.checked_add(1));
//     }

//     sec.map(|sec| libc::timespec {
//         tv_nsec: nsec,
//         tv_sec: sec,
//     })
// }

#[inline]
pub fn thread_yield() {
    thread::yield_now();
}
