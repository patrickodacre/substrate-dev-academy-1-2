use crate::Kitty;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn can_create_kitty() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::create_kitty(Origin::signed(1)));
        let kitty = TemplateModule::owner_to_kitties(&1).unwrap();
        let kitty1 = (&kitty[0]).as_ref().unwrap();
        println!("{:?}", kitty1);
        assert_eq!(kitty1.id, 1);

        assert_ok!(TemplateModule::create_kitty(Origin::signed(1)));
        let kitty = TemplateModule::owner_to_kitties(&1).unwrap();
        let kitty2 = (&kitty[1]).as_ref().unwrap();
        println!("{:?}", kitty2);
        assert_eq!(kitty2.id, 2);

        assert_ne!(kitty1.dna, kitty2.dna);
    });
}

#[test]
fn can_get_kitty_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::create_kitty(Origin::signed(1)));

        let kitties = TemplateModule::owner_to_kitties(&1).expect("no kitties");

        let kitty = (&kitties[0]).as_ref().unwrap();
        assert_eq!(kitty.id, 1);

        let owner = TemplateModule::kitty_to_owner(kitty.id).expect("no owner");

        println!("owner :: {:?}", owner);
        assert_eq!(owner, 1);
    });
}

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        // assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
        // Read pallet storage and assert an expected result.
        // assert_eq!(TemplateModule::something(), Some(42));
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        // assert_noop!(
        // TemplateModule::cause_error(Origin::signed(1)),
        // Error::<Test>::NoneValue
        // );
    });
}
