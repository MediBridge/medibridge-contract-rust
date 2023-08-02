# MediBridge Contract

## Overview
This repository contains the source code for medibridge contract, a smart contract built on the NEAR Protocol. The contract is designed to manage medical records for patients and provide secure access to public records. The smart contract defines various types and structs to represent different medical records and patient information.


## Types and Structs

### Contract 
The main smart contract struct that holds the state of the contract.
### Fields:
- `patients`: A lookup map that associates patient accounts with their medical information represented by Patient struct.
- `public_records`: A vector containing public information related to medical records.
>> Note: Public records are accessible to anyone for anonymous medical research purposes. The information in public records is anonymized to protect patient identities and ensure confidentiality. Researchers can gain valuable insights from the aggregated data while respecting the privacy of individual patients, promoting responsible use of medical information for research and public health initiatives.

### `Patient`
Represents a patient's comprehensive medical information.
Fields:
- `full_name`: A string representing the patient's full name.
- `birthday`: A string representing the patient's birthday
- `gender`: A string representing the patient's gender
- `blood_type`: A string representing the patient's blood type.
- `records`: A vector of `RecordType` containing the patient's medical records.
- `allergies`: A vector of `Allergy` containing the patient's allergies.
- `immunizations`: A vector of `Immunization` containing the patient's immunizations.
- `procedures`: A vector of `Procedure` containing the patient's medical procedures.

### `RecordType Enum`
An enumeration that can hold instances of MedicalRecord, Treatment, or FamilyMedicalRecord. This enum is used to classify the type of each medical record in the contract.
- `MedicalRecord`: Represents a medical record for an individual patient.
- `Treatment`: Represents a medical treatment provided to a patient.
- `FamilyMedicalRecord`: Represents a family medical record containing health information for multiple family members.

### `PublicRecord`
Contains public information related to a medical record.
Fields:
- record_type: A RecordType enum value representing the type of medical record.
- birthday: A string representing the patient's birthday.
- gender: A string representing the patient's gender.
>> Note: The `PublicRecord` struct contains public information related to a medical record. We require the user to specify the patient's birthday and gender for the record to be public because we believe this information is important for medical research and analysis. 

### `Allergy`
Represents an allergy a patient might have.

### `Immunization`
Represents a medical immunization received by a patient.

### `Procedure`
Represents a medical procedure performed on a patient.

## Usage
The smart contract provides methods to add patients, update patient information, add medical records, allergies, immunizations, and procedures.
It also allows users to access patient information and retrieve medical records.

>> Note: This is a brief overview of the smart contract. For detailed information about the functions and methods available in the contract, please refer to the contract's code and associated documentation.

------------------
## Development

### Building the Contract
To build the Medibridge smart contract, follow these steps:
- Make sure you have Rust.
- Install the NEAR CLI tool: [NEAR CLI Installation Guide](https://docs.near.org/tools/near-cli)
- Clone the repository from GitHub.
- Navigate to the project directory.
- Use the following command to build the contract:
```bash
cargo build --target wasm32-unknown-unknown --release
```
### Deploying the Contract
To deploy the Medibridge smart contract, you'll need to interact with the NEAR Protocol's blockchain. See [Deploying smart contracts on the NEAR blockchain](https://docs.near.org/develop/deploy#deploying-the-contract)

### Using the Contract
Once the contract is deployed, you can interact with it using the NEAR CLI, NEAR Wallet, or any other NEAR-compatible wallet.

------------------
**Feel free to contribute and report issues!**
