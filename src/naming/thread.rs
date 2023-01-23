//! Submodule that defines the naming of places and transitions in the Petri net
//! that concern the translation of functions related to threads.
//!
//! These functions are called every time that a new place or transition
//! in the resulting net is created.
//! This ensures a consistent naming and provides a centralized place to tweak
//! the configuration if needed.
//!
//! All functions listed here should have an `#[inline]` attribute for performance reasons.
//! See the reference for more information:
//! <https://doc.rust-lang.org/stable/reference/attributes/codegen.html>

/// Label of the transitions that represent a call to `std::thread::spawn`.
#[inline]
pub fn spawn_transition_labels(index: usize) -> (String, String) {
    (
        format!("std_thread_spawn_{index}"),
        format!("std_thread_spawn_{index}_UNWIND"),
    )
}

/// Label of the transitions that represent a call to `std::thread::JoinHandle::<T>::join`.
#[inline]
pub fn join_transition_labels(index: usize) -> (String, String) {
    (
        format!("std_thread_JoinHandle_T_join_{index}"),
        format!("std_thread_JoinHandle_T_join_{index}_UNWIND"),
    )
}

/// Label of the place that models the thread start state.
#[inline]
pub fn start_place_label(index: usize) -> String {
    format!("THREAD_{index}_START")
}

/// Label of the place that models the thread end state.
#[inline]
pub fn end_place_label(index: usize) -> String {
    format!("THREAD_{index}_END")
}
