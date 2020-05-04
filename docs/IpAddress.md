# IpAddress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**family** | Option<[**crate::models::Family**](Family.md)> |  | [optional]
**address** | **String** | IPv4 or IPv6 address (with mask) | 
**vrf** | Option<[**crate::models::NestedVrf**](NestedVRF.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**status** | Option<[**crate::models::Status6**](Status_6.md)> |  | [optional]
**role** | Option<[**crate::models::Role1**](Role_1.md)> |  | [optional]
**interface** | Option<[**crate::models::IpAddressInterface**](IPAddressInterface.md)> |  | [optional]
**nat_inside** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**nat_outside** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**dns_name** | Option<**String**> | Hostname or FQDN (not case-sensitive) | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


