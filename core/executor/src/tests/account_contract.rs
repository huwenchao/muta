use std::cell::RefCell;
use std::rc::Rc;

use protocol::traits::executor::contract::AccountContract;
use protocol::types::{Address, AssetID, Balance, CarryingAsset, Fee, UserAddress};

use crate::native_contract::NativeAccountContract;
use crate::tests::{create_state_adapter, mock_invoke_context};

#[test]
fn test_account_contract() {
    let state = Rc::new(RefCell::new(create_state_adapter()));
    let mut account = NativeAccountContract::new(state);

    let asset =
        AssetID::from_hex("0000000000000000000000000000000000000000000000000000000000000003")
            .unwrap();
    let fee_asset =
        AssetID::from_hex("0000000000000000000000000000000000000000000000000000000000000004")
            .unwrap();
    let user1 = UserAddress::from_hex("100000000000000000000000000000000000000001").unwrap();
    let user1_addr = Address::User(user1.clone());
    let user2 = Address::from_hex("100000000000000000000000000000000000000002").unwrap();
    account
        .add_balance(&asset, &user1_addr, 10000u64.into())
        .unwrap();

    let cycles_used = Fee {
        asset_id: fee_asset.clone(),
        cycle:    0,
    };
    let cycles_limit = Fee {
        asset_id: fee_asset.clone(),
        cycle:    1_000_000,
    };
    let carrying_asset = CarryingAsset {
        asset_id: asset.clone(),
        amount:   1000u64.into(),
    };
    let ctx = mock_invoke_context(
        user1.clone(),
        Some(carrying_asset),
        cycles_used,
        cycles_limit,
    );
    account.transfer(Rc::clone(&ctx), &user2).unwrap();
    let user1_balance = account.get_balance(&asset, &user1_addr).unwrap();
    assert_eq!(user1_balance, Balance::from(9000u64));
    let user2_balance = account.get_balance(&asset, &user2).unwrap();
    assert_eq!(user2_balance, Balance::from(1000u64));
}
