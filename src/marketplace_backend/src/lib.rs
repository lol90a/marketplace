use ic_cdk::export_candid;
use ic_cdk_macros::*;
use ic_cdk::api::call::call;
use ic_principal::Principal;
use std::collections::HashMap;
use candid::candid_method;

type AssetId = String;
type UserId = String;

#[derive(Default)]
struct Marketplace {
    assets: HashMap<AssetId, u64>,   // Asset ID -> Price
    balances: HashMap<UserId, u64>,  // User -> Balance
}

// Global mutable state for Marketplace
thread_local! {
    static MARKETPLACE: std::cell::RefCell<Marketplace> = std::cell::RefCell::new(Marketplace::default());
}

// Initialization function (no return value allowed)
#[init]
fn init() {
    // Initialization logic, if any
    ic_cdk::print("Marketplace canister initialized successfully!");
}

// Function for listing an asset
#[update]
#[candid_method]
fn list_asset(asset_id: AssetId, price: u64) -> String {
    let caller = ic_cdk::caller().to_text();
    MARKETPLACE.with(|marketplace| {
        let mut state = marketplace.borrow_mut();
        state.assets.insert(asset_id.clone(), price);
        format!("Asset '{}' listed at price {} by {}", asset_id, price, caller)
    })
}

// Function for buying an asset
#[update]
#[candid_method]
async fn buy_asset(asset_id: AssetId) -> String {
    let caller = ic_cdk::caller().to_text();
    let fee_wallet = Principal::from_text("b77ix-eeaaa-aaaaa-qaada-cai").unwrap(); // Wallet to pay the fee from
    let fee_amount: u64 = 10; // Transaction fee

    // Handling marketplace state
    let price = MARKETPLACE.with(|marketplace| {
        let mut state = marketplace.borrow_mut();
        state.assets.remove(&asset_id)
    });

    match price {
        Some(price) => {
            // Call the fee wallet canister to pay the fee
            let fee_payment = call::<(u64,), ()>(  // Specify the type as a tuple (u64,) for arguments and () for the return type
                fee_wallet,
                "pay_fee", // Replace with the actual function to pay fee in the wallet canister
                (fee_amount,)
            );

            // Await the fee payment
            match fee_payment.await {
                Ok(_) => {
                    // Successfully paid fee
                    ic_cdk::print(format!(
                        "Transaction fee of {} ICP paid from wallet {}",
                        fee_amount, fee_wallet
                    ));
                    format!("Asset '{}' purchased successfully for {} ICP by {}", asset_id, price, caller)
                },
                Err(_) => {
                    // Failed to pay fee
                    format!("Failed to transfer fee.")
                },
            }
        },
        None => {
            // In case the asset isn't found
            format!("Asset '{}' not found", asset_id)
        }
    }
}

export_candid!();  // Exports the Candid interface for the canister
