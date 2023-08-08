use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

/// Struct representing a previous treatment received by the patient
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Treatment {
    id: u64,
    treatment: String,
    dosage: String,
    start_date: String,
    finish_date: String,
    reason: Option<String>,
}

impl Treatment {
    /// Create a new treatment with the given information
    pub fn new(
        id: u64,
        treatment: String,
        dosage: String,
        start_date: String,
        finish_date: String,
        reason: Option<String>,
    ) -> Self {
        Self {
            id,
            treatment,
            dosage,
            start_date,
            finish_date,
            reason,
        }
    }
}

/// Struct representing a medical condition in the patient's family medical history
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FamilyMedicalRecord {
    id: u64,
    condition: String,
    relation_to_patient: String,
}

impl FamilyMedicalRecord {
    /// Create a new family medical record with the given information
    pub fn new(id: u64, condition: String, relation_to_patient: String) -> Self {
        Self {
            id,
            condition,
            relation_to_patient,
        }
    }
}

/// Struct representing a personal medical record
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MedicalRecord {
    id: u64,
    condition: String,
    record_data: String,
    date: String,
}

impl MedicalRecord {
    /// Create a new medical record with the given information
    pub fn new(id: u64, condition: String, record_data: String, date: String) -> Self {
        Self {
            id,
            condition,
            record_data,
            date,
        }
    }
}

// Enum representing the different types of records a patient can have
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum RecordType {
    MedicalRecord(MedicalRecord),
    Treatment(Treatment),
    FamilyMedicalRecord(FamilyMedicalRecord),
}

// Generic struct to hold any type of record and additional fields
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PublicRecord {
    record_type: RecordType,
    birthday: String,
    gender: String,
}

impl PublicRecord {
    /// Create a new public record with the given information
    pub fn new(record_type: RecordType, birthday: String, gender: String) -> Self {
        Self {
            record_type,
            birthday,
            gender,
        }
    }
}

// Struct representing an Allergy
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Allergy {
    allergen: String,
    severity: String,
}

impl Allergy {
    /// Create a new allergy with the given information
    pub fn new(allergen: String, severity: String) -> Self {
        Self { allergen, severity }
    }
}

// Struct representing an Immunization
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Immunization {
    name: String,
    date: String,
}

impl Immunization {
    /// Create a new immunization with the given information
    pub fn new(name: String, date: String) -> Self {
        Self { name, date }
    }
}

// Struct representing a medical Procedure
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Procedure {
    name: String,
    date: String,
    description: String,
}

impl Procedure {
    /// Create a new procedure with the given information
    pub fn new(name: String, date: String, description: String) -> Self {
        Self {
            name,
            date,
            description,
        }
    }
}

// Struct representing a patient's medical information
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Patient {
    full_name: String,
    birthday: String,
    gender: String,
    blood_type: String,
    records: Vec<RecordType>,
    allergies: Vec<Allergy>,
    immunizations: Vec<Immunization>,
    procedures: Vec<Procedure>,
}

impl Patient {
    /// Create a new patient with the given information
    pub fn new(
        full_name: String,
        birthday: String,
        gender: String,
        blood_type: String,
        records: Vec<RecordType>,
        allergies: Vec<Allergy>,
        immunizations: Vec<Immunization>,
        procedures: Vec<Procedure>,
    ) -> Self {
        Self {
            full_name,
            birthday,
            gender,
            blood_type,
            records,
            allergies,
            immunizations,
            procedures,
        }
    }

    /// Update the name of the patient
    pub fn update_full_name(&mut self, full_name: String) {
        self.full_name = full_name;
    }

    /// Update the birthday of the patient
    pub fn update_birthday(&mut self, birthday: String) {
        self.birthday = birthday;
    }

    /// Update the blood type of the patient
    pub fn update_blood_type(&mut self, blood_type: String) {
        self.blood_type = blood_type;
    }

    /// Add a new allergy to the patient's allergies
    pub fn add_allergy(&mut self, allergy: Allergy) {
        self.allergies.push(allergy);
    }

    /// Add a new immunization to the patient's immunizations
    pub fn add_immunization(&mut self, immunization: Immunization) {
        self.immunizations.push(immunization);
    }

    /// Add a new procedure to the patient's procedures
    pub fn add_procedure(&mut self, procedure: Procedure) {
        self.procedures.push(procedure);
    }

    /// Add a new medical record of type MedicalRecord to the patient's records
    pub fn add_medical_record(&mut self, medical_record: MedicalRecord) {
        self.records.push(RecordType::MedicalRecord(medical_record));
    }

    /// Add a new treatment record to the patient's records
    pub fn add_treatment(&mut self, treatment_record: Treatment) {
        self.records.push(RecordType::Treatment(treatment_record));
    }

    /// Add a new family medical record to the patient's records
    pub fn add_family_medical_record(&mut self, family_medical_record: FamilyMedicalRecord) {
        self.records
            .push(RecordType::FamilyMedicalRecord(family_medical_record));
    }

    /// Get the patient's birthday
    pub fn birthday(&self) -> String {
        self.birthday.clone()
    }

    /// Get the patient's blood type
    pub fn blood_type(&self) -> String {
        self.blood_type.clone()
    }

    /// Get the patient's gender
    pub fn gender(&self) -> String {
        self.gender.clone()
    }

    /// Get the patient's records
    pub fn records(&self) -> Vec<RecordType> {
        self.records.clone()
    }

    /// Get the patient's allergies
    pub fn allergies(&self) -> Vec<Allergy> {
        self.allergies.clone()
    }

    /// Get the patient's immunizations
    pub fn immunizations(&self) -> Vec<Immunization> {
        self.immunizations.clone()
    }

    /// Get the patient's procedures
    pub fn procedures(&self) -> Vec<Procedure> {
        self.procedures.clone()
    }
}
