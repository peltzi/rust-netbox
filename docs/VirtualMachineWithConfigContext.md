# VirtualMachineWithConfigContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**name** | **String** |  | 
**status** | Option<[**crate::models::Status9**](Status_9.md)> |  | [optional]
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | [optional]
**cluster** | [**crate::models::NestedCluster**](NestedCluster.md) |  | 
**role** | Option<[**crate::models::NestedDeviceRole**](NestedDeviceRole.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**platform** | Option<[**crate::models::NestedPlatform**](NestedPlatform.md)> |  | [optional]
**primary_ip** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**primary_ip4** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**vcpus** | Option<**i32**> |  | [optional]
**memory** | Option<**i32**> |  | [optional]
**disk** | Option<**i32**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**local_context_data** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**config_context** | Option<**::std::collections::HashMap<String, String>**> |  | [optional][readonly]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


