use ic_cdk::export_candid;
use ic_cdk_macros::*;
use std::collections::HashMap;

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
fn buy_asset(asset_id: AssetId) -> String {
    let caller = ic_cdk::caller().to_text();
    MARKETPLACE.with(|marketplace| {
        let mut state = marketplace.borrow_mut();
        if let Some(price) = state.assets.remove(&asset_id) {
            // Simulate reverse gas payment
            let fee_wallet = "rwlgt-4zjif-aaaaa-aaaaa-cai"; // Replace with the actual wallet ID
            let fee_amount: u64 = 10; // Hardcoded transaction fee

            ic_cdk::print(format!(
                "Transaction fee of {} paid from wallet {}",
                fee_amount, fee_wallet
            ));

            format!("Asset '{}' purchased successfully for {} ICP by {}", asset_id, price, caller)
        } else {
            format!("Asset '{}' not found", asset_id)
        }
    })
}

export_candid!();  // Exports the Candid interface
