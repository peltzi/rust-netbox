# PowerPort

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**device** | [**crate::models::NestedDevice**](NestedDevice.md) |  | 
**name** | **String** |  | 
**_type** | Option<[**crate::models::Type3**](Type_3.md)> |  | [optional]
**maximum_draw** | Option<**i32**> | Maximum power draw (watts) | [optional]
**allocated_draw** | Option<**i32**> | Allocated power draw (watts) | [optional]
**description** | Option<**String**> |  | [optional]
**connected_endpoint_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoint** | Option<**::std::collections::HashMap<String, String>**> |          Return the appropriate serializer for the type of connected object.          | [optional][readonly]
**connection_status** | Option<[**crate::models::ConnectionStatus**](Connection_status.md)> |  | [optional]
**cable** | Option<[**crate::models::NestedCable**](NestedCable.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


