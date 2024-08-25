use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg
};


entrypoint!(hello_world);

fn hello_world(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult{

    msg!("Hello world program");

    msg!("this is the program id {}", &program_id);


    // we iterate throught the accounts
    let accounts_iter = &mut accounts.iter();

    // get account info
    let account = next_account_info(accounts_iter)?;

    msg!("Payer's address is: {}", account.key);

    Ok(())
}