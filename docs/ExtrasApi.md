# \ExtrasApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**extras_choices_list**](ExtrasApi.md#extras_choices_list) | **get** /extras/_choices/ | 
[**extras_choices_read**](ExtrasApi.md#extras_choices_read) | **get** /extras/_choices/{id}/ | 
[**extras_config_contexts_create**](ExtrasApi.md#extras_config_contexts_create) | **post** /extras/config-contexts/ | 
[**extras_config_contexts_delete**](ExtrasApi.md#extras_config_contexts_delete) | **delete** /extras/config-contexts/{id}/ | 
[**extras_config_contexts_list**](ExtrasApi.md#extras_config_contexts_list) | **get** /extras/config-contexts/ | 
[**extras_config_contexts_partial_update**](ExtrasApi.md#extras_config_contexts_partial_update) | **patch** /extras/config-contexts/{id}/ | 
[**extras_config_contexts_read**](ExtrasApi.md#extras_config_contexts_read) | **get** /extras/config-contexts/{id}/ | 
[**extras_config_contexts_update**](ExtrasApi.md#extras_config_contexts_update) | **put** /extras/config-contexts/{id}/ | 
[**extras_custom_field_choices_list**](ExtrasApi.md#extras_custom_field_choices_list) | **get** /extras/_custom_field_choices/ | 
[**extras_custom_field_choices_read**](ExtrasApi.md#extras_custom_field_choices_read) | **get** /extras/_custom_field_choices/{id}/ | 
[**extras_export_templates_create**](ExtrasApi.md#extras_export_templates_create) | **post** /extras/export-templates/ | 
[**extras_export_templates_delete**](ExtrasApi.md#extras_export_templates_delete) | **delete** /extras/export-templates/{id}/ | 
[**extras_export_templates_list**](ExtrasApi.md#extras_export_templates_list) | **get** /extras/export-templates/ | 
[**extras_export_templates_partial_update**](ExtrasApi.md#extras_export_templates_partial_update) | **patch** /extras/export-templates/{id}/ | 
[**extras_export_templates_read**](ExtrasApi.md#extras_export_templates_read) | **get** /extras/export-templates/{id}/ | 
[**extras_export_templates_update**](ExtrasApi.md#extras_export_templates_update) | **put** /extras/export-templates/{id}/ | 
[**extras_graphs_create**](ExtrasApi.md#extras_graphs_create) | **post** /extras/graphs/ | 
[**extras_graphs_delete**](ExtrasApi.md#extras_graphs_delete) | **delete** /extras/graphs/{id}/ | 
[**extras_graphs_list**](ExtrasApi.md#extras_graphs_list) | **get** /extras/graphs/ | 
[**extras_graphs_partial_update**](ExtrasApi.md#extras_graphs_partial_update) | **patch** /extras/graphs/{id}/ | 
[**extras_graphs_read**](ExtrasApi.md#extras_graphs_read) | **get** /extras/graphs/{id}/ | 
[**extras_graphs_update**](ExtrasApi.md#extras_graphs_update) | **put** /extras/graphs/{id}/ | 
[**extras_image_attachments_create**](ExtrasApi.md#extras_image_attachments_create) | **post** /extras/image-attachments/ | 
[**extras_image_attachments_delete**](ExtrasApi.md#extras_image_attachments_delete) | **delete** /extras/image-attachments/{id}/ | 
[**extras_image_attachments_list**](ExtrasApi.md#extras_image_attachments_list) | **get** /extras/image-attachments/ | 
[**extras_image_attachments_partial_update**](ExtrasApi.md#extras_image_attachments_partial_update) | **patch** /extras/image-attachments/{id}/ | 
[**extras_image_attachments_read**](ExtrasApi.md#extras_image_attachments_read) | **get** /extras/image-attachments/{id}/ | 
[**extras_image_attachments_update**](ExtrasApi.md#extras_image_attachments_update) | **put** /extras/image-attachments/{id}/ | 
[**extras_object_changes_list**](ExtrasApi.md#extras_object_changes_list) | **get** /extras/object-changes/ | 
[**extras_object_changes_read**](ExtrasApi.md#extras_object_changes_read) | **get** /extras/object-changes/{id}/ | 
[**extras_reports_list**](ExtrasApi.md#extras_reports_list) | **get** /extras/reports/ | 
[**extras_reports_read**](ExtrasApi.md#extras_reports_read) | **get** /extras/reports/{id}/ | 
[**extras_reports_run**](ExtrasApi.md#extras_reports_run) | **post** /extras/reports/{id}/run/ | 
[**extras_scripts_list**](ExtrasApi.md#extras_scripts_list) | **get** /extras/scripts/ | 
[**extras_scripts_read**](ExtrasApi.md#extras_scripts_read) | **get** /extras/scripts/{id}/ | 
[**extras_tags_create**](ExtrasApi.md#extras_tags_create) | **post** /extras/tags/ | 
[**extras_tags_delete**](ExtrasApi.md#extras_tags_delete) | **delete** /extras/tags/{id}/ | 
[**extras_tags_list**](ExtrasApi.md#extras_tags_list) | **get** /extras/tags/ | 
[**extras_tags_partial_update**](ExtrasApi.md#extras_tags_partial_update) | **patch** /extras/tags/{id}/ | 
[**extras_tags_read**](ExtrasApi.md#extras_tags_read) | **get** /extras/tags/{id}/ | 
[**extras_tags_update**](ExtrasApi.md#extras_tags_update) | **put** /extras/tags/{id}/ | 



## extras_choices_list

> extras_choices_list()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_choices_read

> extras_choices_read(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_config_contexts_create

> crate::models::ConfigContext extras_config_contexts_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableConfigContext**](WritableConfigContext.md) |  | [required] |

### Return type

[**crate::models::ConfigContext**](ConfigContext.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_config_contexts_delete

> extras_config_contexts_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this config context. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_config_contexts_list

> crate::models::InlineResponse20037 extras_config_contexts_list(name, is_active, q, region_id, region, site_id, site, role_id, role, platform_id, platform, cluster_group_id, cluster_group, cluster_id, tenant_group_id, tenant_group, tenant_id, tenant, tag, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**is_active** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**region_id** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |
**site_id** | Option<**String**> |  |  |
**site** | Option<**String**> |  |  |
**role_id** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**platform_id** | Option<**String**> |  |  |
**platform** | Option<**String**> |  |  |
**cluster_group_id** | Option<**String**> |  |  |
**cluster_group** | Option<**String**> |  |  |
**cluster_id** | Option<**String**> |  |  |
**tenant_group_id** | Option<**String**> |  |  |
**tenant_group** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**tenant** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20037**](inline_response_200_37.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_config_contexts_partial_update

> crate::models::ConfigContext extras_config_contexts_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this config context. | [required] |
**data** | [**WritableConfigContext**](WritableConfigContext.md) |  | [required] |

### Return type

[**crate::models::ConfigContext**](ConfigContext.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_config_contexts_read

> crate::models::ConfigContext extras_config_contexts_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this config context. | [required] |

### Return type

[**crate::models::ConfigContext**](ConfigContext.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_config_contexts_update

> crate::models::ConfigContext extras_config_contexts_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this config context. | [required] |
**data** | [**WritableConfigContext**](WritableConfigContext.md) |  | [required] |

### Return type

[**crate::models::ConfigContext**](ConfigContext.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_custom_field_choices_list

> extras_custom_field_choices_list()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_custom_field_choices_read

> extras_custom_field_choices_read(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_export_templates_create

> crate::models::ExportTemplate extras_export_templates_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableExportTemplate**](WritableExportTemplate.md) |  | [required] |

### Return type

[**crate::models::ExportTemplate**](ExportTemplate.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_export_templates_delete

> extras_export_templates_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this export template. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_export_templates_list

> crate::models::InlineResponse20038 extras_export_templates_list(content_type, name, template_language, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**template_language** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20038**](inline_response_200_38.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_export_templates_partial_update

> crate::models::ExportTemplate extras_export_templates_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this export template. | [required] |
**data** | [**WritableExportTemplate**](WritableExportTemplate.md) |  | [required] |

### Return type

[**crate::models::ExportTemplate**](ExportTemplate.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_export_templates_read

> crate::models::ExportTemplate extras_export_templates_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this export template. | [required] |

### Return type

[**crate::models::ExportTemplate**](ExportTemplate.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_export_templates_update

> crate::models::ExportTemplate extras_export_templates_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this export template. | [required] |
**data** | [**WritableExportTemplate**](WritableExportTemplate.md) |  | [required] |

### Return type

[**crate::models::ExportTemplate**](ExportTemplate.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_graphs_create

> crate::models::Graph extras_graphs_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**Graph**](Graph.md) |  | [required] |

### Return type

[**crate::models::Graph**](Graph.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_graphs_delete

> extras_graphs_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this graph. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_graphs_list

> crate::models::InlineResponse20039 extras_graphs_list(_type, name, template_language, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**template_language** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20039**](inline_response_200_39.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_graphs_partial_update

> crate::models::Graph extras_graphs_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this graph. | [required] |
**data** | [**Graph**](Graph.md) |  | [required] |

### Return type

[**crate::models::Graph**](Graph.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_graphs_read

> crate::models::Graph extras_graphs_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this graph. | [required] |

### Return type

[**crate::models::Graph**](Graph.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_graphs_update

> crate::models::Graph extras_graphs_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this graph. | [required] |
**data** | [**Graph**](Graph.md) |  | [required] |

### Return type

[**crate::models::Graph**](Graph.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_image_attachments_create

> crate::models::ImageAttachment extras_image_attachments_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**ImageAttachment**](ImageAttachment.md) |  | [required] |

### Return type

[**crate::models::ImageAttachment**](ImageAttachment.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_image_attachments_delete

> extras_image_attachments_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this image attachment. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_image_attachments_list

> crate::models::InlineResponse20040 extras_image_attachments_list(limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20040**](inline_response_200_40.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_image_attachments_partial_update

> crate::models::ImageAttachment extras_image_attachments_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this image attachment. | [required] |
**data** | [**ImageAttachment**](ImageAttachment.md) |  | [required] |

### Return type

[**crate::models::ImageAttachment**](ImageAttachment.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_image_attachments_read

> crate::models::ImageAttachment extras_image_attachments_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this image attachment. | [required] |

### Return type

[**crate::models::ImageAttachment**](ImageAttachment.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_image_attachments_update

> crate::models::ImageAttachment extras_image_attachments_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this image attachment. | [required] |
**data** | [**ImageAttachment**](ImageAttachment.md) |  | [required] |

### Return type

[**crate::models::ImageAttachment**](ImageAttachment.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_object_changes_list

> crate::models::InlineResponse20041 extras_object_changes_list(user, user_name, request_id, action, changed_object_type, changed_object_id, object_repr, q, time, limit, offset)


Retrieve a list of recent changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | Option<**String**> |  |  |
**user_name** | Option<**String**> |  |  |
**request_id** | Option<**String**> |  |  |
**action** | Option<**String**> |  |  |
**changed_object_type** | Option<**String**> |  |  |
**changed_object_id** | Option<**f32**> |  |  |
**object_repr** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**time** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20041**](inline_response_200_41.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_object_changes_read

> crate::models::ObjectChange extras_object_changes_read(id)


Retrieve a list of recent changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this object change. | [required] |

### Return type

[**crate::models::ObjectChange**](ObjectChange.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_reports_list

> extras_reports_list()


Compile all reports and their related results (if any). Result data is deferred in the list view.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_reports_read

> extras_reports_read(id)


Retrieve a single Report identified as \"<module>.<report>\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_reports_run

> extras_reports_run(id)


Run a Report and create a new ReportResult, overwriting any previous result for the Report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_scripts_list

> extras_scripts_list()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_scripts_read

> extras_scripts_read(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_tags_create

> crate::models::Tag extras_tags_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**Tag**](Tag.md) |  | [required] |

### Return type

[**crate::models::Tag**](Tag.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_tags_delete

> extras_tags_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tag. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_tags_list

> crate::models::InlineResponse20042 extras_tags_list(name, slug, q, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20042**](inline_response_200_42.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_tags_partial_update

> crate::models::Tag extras_tags_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tag. | [required] |
**data** | [**Tag**](Tag.md) |  | [required] |

### Return type

[**crate::models::Tag**](Tag.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_tags_read

> crate::models::Tag extras_tags_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tag. | [required] |

### Return type

[**crate::models::Tag**](Tag.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extras_tags_update

> crate::models::Tag extras_tags_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tag. | [required] |
**data** | [**Tag**](Tag.md) |  | [required] |

### Return type

[**crate::models::Tag**](Tag.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

