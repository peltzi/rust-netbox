# PowerFeed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**power_panel** | [**crate::models::NestedPowerPanel**](NestedPowerPanel.md) |  | 
**rack** | Option<[**crate::models::NestedRack**](NestedRack.md)> |  | [optional]
**name** | **String** |  | 
**status** | Option<[**crate::models::Status3**](Status_3.md)> |  | [optional]
**_type** | Option<[**crate::models::Type4**](Type_4.md)> |  | [optional]
**supply** | Option<[**crate::models::Supply**](Supply.md)> |  | [optional]
**phase** | Option<[**crate::models::Phase**](Phase.md)> |  | [optional]
**voltage** | Option<**i32**> |  | [optional]
**amperage** | Option<**i32**> |  | [optional]
**max_utilization** | Option<**i32**> | Maximum permissible draw (percentage) | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


