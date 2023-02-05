//! Module that defines some functions to use as wrappers
//! around the methods provided by the library `netcrab`.
//! This ensures a proper error handling when adding arcs to the net.

pub use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

/// Adds an arc from a place to a transition with multiplicity one.
///
/// # Panics
///
/// If the arc could not be created, then the function panics.
#[inline]
pub fn add_arc_place_transition(
    net: &mut PetriNet,
    place_ref: &PlaceRef,
    transition_ref: &TransitionRef,
) {
    net.add_arc_place_transition(place_ref, transition_ref)
        .unwrap_or_else(|_| {
            panic!(
                "BUG: Adding an arc from `{}` to `{}` should not fail",
                place_ref.label(),
                transition_ref.label()
            );
        });
}

/// Adds an arc from a transition to a place with multiplicity one.
///
/// # Panics
///
/// If the arc could not be created, then the function panics.
#[inline]
pub fn add_arc_transition_place(
    net: &mut PetriNet,
    transition_ref: &TransitionRef,
    place_ref: &PlaceRef,
) {
    net.add_arc_transition_place(transition_ref, place_ref)
        .unwrap_or_else(|_| {
            panic!(
                "BUG: Adding an arc from `{}` to `{}` should not fail",
                transition_ref.label(),
                place_ref.label()
            );
        });
}
