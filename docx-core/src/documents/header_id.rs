/*
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static HEADER_ID: AtomicUsize = AtomicUsize::new(1);

#[cfg(not(test))]
pub fn generate_header_id() -> usize 