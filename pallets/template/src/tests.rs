use crate::Kitty;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn can_create_kitty() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::create_kitty(Origin::signed(1)));
        let number_of_kitties = TemplateModule::number_of_kitties().unwrap();
        assert_eq!(number_of_kitties, 1);

        let kitties = TemplateModule::owner_to_kitties(&1).unwrap();
        let kitty_id = (&kitties.get(0)).unwrap();
        println!("{:?}", kitty_id);
        assert_eq!(kitty_id, &1);

        assert_ok!(TemplateModule::create_kitty(Origin::signed(1)));
        let number_of_kitties = TemplateModule::number_of_kitties().unwrap();
        assert_eq!(number_of_kitties, 2);

        let kitties = TemplateModule::owner_to_kitties(&1).unwrap();
        let kitty_id = (&kitties.get(1)).unwrap();
        println!("{:?}", kitty_id);
        assert_eq!(kitty_id, &2);
    });
}

#[test]
fn can_get_kitty_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::create_kitty(Origin::signed(1)));

        let kitty_id: u64 = 1;
        let expected_owner_id: u64 = 1;
        let owner: u64 = TemplateModule::kitty_to_owner(&kitty_id).unwrap();
        assert_eq!(owner, expected_owner_id);
    });
}

#[test]
fn can_get_owners_kitties() {
    new_test_ext().execute_with(|| {
        let owner_id: u64 = 1;

        assert_ok!(TemplateModule::create_kitty(Origin::signed(owner_id)));
        let kitties = TemplateModule::owner_to_kitties(&owner_id).unwrap();
        assert_eq!(kitties.len(), 1);

        assert_ok!(TemplateModule::create_kitty(Origin::signed(owner_id)));
        let kitties = TemplateModule::owner_to_kitties(&owner_id).unwrap();
        assert_eq!(kitties.len(), 2);
    });
}

#[test]
fn can_get_number_of_kitties() {
    new_test_ext().execute_with(|| {
        let owner_id = 1;
        assert_ok!(TemplateModule::create_kitty(Origin::signed(owner_id)));
        assert_ok!(TemplateModule::create_kitty(Origin::signed(owner_id)));
        assert_ok!(TemplateModule::create_kitty(Origin::signed(owner_id)));

        let number_of_kitties = TemplateModule::number_of_kitties().unwrap();

        assert_eq!(number_of_kitties, 3);
    });
}

#[test]
fn owner_can_have_zero_kitties() {
    new_test_ext().execute_with(|| {
        // can test using a default empty vec
        // let kitties = TemplateModule::owner_to_kitties(&1).unwrap_or(Vec::<Option<Kitty>>::new());
        // println!("no kitties :: {:?}", kitties);

        // or do something likes this::
        match TemplateModule::owner_to_kitties(&1) {
            Some(_kitties) => {
                panic!("There should not be any kitties");
            }
            None => {}
        }
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
