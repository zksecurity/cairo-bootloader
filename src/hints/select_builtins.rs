use std::{any::Any, collections::HashMap};

use cairo_vm::{
    hint_processor::{
        builtin_hint_processor::hint_utils::get_integer_from_var_name,
        hint_processor_definition::{HintExtension, HintReference},
    },
    serde::deserialize_program::ApTracking,
    types::{errors::math_errors::MathError, exec_scope::ExecutionScopes},
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
};
use num_traits::ToPrimitive;

use crate::hints::vars;

/// Implements
/// %{ vm_enter_scope({'n_selected_builtins': ids.n_selected_builtins}) %}
pub fn select_builtins_enter_scope(
    vm: &mut VirtualMachine,
    exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
) -> Result<HintExtension, HintError> {
    let n_selected_builtins = get_integer_from_var_name(vars::N_SELECTED_BUILTINS, vm, ids_data, ap_tracking)?;
    let n_selected_builtins = n_selected_builtins
        .to_usize()
        .ok_or(MathError::Felt252ToUsizeConversion(Box::new(n_selected_builtins)))?;

    exec_scopes.enter_scope(HashMap::from([(
        vars::N_SELECTED_BUILTINS.to_string(),
        Box::new(n_selected_builtins) as Box<dyn Any>,
    )]));

    Ok(HashMap::new())
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use cairo_vm::{serde::deserialize_program::ApTracking, types::exec_scope::ExecutionScopes, vm::vm_core::VirtualMachine};

    use super::*;
    use crate::{define_segments, ids_data, vm};

    #[test]
    fn test_select_builtins_enter_scope() {
        let mut vm = vm!();
        // Set n_selected_builtins to 7
        vm.set_fp(1);
        define_segments!(vm, 2, [((1, 0), 7)]);
        let ids_data = ids_data![vars::N_SELECTED_BUILTINS];
        let n_selected_builtins = 7usize;

        let ap_tracking = ApTracking::default();
        let mut exec_scopes = ExecutionScopes::new();

        select_builtins_enter_scope(&mut vm, &mut exec_scopes, &ids_data, &ap_tracking).expect("Hint failed unexpectedly");

        // Check that we entered a new scope
        assert_eq!(exec_scopes.data.len(), 2);
        assert_eq!(exec_scopes.data[1].len(), 1);

        let n_selected_builtins_var: usize = exec_scopes.get(vars::N_SELECTED_BUILTINS).unwrap();

        assert_eq!(n_selected_builtins_var, n_selected_builtins);
    }
}
