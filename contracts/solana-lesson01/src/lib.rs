// Import necessary modules from the solana_program crate. This includes types and functions for account information,
// entry points to the program, logging messages, and public keys.
use solana_program::{
    account_info::AccountInfo, // Used for handling account information.
    entrypoint,                // Macro to mark the entry point to the program.
    entrypoint::ProgramResult, // Type used for returning success or error from the entry point.
    msg,                       // Macro for logging messages.
    pubkey::Pubkey,            // Type representing a public key in Solana.
};

// Define the entry point of the program using the `entrypoint!` macro. This macro expands into the necessary boilerplate
// to interface with Solana's runtime. The `process_instruction` function is specified as the entry point here.
entrypoint!(process_instruction);

// The main function that gets called when the program is executed. This function must match the signature expected by
// the Solana runtime:
// - `program_id` is the public key of the deployed program itself.
// - `accounts` is a slice of AccountInfo objects, representing all the accounts that might be read from or written to.
// - `instruction_data` is a slice of bytes containing the data passed to the program by the transaction that invoked it.
pub fn process_instruction(
    program_id: &Pubkey,      // The public key of the program being executed.
    accounts: &[AccountInfo], // A list of accounts involved in the transaction.
    instruction_data: &[u8],  // Data passed to the program by the instruction.
) -> ProgramResult {
    // Log a simple message to the program's output. This is useful for debugging and understanding program flow.
    // The message will be visible in the transaction log.
    msg!("Hello Bootstrap!");
    msg!("Program ID: {:?}", program_id);

    // Return `Ok(())` to indicate that the program executed successfully. If an error occurred, you would return
    // an appropriate error wrapped in `Err(...)`.
    Ok(())
}
