# ObjectChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**time** | Option<**String**> |  | [optional][readonly]
**user** | Option<[**crate::models::NestedUser**](NestedUser.md)> |  | [optional]
**user_name** | Option<**String**> |  | [optional][readonly]
**request_id** | Option<**String**> |  | [optional][readonly]
**action** | Option<[**crate::models::Action**](Action.md)> |  | [optional]
**changed_object_type** | Option<**String**> |  | [optional][readonly]
**changed_object_id** | **i32** |  | 
**changed_object** | Option<**::std::collections::HashMap<String, String>**> |          Serialize a nested representation of the changed object.          | [optional][readonly]
**object_data** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


