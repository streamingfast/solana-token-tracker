use crate::{constants, TokenParams};
use crate::pb::solana_token_tracker::types::v1::{
    Burn, InitializedAccount, Mint, Output, Transfer,
};
use std::ops::Div;
use substreams::errors::Error;

use substreams_solana::pb::sf::solana::r#type::v1::{
    CompiledInstruction, TokenBalance, TransactionStatusMeta,
};

use substreams_solana_program_instructions::{
    token_instruction_2022::TokenInstruction
};

pub fn process_compiled_instruction(
    output: &mut Output,
    timestamp: i64,
    trx_hash: &String,
    meta: &TransactionStatusMeta,
    inst_index: u32,
    inst: &CompiledInstruction,
    accounts: &Vec<String>,
    parameters: &TokenParams
) {
    let instruction_program_account = &accounts[inst.program_id_index as usize];

    if instruction_program_account == constants::TOKEN_PROGRAM {
        match process_token_instruction(trx_hash, timestamp, &inst.data, &inst.accounts, meta, accounts, output, parameters) {
            Err(err) => {
                panic!(
                    "trx_hash {} top level transaction without inner instructions: {}",
                    trx_hash, err
                );
            }
            Ok(()) => {}
        }

    }

    process_inner_instructions(output, inst_index, meta, accounts, trx_hash, timestamp, parameters);
}


pub fn process_inner_instructions(
    output: &mut Output,
    instruction_index: u32,
    meta: &TransactionStatusMeta,
    accounts: &Vec<String>,
    trx_hash: &String,
    timestamp: i64,
    parameters: &TokenParams,
) {
    meta.inner_instructions
        .iter()
        .filter(|inst| inst.index == instruction_index)
        .for_each(|inst| {
            inst.instructions
                .iter()
                .filter(|&inner_instruction| {
                    let instruction_program_account = &accounts[inner_instruction.program_id_index as usize];
                    instruction_program_account == constants::TOKEN_PROGRAM
                })
                .for_each(|inner_instruction| {
                    match process_token_instruction(
                        trx_hash,
                        timestamp,
                        &inner_instruction.data,
                        &inner_instruction.accounts,
                        meta,
                        accounts,
                        output,
                        parameters
                    ) {
                        Err(err) => {
                            panic!("trx_hash {} filtering inner instructions: {}", trx_hash, err)
                        }
                        Ok(()) => {}
                    }
                })
        });
}

fn process_token_instruction(
    trx_hash: &String,
    timestamp: i64,
    data: &Vec<u8>,
    inst_accounts: &Vec<u8>,
    meta: &TransactionStatusMeta,
    accounts: &Vec<String>,
    output: &mut Output,
    parameters: &TokenParams,
) -> Result<(),Error> {
    match TokenInstruction::unpack(&data) {
        Err(err) => {
            substreams::log::info!("unpacking token instruction {:?}", err);
            return Err(anyhow::anyhow!("unpacking token instruction: {}", err));
        }
        Ok(instruction) => match instruction {
            TokenInstruction::Transfer { amount: amt }  => {
                let authority = &accounts[inst_accounts[2] as usize];
                if is_token_transfer(&meta.pre_token_balances, &authority, &parameters.token_contract) {
                    let source = &accounts[inst_accounts[0] as usize];
                    let destination = &accounts[inst_accounts[1] as usize];
                    output.transfers.push(Transfer {
                        trx_hash: trx_hash.to_owned(),
                        timestamp,
                        from: source.to_owned(),
                        to: destination.to_owned(),
                        amount: amount_to_decimals(amt as f64, parameters.token_decimals as f64),
                    });
                    return Ok(());
                }
            }
             TokenInstruction::TransferChecked { amount: amt, .. } => {
                substreams::log::println("transfer");
                let mint = &accounts[inst_accounts[1] as usize];
                if is_token(mint, &parameters.token_contract) {
                    let source = &accounts[inst_accounts[0] as usize];
                    let destination = &accounts[inst_accounts[2] as usize];
                    output.transfers.push(Transfer {
                        trx_hash: trx_hash.to_owned(),
                        timestamp,
                        from: source.to_owned(),
                        to: destination.to_owned(),
                        amount: amount_to_decimals(amt as f64, parameters.token_decimals as f64),
                    });
                    return Ok(());
                }
            }
            TokenInstruction::MintTo { amount: amt } | TokenInstruction::MintToChecked { amount: amt, .. } => {
                let mint = fetch_account_to(&accounts, inst_accounts[0]);
                if mint.ne(&parameters.token_contract) {
                    return Ok(());
                }

                let account_to = fetch_account_to(&accounts, inst_accounts[1]);
                output.mints.push(Mint {
                    trx_hash: trx_hash.to_owned(),
                    timestamp,
                    to: account_to,
                    amount: amount_to_decimals(amt as f64, parameters.token_decimals as f64),
                });
                return Ok(());
            }
            TokenInstruction::Burn { amount: amt } | TokenInstruction::BurnChecked { amount: amt, .. } => {
                let mint = fetch_account_to(&accounts, inst_accounts[1]);
                if mint.ne(&parameters.token_contract) {
                    return Ok(());
                }

                let account_from = fetch_account_to(&accounts, inst_accounts[0]);
                output.burns.push(Burn {
                    trx_hash: trx_hash.to_owned(),
                    timestamp,
                    from: account_from,
                    amount: amount_to_decimals(amt as f64, parameters.token_decimals as f64),
                });
                return Ok(());
            }
            TokenInstruction::InitializeAccount {} => {
                let mint = fetch_account_to(&accounts, inst_accounts[1]);
                if mint.ne(&parameters.token_contract) {
                    return Ok(());
                }

                let account = fetch_account_to(&accounts, inst_accounts[0]);
                let owner = fetch_account_to(&accounts, inst_accounts[2]);
                output.initialized_account.push(InitializedAccount {
                    trx_hash: trx_hash.to_owned(),
                    account,
                    mint,
                    owner,
                });
                return Ok(())
            }
            TokenInstruction::InitializeAccount2 { owner: ow } | TokenInstruction::InitializeAccount3 { owner: ow } => {
                let mint = fetch_account_to(&accounts, inst_accounts[1]);
                if mint.ne(&parameters.token_contract) {
                    return Ok(());
                }

                let account = fetch_account_to(&accounts, inst_accounts[0]);
                output.initialized_account.push(InitializedAccount {
                    trx_hash: trx_hash.to_owned(),
                    account,
                    mint,
                    owner: bs58::encode(ow).into_string(),
                });
                return Ok(());
            }
            _ => {}
        },
    }

    return Ok(());
}

fn amount_to_decimals(amount: f64, decimal: f64) -> f64 {
    let base: f64 = 10.0;
    return amount.div(&(base.powf(decimal)));
}

fn fetch_account_to(account_keys: &Vec<String>, position: u8) -> String {
    // Instruction account will contain the list of accounts to fetch in the accounts list
    // inst account pos 0 -> mint_info
    // inst account pos 1 -> destination_account_info
    // inst account pos 2 -> owner_info
    return account_keys[position as usize].to_owned();
}

fn is_token(account: &String, contract_address: &String) -> bool{
    return account.eq(contract_address)
}

fn is_token_transfer(pre_token_balances: &Vec<TokenBalance>, account: &String, contract_address: &String) -> bool {
    for token_balance in pre_token_balances.iter() {
        if token_balance.owner.eq(account) && token_balance.mint.eq(contract_address) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use crate::utils::amount_to_decimals;

    #[test]
    pub fn test_amount_to_decimals() {
        let amount = 4983184141.0;
        let expected = 4.983184141;

        let actual = amount_to_decimals(amount, 9 as f64);
        println!("expected {:?} actual {:?}", expected, actual);
        assert_eq!(expected, actual)
    }
}
