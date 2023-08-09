## These are some CLI Functions to run so that we can show the functionality of the Smart Contract.

### IPFS LINK TO USE https://gateway.lighthouse.storage/ipfs/QmQ21qd6LyW11swTDZZj4QWJTea3PTGsHK2xgZJe23sbeU

1. Make sure the contract is deployed by running `near dev-deploy ./target/wasm32-unknown-unknown/release/medibridge_contract_rust.wasm   new  '{}'`

2. Now the first function is to add Patients to the blockchain. This function takes in the basic information such as FullName, Birthday, Gender and Blood type and creates a user profile mapped to their accountId. We use `predecessor_account_id` to make sure the accountId is created against the callers id.

```bash
near call dev-1691589894264-83540198083565 add_patient '{"full_name":"Prakhar","birthday":"30","gender":"male","blood_type":"0+"}' --accountId kinosxz.testnet
```


3. Going over to fetch the account information of the Patient logged in to us. 
```bash
near call  dev-1691589894264-83540198083565 get_patient '{}' --accountId dev-1691589894264-83540198083565
```

4. Add allergies for the account
```bash
near call  dev-1691589894264-83540198083565 add_allergy '{"allergen":"Balsam of Peru", "severity":"mild"}' --accountId dev-1691589894264-83540198083565
```

5. Add Immunization for the account
```bash
near call  dev-1691589894264-83540198083565 add_immunization '{"name":"name", "date":"30/07/2023"}' --accountId dev-1691589894264-83540198083565
```

6. Add add_medical_record for the account
```bash
near call  dev-1691589894264-83540198083565 add_immunization '{"id":123, "condition":"Type 1 Diabetes","record_data":"https://gateway.lighthouse.storage/ipfs/QmQ21qd6LyW11swTDZZj4QWJTea3PTGsHK2xgZJe23sbeU","date":"30/08/2023","public":true}' --accountId dev-1691589894264-83540198083565
```

7. Add add_family_medical_record for the account
```bash
near call  dev-1691589894264-83540198083565 add_immunization '{"id":123, "condition":"Type 1 Diabetes","relation_to_patient":"Father","public":true}' --accountId dev-1691589894264-83540198083565
```