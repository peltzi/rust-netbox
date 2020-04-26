# Tenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**name** | **String** |  | 
**slug** | **String** |  | 
**group** | Option<[**crate::models::NestedTenantGroup**](NestedTenantGroup.md)> |  | [optional]
**description** | Option<**String**> | Long-form name (optional) | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**circuit_count** | Option<**i32**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]
**ipaddress_count** | Option<**i32**> |  | [optional][readonly]
**prefix_count** | Option<**i32**> |  | [optional][readonly]
**rack_count** | Option<**i32**> |  | [optional][readonly]
**site_count** | Option<**i32**> |  | [optional][readonly]
**virtualmachine_count** | Option<**i32**> |  | [optional][readonly]
**vlan_count** | Option<**i32**> |  | [optional][readonly]
**vrf_count** | Option<**i32**> |  | [optional][readonly]
**cluster_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


