use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("This is an error message clients will automatically display")]
    InvalidAuthorityKey,
    #[msg("InvalidMintWalletAdress")]
    InvalidMintWalletAdress
}
