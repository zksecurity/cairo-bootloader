use std::{collections::HashMap, path::PathBuf};

use cairo_vm::{
    types::{errors::program_errors::ProgramError, exec_scope::ExecutionScopes, program::Program},
    vm::runners::cairo_pie::CairoPie,
};

use crate::hints::{
    types::{CairoPieTask, RunProgramTask, TaskSpec},
    BootloaderInput, BOOTLOADER_INPUT,
};

#[derive(thiserror_no_std::Error, Debug)]
pub enum BootloaderTaskError {
    #[error("Failed to read program: {0}")]
    Program(#[from] ProgramError),

    #[error("Failed to read PIE: {0}")]
    Pie(#[from] std::io::Error),
}

pub fn make_bootloader_tasks(
    programs: Option<&[PathBuf]>,
    program_inputs: Option<&[HashMap<String, serde_json::Value>]>,
    pies: Option<&[PathBuf]>,
) -> Result<Vec<TaskSpec>, BootloaderTaskError> {
    let mut tasks: Vec<TaskSpec> = Vec::new();
    if let (Some(programs), Some(program_inputs)) = (programs, program_inputs) {
        assert_eq!(
            programs.len(),
            program_inputs.len(),
            "The length of programs and program_inputs must be equal"
        );

        programs
            .iter()
            .zip(program_inputs.iter())
            .try_for_each(|(program_file, program_input)| -> Result<(), BootloaderTaskError> {
                let program = Program::from_file(program_file, Some("main")).map_err(BootloaderTaskError::Program)?;
                tasks.push(TaskSpec::RunProgram(RunProgramTask {
                    program,
                    program_input: program_input.clone(),
                    use_poseidon: false,
                }));
                Ok(())
            })?;
    }

    if let Some(pies) = pies {
        pies.iter().try_for_each(|pie| -> Result<(), BootloaderTaskError> {
            let cairo_pie = CairoPie::read_zip_file(pie).map_err(BootloaderTaskError::Pie)?;
            tasks.push(TaskSpec::CairoPieTask(CairoPieTask {
                cairo_pie,
                use_poseidon: false,
            }));
            Ok(())
        })?;
    }

    Ok(tasks)
}

/// Inserts the bootloader input in the execution scopes.
pub fn insert_bootloader_input(exec_scopes: &mut ExecutionScopes, bootloader_input: BootloaderInput) {
    exec_scopes.insert_value(BOOTLOADER_INPUT, bootloader_input);
}
