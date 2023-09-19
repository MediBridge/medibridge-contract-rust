use crate::types::{
    Allergy, FamilyMedicalRecord, Immunization, MedicalRecord, Patient, Procedure, PublicRecord,
    RecordType, Treatment,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, Vector},
    env, log, near_bindgen, require, AccountId, PanicOnDefault, Promise,
};

#[cfg(test)]
mod tests;
mod types;

// Define the contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    patients: LookupMap<AccountId, Patient>,
    public_records: Vector<PublicRecord>,
}

#[near_bindgen]
impl Contract {
    /// Initialize the contract

    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        log!("Contract done");
        Self {
            patients: LookupMap::new(b"patients".to_vec()),
            public_records: Vector::new(b"public_records".to_vec()),
        }
    }

    /// Add a new patient to the contract with the provided
    /// medical information and personal details
    pub fn add_patient(
        &mut self,
        full_name: String,
        birthday: String,
        gender: String,
        blood_type: String,
    ) {
        let account_id = env::predecessor_account_id();

        require!(
            !self.patients.contains_key(&account_id),
            "Patient already exists."
        );

        let patient = Patient::new(
            full_name,
            birthday,
            gender,
            blood_type,
            vec![],
            vec![],
            vec![],
            vec![],
        );

        self.patients.insert(&account_id, &patient);
        log!("Registered patient successfully. ID: {}", account_id);
    }

    /// Get patient information for the calling account
    pub fn get_patient(&self) -> Patient {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );

        // We can safely unwrap here because we know the key exists
        self.patients.get(&account_id).unwrap()
    }

    // WORKAROUND SOLUTION TO FETCH PATIENT INFORMATION
    pub fn get_patient_workaround(&self, account_id: &AccountId) -> Patient {
        // let account_id = env::predecessor_account_id();
        require!(self.patients.contains_key(account_id), "Patient not found.");

        // We can safely unwrap here because we know the key exists
        self.patients.get(account_id).unwrap()
    }

    pub fn my_account(&self) -> AccountId {
        let account_id: AccountId = env::current_account_id();
        log!("Caller Account ID: {}", account_id);
        account_id
    }

    /// Get all public records stored on-chain.
    pub fn get_all_public_records(&self) -> Vec<PublicRecord> {
        self.public_records.to_vec()
    }

    /// Remove the patient and their records from the contract storage.
    pub fn remove_patient(&mut self) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );
        log!("Removing patient with ID: {}", account_id);

        // Remove patient from the contract storage
        self.patients.remove(&account_id);

        log!("Patient with ID: {} has been removed.", account_id);
    }

    /// Update the name of the patient
    pub fn full_name(&mut self, full_name: String) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );
        log!("Updating name for patient with ID: {}", account_id);
        log!("New full name: {}", full_name);

        let mut patient = self.patients.get(&account_id).expect("Patient not found.");

        // Update the patient's full name
        patient.update_full_name(full_name);

        // Update the patient in the contract storage
        self.patients.insert(&account_id, &patient);
        log!("Updated name for patient with ID: {}", account_id);
    }

    /// Update the birthday of the patient
    pub fn birthday(&mut self, birthday: String) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );
        log!("Updating birthday for patient with ID: {}", account_id);
        log!("New birthday: {}", birthday);

        let mut patient = self.patients.get(&account_id).expect("Patient not found.");

        // Update the patient's birthday
        patient.update_birthday(birthday);

        // Update the patient in the contract storage
        self.patients.insert(&account_id, &patient);
        log!("Updated birthday for patient with ID: {}", account_id);
    }

    /// Update the gender of the patient
    pub fn gender(&mut self, gender: String) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );
        log!("Updating gender for patient with ID: {}", account_id);
        log!("New gender: {}", gender);

        let mut patient = self.patients.get(&account_id).expect("Patient not found.");

        // Update the patient's gender
        patient.update_gender(gender);

        // Update the patient in the contract storage
        self.patients.insert(&account_id, &patient);
        log!("Updated gender for patient with ID: {}", account_id);
    }

    /// Update the blood type of the patient
    pub fn blood_type(&mut self, blood_type: String) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );
        log!("Updating blood type for patient with ID: {}", account_id);
        log!("New blood type: {}", blood_type);

        let mut patient = self.patients.get(&account_id).expect("Patient not found.");

        // Update the patient's blood type
        patient.update_blood_type(blood_type);

        // Update the patient in the contract storage
        self.patients.insert(&account_id, &patient);
        log!("Updated blood type for patient with ID: {}", account_id);
    }

    /// Add a new allergy for the calling account
    pub fn add_allergy(&mut self, allergen: String, severity: String) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );
        log!("Adding allergy for patient with ID: {}", account_id);
        log!("Allergen: {}", allergen);
        log!("Severity: {}", severity);

        let mut patient = self.patients.get(&account_id).expect("Patient not found.");

        // Create a new allergy
        let allergy = Allergy::new(allergen, severity);

        // Update the patient's allergies vector
        patient.add_allergy(allergy);

        // Update the patient in the contract storage
        self.patients.insert(&account_id, &patient);
        log!("Added allergy for patient with ID: {}", account_id);
    }

    /// Add a new immunization for the calling account
    pub fn add_immunization(&mut self, name: String, date: String) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );
        log!("Adding immunization for patient with ID: {}", account_id);
        log!("Name: {}", name);
        log!("Date: {}", date);

        let mut patient = self.patients.get(&account_id).expect("Patient not found.");

        // Create a new immunization
        let immunization = Immunization::new(name, date);

        // Update the patient's immunizations vector
        patient.add_immunization(immunization);

        // Update the patient in the contract storage
        self.patients.insert(&account_id, &patient);
        log!("Added immunization for patient with ID: {}", account_id);
    }

    /// Add a new procedure for the calling account
    pub fn add_procedure(&mut self, name: String, date: String, description: String) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );
        log!("Adding procedure for patient with ID: {}", account_id);
        log!("Name: {}", name);
        log!("Date: {}", date);
        log!("Description: {}", description);

        let mut patient = self.patients.get(&account_id).expect("Patient not found.");

        // Create a new procedure
        let procedure = Procedure::new(name, date, description);

        // Update the patient's procedures vector
        patient.add_procedure(procedure);

        // Update the patient in the contract storage
        self.patients.insert(&account_id, &patient);
        log!("Added procedure for patient with ID: {}", account_id);
    }

    /// Add a new medical record for the calling account
    pub fn add_medical_record(
        &mut self,
        id: u64,
        condition: String,
        record_data: String,
        date: String,
    ) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );
        log!("Adding medical record for patient with ID: {}", account_id);
        log!("ID: {}", id);
        log!("Condition: {}", condition);
        log!("Record Data: {}", record_data);
        log!("Date: {}", date);

        let mut patient = self.patients.get(&account_id).expect("Patient not found.");

        // Create a new medical record
        let medical_record = MedicalRecord::new(id, condition, record_data, date);

        // Update the patient's medical records vector
        patient.add_medical_record(medical_record);

        // Update the patient in the contract storage
        self.patients.insert(&account_id, &patient);
        log!("Added medical record for patient with ID: {}", account_id);
    }

    /// Add a new PUBLIC medical record
    pub fn add_public_medical_record(
        &mut self,
        id: u64,
        condition: String,
        record_data: String,
        date: String,
    ) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Only registered patients can add public medical records."
        );
        log!("ID: {}", id);
        log!("Condition: {}", condition);
        log!("Record Data: {}", record_data);
        log!("Date: {}", date);

        // Retrieve patient information.
        // We only need this to get the patient's birthday and gender
        // (required for the PublicRecord struct)
        let patient = self.patients.get(&account_id).expect("Patient not found.");

        // Create a new medical record
        let medical_record = MedicalRecord::new(id, condition, record_data, date);

        // Add the medical record to the public records
        let public_medical_record = PublicRecord::new(
            RecordType::MedicalRecord(medical_record.clone()),
            patient.birthday(),
            patient.gender(),
        );
        self.public_records.push(&public_medical_record);

        log!("Added a new public medical record");
    }

    /// Add a new treatment for the calling account
    #[allow(clippy::too_many_arguments)]
    pub fn add_treatment(
        &mut self,
        id: u64,
        treatment: String,
        dosage: String,
        start_date: String,
        finish_date: String,
        reason: Option<String>,
        public: bool,
    ) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );
        log!("Adding treatment for patient with ID: {}", account_id);
        log!("ID: {}", id);
        log!("Treatment: {}", treatment);
        log!("Dosage: {}", dosage);
        log!("Start Date: {}", start_date);
        log!("Finish Date: {}", finish_date);
        if let Some(reason) = &reason {
            log!("Reason: {}", reason);
        } else {
            log!("Reason: None");
        }

        let mut patient = self.patients.get(&account_id).expect("Patient not found.");

        // Create a new treatment
        let treatment = Treatment::new(id, treatment, dosage, start_date, finish_date, reason);

        // Add the treatment to the public records if public is true
        if public {
            let public_treatment = PublicRecord::new(
                RecordType::Treatment(treatment.clone()),
                patient.birthday(),
                patient.gender(),
            );
            self.public_records.push(&public_treatment);
        }

        // Update the patient's treatments vector
        patient.add_treatment(treatment);

        // Update the patient in the contract storage
        self.patients.insert(&account_id, &patient);
        log!("Added treatment for patient with ID: {}", account_id);
    }

    /// Add a new family medical record for the calling account
    pub fn add_family_medical_record(
        &mut self,
        id: u64,
        condition: String,
        relation_to_patient: String,
        public: bool,
    ) {
        let account_id = env::predecessor_account_id();
        require!(
            self.patients.contains_key(&account_id),
            "Patient not found."
        );
        log!(
            "Adding family medical record for patient with ID: {}",
            account_id
        );
        log!("ID: {}", id);
        log!("Condition: {}", condition);
        log!("Relation to Patient: {}", relation_to_patient);

        let mut patient = self.patients.get(&account_id).expect("Patient not found.");

        // Create a new family medical record
        let family_medical_record = FamilyMedicalRecord::new(id, condition, relation_to_patient);

        // Add the family medical record to the public records if public is true
        if public {
            let public_family_medical_record = PublicRecord::new(
                RecordType::FamilyMedicalRecord(family_medical_record.clone()),
                patient.birthday(),
                patient.gender(),
            );
            self.public_records.push(&public_family_medical_record);
        }

        // Update the patient's family medical records vector
        patient.add_family_medical_record(family_medical_record);

        // Update the patient in the contract storage
        self.patients.insert(&account_id, &patient);
        log!(
            "Added family medical record for patient with ID: {}",
            account_id
        );
    }

    /// Transfer the attached deposit to the contract developers, dividing it equally.
    pub fn transfer_to_developers(&mut self) {
        let attached_deposit = env::attached_deposit();

        // The developers of the contract.
        // FIXME: Replace with the actual developers of the contract.
        let developers: Vec<AccountId> = vec![
            "tarek.near".parse().unwrap(),
            "prakhar.near".parse().unwrap(),
            "yulduz.near".parse().unwrap(),
        ];

        // Calculate the split amount for each developer.
        let split_amount = attached_deposit / (developers.len() as u128);

        // Transfer the split amount to each developer.
        for developer_account_id in developers {
            Promise::new(developer_account_id.clone()).transfer(split_amount);
        }
    }
}
