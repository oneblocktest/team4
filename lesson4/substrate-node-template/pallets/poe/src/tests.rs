// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

//正常测试
#[test]
fn create_claim_works(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));

    })
}


//已创建测试
#[test]
fn create_claim_failed_when_claim_already_exist(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone());

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1),claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

//已太长测试
#[test]
fn create_claim_failed_when_claim_poof_too_long(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1,2,4,5,6,7,8,9];

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1),claim.clone()),
            Error::<Test>::ProofTooLong
        );
    })
}

//测试撤销存证
#[test]
fn revoke_claim_works(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()));
    
    })
}

//测试撤销无存证
#[test]
fn revoke_claim_claim_not_exist(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
       
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1),claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    
    })
}

//测试转移凭证
#[test]
fn transfer_claim_works(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone());
        //transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source)
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2));
    
    })
}
//测试转移存证非拥有者
#[test]
fn transfer_claim_not_claimowner(){
    new_test_ext().execute_with(||{
        let claim=vec![0,1];
        let _=PoeModule::create_claim(Origin::signed(1),claim.clone());
      
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(3),claim.clone(),2),
            Error::<Test>::NotClaimOwner
        );
    })
}
