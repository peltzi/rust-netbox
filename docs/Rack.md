# Rack

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**name** | **String** |  | 
**facility_id** | Option<**String**> |  | [optional]
**display_name** | Option<**String**> |  | [optional][readonly]
**site** | [**crate::models::NestedSite**](NestedSite.md) |  | 
**group** | Option<[**crate::models::NestedRackGroup**](NestedRackGroup.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**status** | Option<[**crate::models::Status**](Status.md)> |  | [optional]
**role** | Option<[**crate::models::NestedRackRole**](NestedRackRole.md)> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this rack | [optional]
**_type** | Option<[**crate::models::Status**](Status.md)> |  | [optional]
**width** | Option<[**crate::models::Status**](Status.md)> |  | [optional]
**u_height** | Option<**i32**> |  | [optional]
**desc_units** | Option<**bool**> | Units are numbered top-to-bottom | [optional]
**outer_width** | Option<**i32**> |  | [optional]
**outer_depth** | Option<**i32**> |  | [optional]
**outer_unit** | Option<[**crate::models::Status**](Status.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]
**powerfeed_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


