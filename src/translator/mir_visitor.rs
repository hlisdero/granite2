//! Implementation of the trait `rustc_middle::mir::visit::Visitor` for Translator.
//! <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/visit/trait.Visitor.html>
//!
//! This trait defines a method for visiting every possible MIR element.
//! It is not required to implement every method, only for the elements we care about.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>

use crate::translator::multithreading::identify_assign_of_local_with_join_handle;
use crate::translator::sync::{
    identify_assign_of_copy_of_reference_of_local_with_mutex,
    identify_assign_of_reference_of_arc_with_mutex,
    identify_assign_of_reference_of_local_with_mutex,
};
use crate::translator::Translator;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::TerminatorKind;

impl<'tcx> Visitor<'tcx> for Translator<'tcx> {
    /// Entering a new basic block of the current MIR function.
    /// Activate it. This is the first step to start processing it.
    fn visit_basic_block_data(
        &mut self,
        block: rustc_middle::mir::BasicBlock,
        data: &rustc_middle::mir::BasicBlockData<'tcx>,
    ) {
        let function = self.call_stack.peek_mut();
        function.activate_block(block, &mut self.net);

        self.super_basic_block_data(block, data);
    }

    fn visit_assign(
        &mut self,
        place: &rustc_middle::mir::Place<'tcx>,
        rvalue: &rustc_middle::mir::Rvalue<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        let function = self.call_stack.peek_mut();
        let body = self.tcx.optimized_mir(function.def_id);

        if let Some((lhs, rhs)) =
            identify_assign_of_reference_of_local_with_mutex(place, rvalue, body)
        {
            function.memory.link_local_to_same_mutex(lhs, rhs);
        }

        if let Some((lhs, rhs)) =
            identify_assign_of_copy_of_reference_of_local_with_mutex(place, rvalue, body)
        {
            function.memory.link_local_to_same_mutex(lhs, rhs);
        }

        if let Some((lhs, rhs)) =
            identify_assign_of_reference_of_arc_with_mutex(place, rvalue, body)
        {
            function.memory.link_local_to_same_mutex(lhs, rhs);
        }

        if let Some((lhs, rhs)) = identify_assign_of_local_with_join_handle(place, rvalue, body) {
            function.memory.link_local_to_same_join_handle(lhs, rhs);
        }

        self.super_assign(place, rvalue, location);
    }

    fn visit_statement(
        &mut self,
        statement: &rustc_middle::mir::Statement<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        let function = self.call_stack.peek_mut();
        function.add_statement(statement, &mut self.net);

        self.super_statement(statement, location);
    }

    fn visit_terminator(
        &mut self,
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        location: rustc_middle::mir::Location,
    ) {
        let function = self.call_stack.peek_mut();
        function.finish_statement_block(&mut self.net);

        match terminator.kind {
            TerminatorKind::Goto { target } => {
                function.goto(target, &mut self.net);
            }
            TerminatorKind::SwitchInt {
                discr: _,
                ref targets,
            } => {
                // Convert the specific type for the targets vector into a `std::collections::Vec`
                // <rustc_middle::mir::terminator::SwitchTargets>
                function.switch_int(targets.all_targets().to_vec(), &mut self.net);
            }
            TerminatorKind::Resume | TerminatorKind::Abort => {
                function.unwind(&self.program_panic, &mut self.net);
            }
            TerminatorKind::Return => {
                function.return_statement(&mut self.net);
            }
            TerminatorKind::Unreachable => {
                function.unreachable(&self.program_end, &mut self.net);
            }
            TerminatorKind::Drop {
                place,
                target,
                unwind,
            } => {
                let transition_drop = function.drop(target, unwind, &mut self.net);
                self.mutex_manager.handle_lock_guard_drop(
                    place,
                    &transition_drop,
                    &function.memory,
                    &mut self.net,
                );
            }
            TerminatorKind::DropAndReplace { .. } => {
                unimplemented!("TerminatorKind::DropAndReplace not implemented yet")
            }
            TerminatorKind::Call {
                ref func,
                ref args,
                destination,
                target,
                cleanup,
                from_hir_call: _,
                fn_span: _,
            } => {
                self.call_function(func, args, destination, target, cleanup);
            }
            TerminatorKind::Assert {
                cond: _,
                expected: _,
                msg: _,
                target,
                cleanup,
            } => {
                function.assert(target, cleanup, &mut self.net);
            }
            TerminatorKind::Yield { .. } => {
                unimplemented!("TerminatorKind::Yield not implemented yet")
            }
            TerminatorKind::GeneratorDrop => {
                unimplemented!("TerminatorKind::GeneratorDrop not implemented yet")
            }
            TerminatorKind::FalseEdge { .. } => {
                unimplemented!("TerminatorKind::FalseEdge not implemented yet")
            }
            TerminatorKind::FalseUnwind { .. } => {
                unimplemented!("TerminatorKind::FalseUnwind not implemented yet")
            }
            TerminatorKind::InlineAsm { .. } => {
                unimplemented!("TerminatorKind::InlineAsm not implemented yet")
            }
        }

        self.super_terminator(terminator, location);
    }
}
