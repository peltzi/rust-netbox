# \TenancyApi

All URIs are relative to *http://localhost:32815/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tenancy_choices_list**](TenancyApi.md#tenancy_choices_list) | **get** /tenancy/_choices/ | 
[**tenancy_choices_read**](TenancyApi.md#tenancy_choices_read) | **get** /tenancy/_choices/{id}/ | 
[**tenancy_tenant_groups_create**](TenancyApi.md#tenancy_tenant_groups_create) | **post** /tenancy/tenant-groups/ | 
[**tenancy_tenant_groups_delete**](TenancyApi.md#tenancy_tenant_groups_delete) | **delete** /tenancy/tenant-groups/{id}/ | 
[**tenancy_tenant_groups_list**](TenancyApi.md#tenancy_tenant_groups_list) | **get** /tenancy/tenant-groups/ | 
[**tenancy_tenant_groups_partial_update**](TenancyApi.md#tenancy_tenant_groups_partial_update) | **patch** /tenancy/tenant-groups/{id}/ | 
[**tenancy_tenant_groups_read**](TenancyApi.md#tenancy_tenant_groups_read) | **get** /tenancy/tenant-groups/{id}/ | 
[**tenancy_tenant_groups_update**](TenancyApi.md#tenancy_tenant_groups_update) | **put** /tenancy/tenant-groups/{id}/ | 
[**tenancy_tenants_create**](TenancyApi.md#tenancy_tenants_create) | **post** /tenancy/tenants/ | 
[**tenancy_tenants_delete**](TenancyApi.md#tenancy_tenants_delete) | **delete** /tenancy/tenants/{id}/ | 
[**tenancy_tenants_list**](TenancyApi.md#tenancy_tenants_list) | **get** /tenancy/tenants/ | 
[**tenancy_tenants_partial_update**](TenancyApi.md#tenancy_tenants_partial_update) | **patch** /tenancy/tenants/{id}/ | 
[**tenancy_tenants_read**](TenancyApi.md#tenancy_tenants_read) | **get** /tenancy/tenants/{id}/ | 
[**tenancy_tenants_update**](TenancyApi.md#tenancy_tenants_update) | **put** /tenancy/tenants/{id}/ | 



## tenancy_choices_list

> tenancy_choices_list()


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


## tenancy_choices_read

> tenancy_choices_read(id)


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


## tenancy_tenant_groups_create

> crate::models::TenantGroup tenancy_tenant_groups_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**TenantGroup**](TenantGroup.md) |  | [required] |

### Return type

[**crate::models::TenantGroup**](TenantGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_delete

> tenancy_tenant_groups_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tenant group. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_list

> crate::models::InlineResponse20054 tenancy_tenant_groups_list(id, name, slug, q, id__n, id__lte, id__lt, id__gte, id__gt, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, slug__n, slug__ic, slug__nic, slug__iew, slug__niew, slug__isw, slug__nisw, slug__ie, slug__nie, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**id__n** | Option<**String**> |  |  |
**id__lte** | Option<**String**> |  |  |
**id__lt** | Option<**String**> |  |  |
**id__gte** | Option<**String**> |  |  |
**id__gt** | Option<**String**> |  |  |
**name__n** | Option<**String**> |  |  |
**name__ic** | Option<**String**> |  |  |
**name__nic** | Option<**String**> |  |  |
**name__iew** | Option<**String**> |  |  |
**name__niew** | Option<**String**> |  |  |
**name__isw** | Option<**String**> |  |  |
**name__nisw** | Option<**String**> |  |  |
**name__ie** | Option<**String**> |  |  |
**name__nie** | Option<**String**> |  |  |
**slug__n** | Option<**String**> |  |  |
**slug__ic** | Option<**String**> |  |  |
**slug__nic** | Option<**String**> |  |  |
**slug__iew** | Option<**String**> |  |  |
**slug__niew** | Option<**String**> |  |  |
**slug__isw** | Option<**String**> |  |  |
**slug__nisw** | Option<**String**> |  |  |
**slug__ie** | Option<**String**> |  |  |
**slug__nie** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20054**](inline_response_200_54.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_partial_update

> crate::models::TenantGroup tenancy_tenant_groups_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tenant group. | [required] |
**data** | [**TenantGroup**](TenantGroup.md) |  | [required] |

### Return type

[**crate::models::TenantGroup**](TenantGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_read

> crate::models::TenantGroup tenancy_tenant_groups_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tenant group. | [required] |

### Return type

[**crate::models::TenantGroup**](TenantGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_update

> crate::models::TenantGroup tenancy_tenant_groups_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tenant group. | [required] |
**data** | [**TenantGroup**](TenantGroup.md) |  | [required] |

### Return type

[**crate::models::TenantGroup**](TenantGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_create

> crate::models::Tenant tenancy_tenants_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableTenant**](WritableTenant.md) |  | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_delete

> tenancy_tenants_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tenant. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_list

> crate::models::InlineResponse20055 tenancy_tenants_list(name, slug, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, group_id, group, tag, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, slug__n, slug__ic, slug__nic, slug__iew, slug__niew, slug__isw, slug__nisw, slug__ie, slug__nie, group_id__n, group__n, tag__n, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**created__gte** | Option<**String**> |  |  |
**created__lte** | Option<**String**> |  |  |
**last_updated** | Option<**String**> |  |  |
**last_updated__gte** | Option<**String**> |  |  |
**last_updated__lte** | Option<**String**> |  |  |
**id__in** | Option<**String**> | Multiple values may be separated by commas. |  |
**q** | Option<**String**> |  |  |
**group_id** | Option<**String**> |  |  |
**group** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**name__n** | Option<**String**> |  |  |
**name__ic** | Option<**String**> |  |  |
**name__nic** | Option<**String**> |  |  |
**name__iew** | Option<**String**> |  |  |
**name__niew** | Option<**String**> |  |  |
**name__isw** | Option<**String**> |  |  |
**name__nisw** | Option<**String**> |  |  |
**name__ie** | Option<**String**> |  |  |
**name__nie** | Option<**String**> |  |  |
**slug__n** | Option<**String**> |  |  |
**slug__ic** | Option<**String**> |  |  |
**slug__nic** | Option<**String**> |  |  |
**slug__iew** | Option<**String**> |  |  |
**slug__niew** | Option<**String**> |  |  |
**slug__isw** | Option<**String**> |  |  |
**slug__nisw** | Option<**String**> |  |  |
**slug__ie** | Option<**String**> |  |  |
**slug__nie** | Option<**String**> |  |  |
**group_id__n** | Option<**String**> |  |  |
**group__n** | Option<**String**> |  |  |
**tag__n** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20055**](inline_response_200_55.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_partial_update

> crate::models::Tenant tenancy_tenants_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tenant. | [required] |
**data** | [**WritableTenant**](WritableTenant.md) |  | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_read

> crate::models::Tenant tenancy_tenants_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tenant. | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_update

> crate::models::Tenant tenancy_tenants_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this tenant. | [required] |
**data** | [**WritableTenant**](WritableTenant.md) |  | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

