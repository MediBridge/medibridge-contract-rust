use super::*;
use near_sdk::{
    test_utils::{accounts, VMContextBuilder},
    testing_env,
};

// TODO: Add tests for the following:
// - [ ] Create a new patient
// - [ ] Update the name of a patient
// - [ ] Update the birthday of a patient
// - [ ] Update the blood type of a patient
// - [ ] Add a new allergy to a patient
// - [ ] Add a new immunization to a patient
// - [ ] Add a new procedure to a patient
// - [ ] Add a new record to a patient
// - [ ] Get a patient's information
// - [ ] Get a patient's allergies (for example)
// - [ ] Get all public records

// // Helper method to set up the contract for testing.
// fn setup_contract() -> (VMContextBuilder, Contract) {
//     // Create a new context with the test account.
//     let context = VMContextBuilder::new()
//         .predecessor_account_id(accounts(0))
//         .build();

//     testing_env!(context.predecessor_account_id(accounts(0)).build());

//     // Initialize the contract with the test account as the first patient.
//     let mut contract = Contract::new();

//     (context, contract)
// }

// // Test creating a new patient successfully.
// #[test]
// fn create_new_patient() {
//     // Set up the contract for testing.
//     let (mut context, mut contract) = setup_contract();

//     // Define the patient information.
//     let full_name = "John Doe".to_string();
//     let birthday = "1985-01-01".to_string();
//     let gender = "Male".to_string();
//     let blood_type = "O+".to_string();

//     // Call the add_patient method to create the new patient.
//     contract.add_patient(full_name, birthday, gender, blood_type);

//     // Assert that the patient was created successfully.
//     assert!(contract.patients.contains_key(&accounts(0)));
// }
