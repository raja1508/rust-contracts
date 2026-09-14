use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Counter {
    pub count : u32,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum InstructionData {
    Initialize,
    Increase(u32),
    Decrease(u32)
    
}


// #[cfg(custom_heap)]
entrypoint!(process_instructions); 

pub fn process_instructions(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {

    let mut iter = accounts.iter(); 
    let counter_account = next_account_info(&mut iter)?; 

    if !counter_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature); 
    }

    if counter_account.owner != program_id{
        return Err(ProgramError::IncorrectProgramId);
    }

    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData); 
    }

    
    let instruction = InstructionData::try_from_slice(&instruction_data)?; 

    
    let data = match instruction {
        InstructionData::Initialize => {
            let data = counter_account.data.borrow();
            let parse: Result<Counter, std::io::Error> = Counter::try_from_slice(&data); 

            if let Ok(_) = parse {
                return Err(ProgramError::AccountAlreadyInitialized)
            }

            let counter = Counter {
                count: 0,
            }; 
            counter

        },
        _ => {
            if counter_account.data_len() < std::mem::size_of::<u32>() {
                return Err(ProgramError::AccountDataTooSmall);
            }
            let data = counter_account.data.borrow();
            let mut counter : Counter = Counter::try_from_slice(&data)?; 
            process_counter(&mut counter, instruction)? 
        }
    }; 

    data.serialize(&mut *counter_account.data.borrow_mut())?;

    

    Ok(())
}

fn process_counter(counter: &mut Counter, instruction: InstructionData ) -> Result<Counter, ProgramError>{
    
    match instruction {
        InstructionData::Increase(value) => {
            let x = counter.count.checked_add(value)
            .ok_or(ProgramError::ArithmeticOverflow)?; 
            counter.count = x;
            return Ok(counter.clone()) 
        },
        InstructionData::Decrease(value) => {
            let x = counter.count.checked_sub(value)
            .ok_or(ProgramError::InvalidArgument)?; 
            counter.count = x; 
            return Ok(counter.clone())
        },
        _ => {
            return Err(ProgramError::InvalidInstructionData)
        }
    }
}


#[cfg(test)]
mod counter_tests;