# CircuitTermination

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**circuit** | [**crate::models::NestedCircuit**](NestedCircuit.md) |  | 
**term_side** | **String** |  | 
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | [optional]
**provider_network** | Option<[**crate::models::NestedProviderNetwork**](NestedProviderNetwork.md)> |  | [optional]
**port_speed** | Option<**i32**> |  | [optional]
**upstream_speed** | Option<**i32**> | Upstream speed, if different from port speed | [optional]
**xconnect_id** | Option<**String**> |  | [optional]
**pp_info** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::NestedCable**](NestedCable.md)> |  | [optional]
**cable_peer** | Option<**::std::collections::HashMap<String, Value>**> |  Return the appropriate serializer for the cable termination model.  | [optional][readonly]
**cable_peer_type** | Option<**String**> |  | [optional][readonly]
**_occupied** | Option<**bool**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


