use super::*;
use near_sdk::{
    test_utils::{accounts, VMContextBuilder},
    testing_env,
};

// Allows for modifying the environment of the mocked blockchain
fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    // Create a new context with the test account.
    let mut builder = VMContextBuilder::new();
    // Set the predecessor account ID for the subsequent calls.
    builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}

#[test]
fn test_patient_creation_success() {
    // Arrange
    // Create a new context with the test account.
    let mut context = get_context(accounts(1));
    // Initialize the mocked blockchain
    testing_env!(context.build());

    // Set the testing environment for the subsequent calls
    testing_env!(context.predecessor_account_id(accounts(1)).build());

    let mut contract = Contract::new();

    // Act
    // Call the add_patient method to create the new patient.
    contract.add_patient(
        "Jack Johnson".to_string(),
        "01/02/2023".to_string(),
        "Male".to_string(),
        "A+".to_string(),
    );

    // Assert
    // Assert that the patient was created successfully.
    assert!(contract.patients.contains_key(&accounts(1)));

    // Call get_patient method to get the patient information
    let patient = contract.get_patient();

    // Assert that the patient was created successfully.
    assert_eq!(patient.full_name(), "Jack Johnson".to_string());
    assert_eq!(patient.birthday(), "01/02/2023".to_string());
    assert_eq!(patient.gender(), "Male".to_string());
    assert_eq!(patient.blood_type(), "A+".to_string());

    // Assert that the patient has no records, allergies, immunizations, or procedures
    assert!(patient.records().is_empty());
    assert!(patient.allergies().is_empty());
    assert!(patient.immunizations().is_empty());
    assert!(patient.procedures().is_empty());
}

#[test]
#[should_panic(expected = "Patient already exists.")]
fn test_patient_creation_duplicate() {
    // Arrange
    // Create a new context with the test account.
    let mut context = get_context(accounts(1));
    // Initialize the mocked blockchain
    testing_env!(context.build());

    // Set the testing environment for the subsequent calls
    testing_env!(context.predecessor_account_id(accounts(1)).build());

    let mut contract = Contract::new();

    // Act
    // Call the add_patient method to create the new patient.
    contract.add_patient(
        "Jack Johnson".to_string(),
        "01/02/2023".to_string(),
        "Male".to_string(),
        "A+".to_string(),
    );

    // Assert
    // Try to add the same patient again, which should fail.
    contract.add_patient(
        "Jack Johnson".to_string(),
        "01/02/2023".to_string(),
        "Male".to_string(),
        "A+".to_string(),
    );
}

#[test]
#[should_panic(expected = "Patient not found.")]
fn test_get_patient_nonexistent() {
    // Arrange
    // Create a new context with the test account.
    let context = get_context(accounts(1));
    // Initialize the mocked blockchain
    testing_env!(context.build());

    let contract = Contract::new();

    // Act & Assert
    // Try to get a patient that doesn't exist, which should fail.
    contract.get_patient();
}

#[test]
fn test_update_patient_details() {
    // Arrange
    // Create a new context with the test account.
    let mut context = get_context(accounts(1));
    // Initialize the mocked blockchain
    testing_env!(context.build());

    // Set the testing environment for the subsequent calls
    testing_env!(context.predecessor_account_id(accounts(1)).build());

    let mut contract = Contract::new();

    // Act
    // Call the add_patient method to create the new patient.
    contract.add_patient(
        "John Doe".to_string(),
        "01/01/1990".to_string(),
        "Male".to_string(),
        "O+".to_string(),
    );

    // Update patient details
    contract.full_name("Jane Smith".to_string());
    contract.birthday("02/15/1985".to_string());
    contract.gender("Female".to_string());
    contract.blood_type("A-".to_string());

    // Assert
    // Call get_patient method to get the updated patient information
    let patient = contract.get_patient();

    // Assert that the patient details have been updated successfully.
    assert_eq!(patient.full_name(), "Jane Smith".to_string());
    assert_eq!(patient.birthday(), "02/15/1985".to_string());
    assert_eq!(patient.gender(), "Female".to_string());
    assert_eq!(patient.blood_type(), "A-".to_string());
}
