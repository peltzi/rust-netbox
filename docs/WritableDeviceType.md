# WritableDeviceType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**manufacturer** | **i32** |  | 
**model** | **String** |  | 
**slug** | **String** |  | 
**display_name** | Option<**String**> |  | [optional][readonly]
**part_number** | Option<**String**> | Discrete part number (optional) | [optional]
**u_height** | Option<**i32**> |  | [optional]
**is_full_depth** | Option<**bool**> | Device consumes both front and rear rack faces | [optional]
**subdevice_role** | Option<**String**> | Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child. | [optional]
**front_image** | Option<**String**> |  | [optional][readonly]
**rear_image** | Option<**String**> |  | [optional][readonly]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<[**String**](string.md)> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


