# DeviceInterface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**device** | [**crate::models::NestedDevice**](NestedDevice.md) |  | 
**name** | **String** |  | 
**_type** | [**crate::models::Type2**](Type_2.md) |  | 
**enabled** | Option<**bool**> |  | [optional]
**lag** | Option<[**crate::models::NestedInterface**](NestedInterface.md)> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**mgmt_only** | Option<**bool**> | This interface is used only for out-of-band management | [optional]
**description** | Option<**String**> |  | [optional]
**connected_endpoint_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoint** | Option<**::std::collections::HashMap<String, String>**> |          Return the appropriate serializer for the type of connected object.          | [optional][readonly]
**connection_status** | Option<[**crate::models::ConnectionStatus**](Connection_status.md)> |  | [optional]
**cable** | Option<[**crate::models::NestedCable**](NestedCable.md)> |  | [optional]
**mode** | Option<[**crate::models::Mode**](Mode.md)> |  | [optional]
**untagged_vlan** | Option<[**crate::models::NestedVlan**](NestedVLAN.md)> |  | [optional]
**tagged_vlans** | Option<[**Vec<crate::models::NestedVlan>**](NestedVLAN.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**count_ipaddresses** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


