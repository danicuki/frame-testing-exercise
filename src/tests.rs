use crate::{
	mock::*,
	pallet::{self as pallet_template, *},
};
use frame_support::dispatch::PostDispatchInfo;
use frame_support::{pallet_prelude::*, testing_prelude::*};

fn root() -> RuntimeOrigin {
	RuntimeOrigin::root()
}


mod register_voter {
use sp_runtime::traits::BadOrigin;

	use super::*;
	
	#[test]
	fn works_only_if_root() {
		new_test_ext().execute_with(|| {
			let alice = RuntimeOrigin::signed(1);
			assert_noop!(TemplateModule::register_voter(alice, 1), BadOrigin);
			assert_ok!(TemplateModule::register_voter(root(), 1));
		});
	}

	#[test]
	fn cannot_register_too_many() {
		MyMaxVoters::set(2);
		new_test_ext().execute_with(|| {
			assert_ok!(TemplateModule::register_voter(root(), 1));
			assert_ok!(TemplateModule::register_voter(root(), 2));
			assert_noop!(
				TemplateModule::register_voter(root(), 3),
				Error::<Runtime>::TooManyVoters
			);
		});
	}
}

mod make_vote {

	use super::*;

	#[test]
	fn only_voters_can_vote() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				TemplateModule::make_vote(RuntimeOrigin::signed(1), Vote::Aye),
				Error::<Runtime>::NotVoter
			);
			assert_ok!(TemplateModule::register_voter(root(), 1));
			assert_ok!(TemplateModule::make_vote(RuntimeOrigin::signed(1), Vote::Aye));
		});
	}

	#[test]
	fn can_update_vote() {
		new_test_ext().execute_with(|| {
			assert_ok!(TemplateModule::register_voter(root(), 1));
			assert_ok!(TemplateModule::make_vote(RuntimeOrigin::signed(1), Vote::Aye));
			assert_ok!(TemplateModule::make_vote(RuntimeOrigin::signed(1), Vote::Nay));

			let votes = Votes::<Runtime>::get();
			assert_eq!(votes.len(), 1);
			assert_eq!(votes[0].vote, Vote::Nay);
			assert_eq!(votes[0].who, 1);
		});
	}
}

mod close_vote {
	use super::*;

	#[test]
	fn cannot_close_if_no_voter() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				TemplateModule::close_vote(RuntimeOrigin::signed(1)),
				Error::<Runtime>::NoVoters
			);
		});
	}

	#[test]
	fn cannot_close_if_not_enough_voted() {
		new_test_ext().execute_with(|| {
			assert_ok!(TemplateModule::register_voter(root(), 1));
			assert_noop!(
				TemplateModule::close_vote(RuntimeOrigin::signed(1)),
				Error::<Runtime>::NotComplete
			);
		});
	}

	#[test]
	fn can_close() {
		new_test_ext().execute_with(|| {
			assert_ok!(TemplateModule::register_voter(root(), 1));
			assert_ok!(TemplateModule::make_vote(RuntimeOrigin::signed(1), Vote::Aye));
			assert_ok!(TemplateModule::close_vote(RuntimeOrigin::signed(1)));

			System::assert_has_event(
				Event::<Runtime>::Outcome { aye: true }.into(),
			);
		});
	}

	#[test]
	fn close_even_will_fail() {
		
	}

	#[test]
	fn close_with_abstinence() {
		new_test_ext().execute_with(|| {
			assert_ok!(TemplateModule::register_voter(root(), 1));
			assert_ok!(TemplateModule::make_vote(RuntimeOrigin::signed(1), Vote::Abstain));
			assert_eq!(
				TemplateModule::close_vote(RuntimeOrigin::signed(1)),
				Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes })
			);
		});
	}
}
