# WritableFrontPort

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**device** | **i32** |  | 
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**_type** | **String** |  | 
**color** | Option<**String**> |  | [optional]
**rear_port** | **i32** |  | 
**rear_port_position** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::NestedCable**](NestedCable.md)> |  | [optional]
**cable_peer** | Option<**::std::collections::HashMap<String, Value>**> |  Return the appropriate serializer for the cable termination model.  | [optional][readonly]
**cable_peer_type** | Option<**String**> |  | [optional][readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**_occupied** | Option<**bool**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


