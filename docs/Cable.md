# Cable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**termination_a_type** | **String** |  | 
**termination_a_id** | **i32** |  | 
**termination_a** | Option<**::std::collections::HashMap<String, Value>**> |  | [optional][readonly]
**termination_b_type** | **String** |  | 
**termination_b_id** | **i32** |  | 
**termination_b** | Option<**::std::collections::HashMap<String, Value>**> |  | [optional][readonly]
**_type** | Option<**String**> |  | [optional]
**status** | Option<[**crate::models::Status1**](Status_1.md)> |  | [optional]
**label** | Option<**String**> |  | [optional]
**color** | Option<**String**> |  | [optional]
**length** | Option<**f32**> |  | [optional]
**length_unit** | Option<[**crate::models::LengthUnit**](Length_unit.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


