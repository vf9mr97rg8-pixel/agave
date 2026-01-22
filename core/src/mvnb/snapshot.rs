use solana_runtime::bank::Bank;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct WalletSnapshot {
    pub pubkey: Pubkey,
    pub slot: u64,
    pub lamports: u64,
    pub spl_tokens: Vec<SplTokenBalance>,
}

#[derive(Debug, Clone)]
pub struct SplTokenBalance {
    pub mint: Pubkey,
    pub amount: u64,
    pub decimals: u8,
}

/// INTERNAL ONLY.
/// No RPC. No IO. No authority.
pub fn snapshot_wallet(bank: &Bank, wallet: &Pubkey) -> WalletSnapshot {
    let lamports = bank.get_balance(wallet);
    let slot = bank.slot();

    WalletSnapshot {
        pubkey: *wallet,
        slot,
        lamports,
        spl_tokens: Vec::new(), // SPL decoding comes next
    }
}
