use cumulus_primitives_core::ParaId;
use frame_support::{
    sp_runtime::traits::{AccountIdConversion, IdentifyAccount, Verify},
    sp_runtime::MultiSignature,
    PalletId,
};
use std::convert::TryInto;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Signature = MultiSignature;

fn main() {
    match std::env::args()
        .nth(1)
        .expect("expected command `pallet` or `paraid` ")
        .as_str()
    {
        "pallet" | "palletid" => {
            let pallet = std::env::args().nth(2).unwrap();
            let id = PalletId(
                pallet
                    .as_bytes()
                    .try_into()
                    .expect("Expected 8-byte pallet id"),
            );
            let account: AccountId = id.into_account();
            println!("PalletId({}): {:?}", pallet, account.to_string());
        }
        "paraid" => {
            let para = std::env::args().nth(2).unwrap();
            let id = ParaId::new(para.parse().expect("failed to parse para id"));
            let account: AccountId = id.into_account();
            println!("ParaId({}): {:?}", para, account.to_string());
        }
        "help" | "--help" => {
            println!(
                "Usage:
            substrate-id-to-account paraid 200
            substrate-id-to-account pallet 12345678
            "
            )
        }
        s => eprintln!("Unexpected command {}", s),
    };
}
