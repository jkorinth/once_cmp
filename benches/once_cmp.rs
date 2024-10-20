use criterion::*;
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::RwLock;

const ITERATIONS: usize = 1000000;

macro_rules! do_once_lite_if_mut_bool {
    ($cond:expr, $e:expr) => {
        #[link_section = ".data.once"]
        static mut ALREADY_DONE: bool = false;
        if $cond && unsafe { !ALREADY_DONE } {
            unsafe { ALREADY_DONE = true; }
            $e;
        }
    }
}

macro_rules! warn_once_mut_bool {
    ($($arg:tt)*) => {
        do_once_lite_if_mut_bool!(true, println!($($arg)*))
    }
}

fn bench_mut_bool() {
    for _ in 0..ITERATIONS {
        warn_once_mut_bool!("complex {} arguments are void", 42);
    }
}

macro_rules! do_once_lite_if_mut_volatile_bool {
    ($cond:expr, $e:expr) => {
        #[link_section = ".data.once"]
        static mut ALREADY_DONE: bool = false;
        let p = std::ptr::addr_of_mut!(ALREADY_DONE) as *mut bool;
        if $cond && unsafe { !std::ptr::read_volatile(p) } {
            unsafe { std::ptr::write_volatile(p, true) };
            $e;
        }
    }
}

macro_rules! warn_once_mut_volatile_bool {
    ($($arg:tt)*) => {
        do_once_lite_if_mut_volatile_bool!(true, println!($($arg)*))
    }
}

fn bench_mut_volatile_bool() {
    for _ in 0..ITERATIONS {
        warn_once_mut_volatile_bool!("complex {} arguments are void", 42);
    }
}

macro_rules! do_once_lite_if_abool_relaxed {
    ($cond:expr, $e:expr) => {
        #[link_section = ".data.once"]
        static ALREADY_DONE: AtomicBool = AtomicBool::new(false);
        if $cond && !ALREADY_DONE.load(Ordering::Relaxed) {
            ALREADY_DONE.store(true, Ordering::Relaxed);
            $e;
        }
    }
}

macro_rules! warn_once_abool_relaxed {
    ($($arg:tt)*) => {
        do_once_lite_if_abool_relaxed!(true, println!($($arg)*))
    }
}

fn bench_abool_relaxed() {
    for _ in 0..ITERATIONS {
        warn_once_abool_relaxed!("complex {} arguments are void", 42);
    }
}

macro_rules! do_once_lite_if_abool_seqcst {
    ($cond:expr, $e:expr) => {
        #[link_section = ".data.once"]
        static ALREADY_DONE: AtomicBool = AtomicBool::new(false);
        if $cond && !ALREADY_DONE.load(Ordering::SeqCst) {
            ALREADY_DONE.store(true, Ordering::SeqCst);
            $e;
        }
    }
}

macro_rules! warn_once_abool_seqcst {
    ($($arg:tt)*) => {
        do_once_lite_if_abool_seqcst!(true, println!($($arg)*))
    }
}

fn bench_abool_seqcst() {
    for _ in 0..ITERATIONS {
        warn_once_abool_seqcst!("complex {} arguments are void", 42);
    }
}

macro_rules! do_once_lite_if_rwlock {
    ($cond:expr, $e:expr) => {
        #[link_section = ".data.once"]
        static ALREADY_DONE: RwLock<bool> = RwLock::new(false);
        if $cond && !*ALREADY_DONE.read().unwrap() {
            *ALREADY_DONE.write().unwrap() = true;
            $e;
        }
    }
}

macro_rules! warn_once_rwlock {
    ($($arg:tt)*) => {
        do_once_lite_if_rwlock!(true, println!($($arg)*))
    }
}

fn bench_rwlock() {
    for _ in 0..ITERATIONS {
        warn_once_rwlock!("complex {} arguments are void", 42);
    }
}

fn criterion_benchmark_warn_onces(c: &mut Criterion) {
    c.bench_function("mut_bool", |b| b.iter(bench_mut_bool));
    c.bench_function("mut_volatile_bool", |b| b.iter(bench_mut_volatile_bool));
    c.bench_function("abool_relaxed", |b| b.iter(bench_abool_relaxed));
    c.bench_function("abool_seqcst", |b| b.iter(bench_abool_seqcst));
    c.bench_function("rwlock", |b| b.iter(bench_rwlock));
}

criterion_group!(benches, criterion_benchmark_warn_onces);
criterion_main!(benches);
