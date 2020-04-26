# WritableDeviceWithConfigContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**display_name** | Option<**String**> |  | [optional][readonly]
**device_type** | **i32** |  | 
**device_role** | **i32** |  | 
**tenant** | Option<**i32**> |  | [optional]
**platform** | Option<**i32**> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this device | [optional]
**site** | **i32** |  | 
**rack** | Option<**i32**> |  | [optional]
**position** | Option<**i32**> | The lowest-numbered unit occupied by the device | [optional]
**face** | Option<**String**> |  | [optional]
**parent_device** | Option<[**crate::models::NestedDevice**](NestedDevice.md)> |  | [optional]
**status** | Option<**String**> |  | [optional]
**primary_ip** | Option<**String**> |  | [optional][readonly]
**primary_ip4** | Option<**i32**> |  | [optional]
**primary_ip6** | Option<**i32**> |  | [optional]
**cluster** | Option<**i32**> |  | [optional]
**virtual_chassis** | Option<**i32**> |  | [optional]
**vc_position** | Option<**i32**> |  | [optional]
**vc_priority** | Option<**i32**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**local_context_data** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**config_context** | Option<**::std::collections::HashMap<String, String>**> |  | [optional][readonly]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


