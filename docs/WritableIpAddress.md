# WritableIpAddress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**family** | Option<**i32**> |  | [optional][readonly]
**address** | **String** | IPv4 or IPv6 address (with mask) | 
**vrf** | Option<**i32**> |  | [optional]
**tenant** | Option<**i32**> |  | [optional]
**status** | Option<**String**> | The operational status of this IP | [optional]
**role** | Option<**String**> | The functional role of this IP | [optional]
**interface** | Option<**i32**> |  | [optional]
**nat_inside** | Option<**i32**> | The IP for which this address is the \"outside\" IP | [optional]
**nat_outside** | **i32** |  | 
**dns_name** | Option<**String**> | Hostname or FQDN (not case-sensitive) | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


