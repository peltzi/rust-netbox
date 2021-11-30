# VirtualMachineWithConfigContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**status** | Option<[**crate::models::Status10**](Status_10.md)> |  | [optional]
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | [optional]
**cluster** | Option<[**crate::models::NestedCluster**](NestedCluster.md)> |  | 
**role** | Option<[**crate::models::NestedDeviceRole**](NestedDeviceRole.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**platform** | Option<[**crate::models::NestedPlatform**](NestedPlatform.md)> |  | [optional]
**primary_ip** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**primary_ip4** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**vcpus** | Option<**f32**> |  | [optional]
**memory** | Option<**i32**> |  | [optional]
**disk** | Option<**i32**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**local_context_data** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**config_context** | Option<**::std::collections::HashMap<String, Value>**> |  | [optional][readonly]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


