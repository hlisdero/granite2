//! Representation of a condition variable in the Petri net.
//!
//! The behavior of the condition variable is modelled using
//! four places and two transitions. The corresponding net is:
//!
//! ```dot
//! digraph condvar {
//!     lost_signal_possible [shape="circle" xlabel="lost_signal_possible" label="•"];
//!     signal_input [shape="circle" xlabel="signal_input" label=""];
//!     wait_input [shape="circle" xlabel="wait_input" label=""];
//!     signal_output [shape="circle" xlabel="signal_output" label=""];
//!     lost_signal_transition [shape="box" xlabel="" label="lost_signal_transition"];
//!     signal_transition [shape="box" xlabel="" label="signal_transition"];
//!     lost_signal_possible -> lost_signal_transition;
//!     signal_input -> lost_signal_transition;
//!     lost_signal_transition -> lost_signal_possible;
//!     signal_input -> signal_transition;
//!     wait_input -> signal_transition;
//!     signal_transition -> signal_output;
//!     signal_transition -> lost_signal_possible;
//! }
//! ```
//!
//! `lost_signal_possible` and `lost_signal_transition` model the behavior of lost signals.
//! There is a conflict between the two transitions. If `wait()` is called before `signal()`,
//! the token in `signal_input` will be consumed by `lost_signal_transition`.
//! But if `wait()` is called first, then the token from `lost_signal_possible` will be removed,
//! preventing the `lost_signal_transition` from firing and ensuring that a token in `signal_output`
//! is set, which will allow the waiting thread to continue.
//!
//! This Petri net model is a slightly simplified version of the one presented in the paper
//! "Modelling Multithreaded Applications Using Petri Nets" by Kavi, Moshtaghi and Chen.
//! <https://www.researchgate.net/publication/220091454_Modeling_Multithreaded_Applications_Using_Petri_Nets>

use crate::naming::condvar::{place_labels, transition_labels};
use crate::petri_net_interface::{add_arc_place_transition, add_arc_transition_place};
use crate::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};

pub struct Condvar {
    lost_signal_possible: PlaceRef,
    signal_input: PlaceRef,
    wait_input: PlaceRef,
    signal_output: PlaceRef,
}

impl Condvar {
    /// Creates a new condition variable whose label is based on `index`.
    /// Adds its Petri net model to the net (4 places and 2 transitions).
    pub fn new(index: usize, net: &mut PetriNet) -> Self {
        let (p1, p2, p3, p4) = place_labels(index);
        let lost_signal_possible = net.add_place(&p1);
        let signal_input = net.add_place(&p2);
        let wait_input = net.add_place(&p3);
        let signal_output = net.add_place(&p4);

        net.add_token(&lost_signal_possible, 1).expect(
            "BUG: Adding initial token to `lost_signal_place` should not cause an overflow",
        );

        let (t1, t2) = transition_labels(index);
        let lost_signal_transition = net.add_transition(&t1);
        let signal_transition = net.add_transition(&t2);

        add_arc_place_transition(net, &lost_signal_possible, &lost_signal_transition);
        add_arc_place_transition(net, &signal_input, &lost_signal_transition);
        add_arc_place_transition(net, &wait_input, &signal_transition);
        add_arc_place_transition(net, &signal_input, &signal_transition);

        add_arc_transition_place(net, &lost_signal_transition, &lost_signal_possible);
        add_arc_transition_place(net, &signal_transition, &lost_signal_possible);
        add_arc_transition_place(net, &signal_transition, &signal_output);

        Self {
            lost_signal_possible,
            signal_input,
            wait_input,
            signal_output,
        }
    }

    /// Links the Petri net model of the condition variable to the representation of
    /// a call to `std::sync::Condvar::wait`.
    /// Connects the `lost_signal_possible` place to the `wait_start_transition` transition.
    /// Connects the `wait_start_transition` transition to the `wait_input` place.
    /// Connects the `signal_output` place to the `wait_end_transition` transition.
    pub fn link_to_wait_call(
        &self,
        wait_start_transition: &TransitionRef,
        wait_end_transition: &TransitionRef,
        net: &mut PetriNet,
    ) {
        add_arc_place_transition(net, &self.lost_signal_possible, wait_start_transition);
        add_arc_transition_place(net, wait_start_transition, &self.wait_input);
        add_arc_place_transition(net, &self.signal_output, wait_end_transition);
    }

    /// Links the Petri net model of the condition variable to the representation of
    /// a call to `std::sync::Condvar::notify_one`.
    /// Connects the `signal_transition` transition to the `signal_input` place.
    pub fn link_to_notify_one_call(&self, signal_transition: &TransitionRef, net: &mut PetriNet) {
        add_arc_transition_place(net, signal_transition, &self.signal_input);
    }
}
