# WritableDeviceInterface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**device** | **i32** |  | 
**name** | **String** |  | 
**_type** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**lag** | Option<**i32**> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**mgmt_only** | Option<**bool**> | This interface is used only for out-of-band management | [optional]
**description** | Option<**String**> |  | [optional]
**connected_endpoint_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoint** | Option<**::std::collections::HashMap<String, String>**> |          Return the appropriate serializer for the type of connected object.          | [optional][readonly]
**connection_status** | Option<**bool**> |  | [optional]
**cable** | Option<[**crate::models::NestedCable**](NestedCable.md)> |  | [optional]
**mode** | Option<**String**> |  | [optional]
**untagged_vlan** | Option<**i32**> |  | [optional]
**tagged_vlans** | Option<**Vec<i32>**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**count_ipaddresses** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


