# WritablePowerFeed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**power_panel** | **i32** |  | 
**rack** | Option<**i32**> |  | [optional]
**name** | **String** |  | 
**status** | Option<**String**> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**supply** | Option<**String**> |  | [optional]
**phase** | Option<**String**> |  | [optional]
**voltage** | Option<**i32**> |  | [optional]
**amperage** | Option<**i32**> |  | [optional]
**max_utilization** | Option<**i32**> | Maximum permissible draw (percentage) | [optional]
**comments** | Option<**String**> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::NestedCable**](NestedCable.md)> |  | [optional]
**cable_peer** | Option<**::std::collections::HashMap<String, Value>**> |  Return the appropriate serializer for the cable termination model.  | [optional][readonly]
**cable_peer_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoint** | Option<**::std::collections::HashMap<String, Value>**> |  Return the appropriate serializer for the type of connected object.  | [optional][readonly]
**connected_endpoint_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoint_reachable** | Option<**bool**> |  | [optional][readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**_occupied** | Option<**bool**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


