use serde_json::{json, Value};
use basyx_rs::{prelude::SubmodelElement, submodel_element, Submodel};

pub fn submodel_to_submodel_value(submodel: Value) -> Value{
    fn submodel_elements_to_submodel_value(submodel_elements: Vec<SubmodelElement>) -> Value {
        let mut submodel_element_value: Value = json!({});
    
        for submodel_element in submodel_elements.iter() {
            match submodel_element {
                // Handle `Property` element
                SubmodelElement::Property(property) => {
                    if let Some(id_short) = &property.id_short {
                        if let Some(value) = &property.value {
                            submodel_element_value[id_short] = json!(value);
                        } else {
                            submodel_element_value[id_short] = json!(null);
                        }
                    }
                }
    
                // // Handle `MultiLanguageProperty` element
                // SubmodelElement::MultiLanguageProperty(multi_lang_property) => {
                //     if let Some(id_short) = &multi_lang_property.id_short {
                //         let mut language_json = json!({});
                //         if let Some(values) = &multi_lang_property.value {
                //             for value in values {
                //                 if let (Some(language), Some(text)) = (&value.language, &value.text) {
                //                     language_json[language] = json!(text);
                //                 }
                //             }
                //         }
                //         submodel_element_value[id_short] = language_json;
                //     }
                // }
    
                // Handle `SubmodelElementCollection` recursively
                SubmodelElement::SubmodelElementCollection(submodel_element_collection) => {
                    if let Some(id_short) = &submodel_element_collection.id_short {
                        if let Some(nested_elements) = &submodel_element_collection.value {
                            let nested_value = submodel_elements_to_submodel_value(nested_elements.clone());
                            submodel_element_value[id_short] = nested_value;
                        }
                    }
                }
    
                // Handle other variants if necessary
                _ => {}
            }
        }
    
        submodel_element_value
    }

    let submodel: Submodel = serde_json::from_value(submodel).unwrap();
    let mut submodel_value: Value = json!({});

    if let Some(submodel_elements) = submodel.submodel_elements {
        submodel_value = submodel_elements_to_submodel_value(submodel_elements);
    }

    return submodel_value; 
}

pub fn merge_submodel_value_to_submodel(submodel: Value, submodel_value: Value) -> Value {
    fn merge_submodel_value_to_submodel_elements(
        submodel_elements: &mut Vec<SubmodelElement>,
        submodel_value: Value,
    ) {
        for submodel_element in submodel_elements.iter_mut() {
            match submodel_element {
                // Match on Property variant
                SubmodelElement::Property(ref mut property) => {
                    if let Some(id_short) = &property.id_short {
                        if let Some(new_value) = submodel_value.get(id_short) {
                            if let Some(value_field) = &mut property.value {
                                if let Some(string_value) = new_value.as_str() {
                                    *value_field = string_value.to_string();  // Assign the new value
                                }
                                else {
                                    *value_field = new_value.to_string();  // Assign the new value
                                }
                            } else {
                                // If property.value is None, create a new value
                                property.value = Some(
                                    if let Some(string_value) = new_value.as_str() {
                                        string_value.to_string()  // Create a new String from the string value
                                    } else {
                                        new_value.to_string()  // Create a new String from the JSON serialized value
                                    }
                                );
                            }
                        }
                    }
                }
    
                // Match on SubmodelElementCollection variant and recurse
                SubmodelElement::SubmodelElementCollection(ref mut submodel_element_collection) => {
                    if let Some(id_short) = &submodel_element_collection.id_short {
                        if let Some(new_value) = submodel_value.get(id_short) {
                            if let Some(value_field) = &mut submodel_element_collection.value {
                                merge_submodel_value_to_submodel_elements(value_field, new_value.clone());
                            }
                        }
                    }
                }
    
                // Other variants are ignored
                _ => {}
            }
        }
    }
    

    let mut submodel: Submodel = serde_json::from_value(submodel).unwrap();
    // Copy
    if let Some(submodel_elements) = submodel.submodel_elements.as_mut() {
        // Directly modify `submodel_elements` via a mutable reference
        merge_submodel_value_to_submodel_elements(submodel_elements, submodel_value);
    }

    let merge_submodel: Value = serde_json::to_value(submodel).unwrap();

    return merge_submodel;
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn test_submodel_to_submodel_value(){
        let submodel: Value = json!({
            "modelType": "Submodel",
            "kind": "Instance",
            "id": "https://hilscher.com/ids/sm/9390_4160_0132_0940",
            "description": [
                {
                    "language": "en",
                    "text": "Providessome basic system information such as CPU type, CPU usage, installed memory size, memory usage, temperatures and others."
                }
            ],
            "idShort": "SystemInformation",
            "submodelElements": [
                {
                    "modelType": "Property",
                    "value": "NORMAL",
                    "valueType": "xs:string",
                    "description": [
                        {
                            "language": "en",
                            "text": "Note: One out of the following NAMUR NE107 status: NORMAL, FAILURE, CHECK_FUNCTION, OFF_SPEC, MAINTENANCE_REQUIRED"
                        }
                    ],
                    "idShort": "HealthStatus"
                },
                {
                    "modelType": "Property",
                    "valueType": "xs:dateTime",
                    "description": [
                        {
                            "language": "en",
                            "text": "Note: Timestamp of the last update"
                        }
                    ],
                    "idShort": "LastUpdate"
                },
                {
                    "modelType": "SubmodelElementCollection",
                    "idShort": "Hardware",
                    "value": [
                        {
                            "modelType": "SubmodelElementCollection",
                            "idShort": "Processor",
                            "value": [
                                {
                                    "modelType": "Property",
                                    "value": "ARMv8",
                                    "valueType": "xs:string",
                                    "idShort": "CpuType"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "4",
                                    "valueType": "xs:integer",
                                    "idShort": "CpuCores"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "1.8 GHz",
                                    "valueType": "xs:string",
                                    "idShort": "CpuClock"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "17 %",
                                    "valueType": "xs:string",
                                    "idShort": "CpuUsage"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "55 °C",
                                    "valueType": "xs:string",
                                    "idShort": "CpuTemperature"
                                }
                            ]
                        },
                        {
                            "modelType": "SubmodelElementCollection",
                            "idShort": "Memory",
                            "value": [
                                {
                                    "modelType": "Property",
                                    "value": "2 GB",
                                    "valueType": "xs:string",
                                    "idShort": "RAMInstalled"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "729 MB",
                                    "valueType": "xs:string",
                                    "idShort": "RAMFree"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "17 GB",
                                    "valueType": "xs:string",
                                    "idShort": "DiskInstalled"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "14.3 GB",
                                    "valueType": "xs:string",
                                    "idShort": "DiskFree"
                                }
                            ]
                        },
                        {
                            "modelType": "Property",
                            "value": "42 °C",
                            "valueType": "xs:string",
                            "idShort": "BoardTemperature"
                        }
                    ]
                }
            ]
        });

        let expected_output: Value = json!({
            "LastUpdate": null,
            "HealthStatus": "NORMAL",
            "Hardware": {
                "Processor": {
                    "CpuClock": "1.8 GHz",
                    "CpuCores": "4",
                    "CpuTemperature": "55 °C",
                    "CpuUsage": "17 %",
                    "CpuType": "ARMv8"
                },
                "BoardTemperature": "42 °C",
                "Memory": {
                    "RAMInstalled": "2 GB",
                    "RAMFree": "729 MB",
                    "DiskFree": "14.3 GB",
                    "DiskInstalled": "17 GB"
                }
            }
        });

        let result = submodel_to_submodel_value(submodel);
        println!("Result:\n{}", serde_json::to_string_pretty(&result).unwrap());
        println!("Expected:\n{}", serde_json::to_string_pretty(&expected_output).unwrap());
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_merge_recursive_object(){
        let submodel: Value = json!({
            "modelType": "Submodel",
            "kind": "Instance",
            "id": "https://hilscher.com/ids/sm/9390_4160_0132_0940",
            "description": [
                {
                    "language": "en",
                    "text": "Providessome basic system information such as CPU type, CPU usage, installed memory size, memory usage, temperatures and others."
                }
            ],
            "idShort": "SystemInformation",
            "submodelElements": [
                {
                    "modelType": "Property",
                    "value": "NORMAL",
                    "valueType": "xs:string",
                    "description": [
                        {
                            "language": "en",
                            "text": "Note: One out of the following NAMUR NE107 status: NORMAL, FAILURE, CHECK_FUNCTION, OFF_SPEC, MAINTENANCE_REQUIRED"
                        }
                    ],
                    "idShort": "HealthStatus"
                },
                {
                    "modelType": "Property",
                    "valueType": "xs:dateTime",
                    "description": [
                        {
                            "language": "en",
                            "text": "Note: Timestamp of the last update"
                        }
                    ],
                    "idShort": "LastUpdate"
                },
                {
                    "modelType": "SubmodelElementCollection",
                    "idShort": "Hardware",
                    "value": [
                        {
                            "modelType": "SubmodelElementCollection",
                            "idShort": "Processor",
                            "value": [
                                {
                                    "modelType": "Property",
                                    "value": "ARMv8",
                                    "valueType": "xs:string",
                                    "idShort": "CpuType"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "4",
                                    "valueType": "xs:integer",
                                    "idShort": "CpuCores"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "1.8 GHz",
                                    "valueType": "xs:string",
                                    "idShort": "CpuClock"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "17 %",
                                    "valueType": "xs:string",
                                    "idShort": "CpuUsage"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "55 °C",
                                    "valueType": "xs:string",
                                    "idShort": "CpuTemperature"
                                }
                            ]
                        },
                        {
                            "modelType": "SubmodelElementCollection",
                            "idShort": "Memory",
                            "value": [
                                {
                                    "modelType": "Property",
                                    "value": "2 GB",
                                    "valueType": "xs:string",
                                    "idShort": "RAMInstalled"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "729 MB",
                                    "valueType": "xs:string",
                                    "idShort": "RAMFree"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "17 GB",
                                    "valueType": "xs:string",
                                    "idShort": "DiskInstalled"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "14.3 GB",
                                    "valueType": "xs:string",
                                    "idShort": "DiskFree"
                                }
                            ]
                        },
                        {
                            "modelType": "Property",
                            "value": "42 °C",
                            "valueType": "xs:string",
                            "idShort": "BoardTemperature"
                        }
                    ]
                }
            ]
        });

        let submodel_value: Value = json!({
            "HealthStatus": "NORMAL",
            "LastUpdate": "2022-01-01T12:00:00Z",
            "Hardware": {
                "Processor": {
                    "CpuType": "ARMv7",
                    "CpuCores": 4,
                    "CpuClock": "1.8 GHz",
                    "CpuUsage": "17 %",
                    "CpuTemperature": "55 °C"
                },
                "Memory": {
                    "RAMInstalled": "2 GB",
                    "RAMFree": "729 MB",
                    "DiskInstalled": "17 GB",
                    "DiskFree": "14.3 GB"
                },
                "BoardTemperature": "50 °C"
            }
        });

        let expected_output: Value = json!({
            "modelType": "Submodel",
            "kind": "Instance",
            "id": "https://hilscher.com/ids/sm/9390_4160_0132_0940",
            "description": [
                {
                    "language": "en",
                    "text": "Providessome basic system information such as CPU type, CPU usage, installed memory size, memory usage, temperatures and others."
                }
            ],
            "idShort": "SystemInformation",
            "submodelElements": [
                {
                    "modelType": "Property",
                    "value": "NORMAL",
                    "valueType": "xs:string",
                    "description": [
                        {
                            "language": "en",
                            "text": "Note: One out of the following NAMUR NE107 status: NORMAL, FAILURE, CHECK_FUNCTION, OFF_SPEC, MAINTENANCE_REQUIRED"
                        }
                    ],
                    "idShort": "HealthStatus"
                },
                {
                    "modelType": "Property",
                    "valueType": "xs:dateTime",
                    "description": [
                        {
                            "language": "en",
                            "text": "Note: Timestamp of the last update"
                        }
                    ],
                    "idShort": "LastUpdate"
                },
                {
                    "modelType": "SubmodelElementCollection",
                    "idShort": "Hardware",
                    "value": [
                        {
                            "modelType": "SubmodelElementCollection",
                            "idShort": "Processor",
                            "value": [
                                {
                                    "modelType": "Property",
                                    "value": "ARMv7",
                                    "valueType": "xs:string",
                                    "idShort": "CpuType"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "4",
                                    "valueType": "xs:integer",
                                    "idShort": "CpuCores"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "1.8 GHz",
                                    "valueType": "xs:string",
                                    "idShort": "CpuClock"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "17 %",
                                    "valueType": "xs:string",
                                    "idShort": "CpuUsage"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "55 °C",
                                    "valueType": "xs:string",
                                    "idShort": "CpuTemperature"
                                }
                            ]
                        },
                        {
                            "modelType": "SubmodelElementCollection",
                            "idShort": "Memory",
                            "value": [
                                {
                                    "modelType": "Property",
                                    "value": "2 GB",
                                    "valueType": "xs:string",
                                    "idShort": "RAMInstalled"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "729 MB",
                                    "valueType": "xs:string",
                                    "idShort": "RAMFree"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "17 GB",
                                    "valueType": "xs:string",
                                    "idShort": "DiskInstalled"
                                },
                                {
                                    "modelType": "Property",
                                    "value": "14.3 GB",
                                    "valueType": "xs:string",
                                    "idShort": "DiskFree"
                                }
                            ]
                        },
                        {
                            "modelType": "Property",
                            "value": "50 °C",
                            "valueType": "xs:string",
                            "idShort": "BoardTemperature"
                        }
                    ]
                }
            ]
        });

        // Call the function and compare the result with the expected output
        let result = merge_submodel_value_to_submodel(submodel, submodel_value);
        println!("Result:\n{}", serde_json::to_string_pretty(&result).unwrap());
        println!("Expected:\n{}", serde_json::to_string_pretty(&expected_output).unwrap());
        assert_eq!(result, expected_output);
    }
    #[test]
    fn test_merge_json_objects() {
        // Define the input JSON objects
        let submodel: Value = json!({
            "modelType": "Submodel",
            "kind": "Instance",
            "semanticId": {
                "keys": [],
                "type": "ExternalReference"
            },
            "id": "https://murrelektronik.com/ids/ManagedDevice/9120_3140_4042_4807",
            "description": [
                {
                    "language": "en",
                    "text": "Contains information about menagement endpoint and onboarding status"
                }
            ],
            "idShort": "ManagedDevice",
            "submodelElements": [
                {
                    "modelType": "Property",
                    "semanticId": {
                        "keys": [],
                        "type": "ExternalReference"
                    },
                    "value": "OFFBOARDED",
                    "valueType": "xs:string",
                    "description": [
                        {
                            "language": "en",
                            "text": "Note: One out of the following status are valide: OFFBOARDED, ONBOARDED, OFFBOARDING_REQUESTED. The devixe can set OFFBOARDED and ONBOARDED. The EMS can set OFFBOARDING_REQUESTED, so the device can than set to OFFBOARDED. Default is OFFBOARDED."
                        }
                    ],
                    "idShort": "BoardingStatus"
                },
                {
                    "modelType": "Property",
                    "semanticId": {
                        "keys": [],
                        "type": "ExternalReference"
                    },
                    "valueType": "xs:string",
                    "category": "VARIABLE",
                    "description": [
                        {
                            "language": "en",
                            "text": "Note: Timestamp of the last update"
                        }
                    ],
                    "idShort": "LastUpdate"
                }
            ]
        });

        let submodel_value: Value = json!({
            "BoardingStatus": "ONBOARDED",
            "LastUpdate": null
        });

        // Expected output after merging
        let expected_output: Value = json!({
            "modelType": "Submodel",
            "kind": "Instance",
            "semanticId": {
                "keys": [],
                "type": "ExternalReference"
            },
            "id": "https://murrelektronik.com/ids/ManagedDevice/9120_3140_4042_4807",
            "description": [
                {
                    "language": "en",
                    "text": "Contains information about menagement endpoint and onboarding status"
                }
            ],
            "idShort": "ManagedDevice",
            "submodelElements": [
                {
                    "modelType": "Property",
                    "semanticId": {
                        "keys": [],
                        "type": "ExternalReference"
                    },
                    "value": "ONBOARDED",
                    "valueType": "xs:string",
                    "description": [
                        {
                            "language": "en",
                            "text": "Note: One out of the following status are valide: OFFBOARDED, ONBOARDED, OFFBOARDING_REQUESTED. The devixe can set OFFBOARDED and ONBOARDED. The EMS can set OFFBOARDING_REQUESTED, so the device can than set to OFFBOARDED. Default is OFFBOARDED."
                        }
                    ],
                    "idShort": "BoardingStatus"
                },
                {
                    "modelType": "Property",
                    "semanticId": {
                        "keys": [],
                        "type": "ExternalReference"
                    },
                    "valueType": "xs:string",
                    "category": "VARIABLE",
                    "description": [
                        {
                            "language": "en",
                            "text": "Note: Timestamp of the last update"
                        }
                    ],
                    "idShort": "LastUpdate"
                }
            ]
        });

        // Call the function and compare the result with the expected output
        let result = merge_submodel_value_to_submodel(submodel, submodel_value);
        println!("Result:\n{}", serde_json::to_string_pretty(&result).unwrap());
        println!("Expected:\n{}", serde_json::to_string_pretty(&expected_output).unwrap());
        assert_eq!(result, expected_output);
    }
}
