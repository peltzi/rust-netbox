# Site

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**name** | **String** |  | 
**slug** | **String** |  | 
**status** | Option<[**crate::models::Status**](Status.md)> |  | [optional]
**region** | Option<[**crate::models::NestedRegion**](NestedRegion.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**facility** | Option<**String**> |  | [optional]
**asn** | Option<**i32**> |  | [optional]
**time_zone** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**physical_address** | Option<**String**> |  | [optional]
**shipping_address** | Option<**String**> |  | [optional]
**latitude** | Option<**String**> |  | [optional]
**longitude** | Option<**String**> |  | [optional]
**contact_name** | Option<**String**> |  | [optional]
**contact_phone** | Option<**String**> |  | [optional]
**contact_email** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**circuit_count** | Option<**i32**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]
**prefix_count** | Option<**i32**> |  | [optional][readonly]
**rack_count** | Option<**i32**> |  | [optional][readonly]
**virtualmachine_count** | Option<**i32**> |  | [optional][readonly]
**vlan_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


