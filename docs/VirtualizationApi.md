# \VirtualizationApi

All URIs are relative to *http://localhost:32815/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**virtualization_choices_list**](VirtualizationApi.md#virtualization_choices_list) | **get** /virtualization/_choices/ | 
[**virtualization_choices_read**](VirtualizationApi.md#virtualization_choices_read) | **get** /virtualization/_choices/{id}/ | 
[**virtualization_cluster_groups_create**](VirtualizationApi.md#virtualization_cluster_groups_create) | **post** /virtualization/cluster-groups/ | 
[**virtualization_cluster_groups_delete**](VirtualizationApi.md#virtualization_cluster_groups_delete) | **delete** /virtualization/cluster-groups/{id}/ | 
[**virtualization_cluster_groups_list**](VirtualizationApi.md#virtualization_cluster_groups_list) | **get** /virtualization/cluster-groups/ | 
[**virtualization_cluster_groups_partial_update**](VirtualizationApi.md#virtualization_cluster_groups_partial_update) | **patch** /virtualization/cluster-groups/{id}/ | 
[**virtualization_cluster_groups_read**](VirtualizationApi.md#virtualization_cluster_groups_read) | **get** /virtualization/cluster-groups/{id}/ | 
[**virtualization_cluster_groups_update**](VirtualizationApi.md#virtualization_cluster_groups_update) | **put** /virtualization/cluster-groups/{id}/ | 
[**virtualization_cluster_types_create**](VirtualizationApi.md#virtualization_cluster_types_create) | **post** /virtualization/cluster-types/ | 
[**virtualization_cluster_types_delete**](VirtualizationApi.md#virtualization_cluster_types_delete) | **delete** /virtualization/cluster-types/{id}/ | 
[**virtualization_cluster_types_list**](VirtualizationApi.md#virtualization_cluster_types_list) | **get** /virtualization/cluster-types/ | 
[**virtualization_cluster_types_partial_update**](VirtualizationApi.md#virtualization_cluster_types_partial_update) | **patch** /virtualization/cluster-types/{id}/ | 
[**virtualization_cluster_types_read**](VirtualizationApi.md#virtualization_cluster_types_read) | **get** /virtualization/cluster-types/{id}/ | 
[**virtualization_cluster_types_update**](VirtualizationApi.md#virtualization_cluster_types_update) | **put** /virtualization/cluster-types/{id}/ | 
[**virtualization_clusters_create**](VirtualizationApi.md#virtualization_clusters_create) | **post** /virtualization/clusters/ | 
[**virtualization_clusters_delete**](VirtualizationApi.md#virtualization_clusters_delete) | **delete** /virtualization/clusters/{id}/ | 
[**virtualization_clusters_list**](VirtualizationApi.md#virtualization_clusters_list) | **get** /virtualization/clusters/ | 
[**virtualization_clusters_partial_update**](VirtualizationApi.md#virtualization_clusters_partial_update) | **patch** /virtualization/clusters/{id}/ | 
[**virtualization_clusters_read**](VirtualizationApi.md#virtualization_clusters_read) | **get** /virtualization/clusters/{id}/ | 
[**virtualization_clusters_update**](VirtualizationApi.md#virtualization_clusters_update) | **put** /virtualization/clusters/{id}/ | 
[**virtualization_interfaces_create**](VirtualizationApi.md#virtualization_interfaces_create) | **post** /virtualization/interfaces/ | 
[**virtualization_interfaces_delete**](VirtualizationApi.md#virtualization_interfaces_delete) | **delete** /virtualization/interfaces/{id}/ | 
[**virtualization_interfaces_list**](VirtualizationApi.md#virtualization_interfaces_list) | **get** /virtualization/interfaces/ | 
[**virtualization_interfaces_partial_update**](VirtualizationApi.md#virtualization_interfaces_partial_update) | **patch** /virtualization/interfaces/{id}/ | 
[**virtualization_interfaces_read**](VirtualizationApi.md#virtualization_interfaces_read) | **get** /virtualization/interfaces/{id}/ | 
[**virtualization_interfaces_update**](VirtualizationApi.md#virtualization_interfaces_update) | **put** /virtualization/interfaces/{id}/ | 
[**virtualization_virtual_machines_create**](VirtualizationApi.md#virtualization_virtual_machines_create) | **post** /virtualization/virtual-machines/ | 
[**virtualization_virtual_machines_delete**](VirtualizationApi.md#virtualization_virtual_machines_delete) | **delete** /virtualization/virtual-machines/{id}/ | 
[**virtualization_virtual_machines_list**](VirtualizationApi.md#virtualization_virtual_machines_list) | **get** /virtualization/virtual-machines/ | 
[**virtualization_virtual_machines_partial_update**](VirtualizationApi.md#virtualization_virtual_machines_partial_update) | **patch** /virtualization/virtual-machines/{id}/ | 
[**virtualization_virtual_machines_read**](VirtualizationApi.md#virtualization_virtual_machines_read) | **get** /virtualization/virtual-machines/{id}/ | 
[**virtualization_virtual_machines_update**](VirtualizationApi.md#virtualization_virtual_machines_update) | **put** /virtualization/virtual-machines/{id}/ | 



## virtualization_choices_list

> virtualization_choices_list()


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


## virtualization_choices_read

> virtualization_choices_read(id)


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


## virtualization_cluster_groups_create

> crate::models::ClusterGroup virtualization_cluster_groups_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**ClusterGroup**](ClusterGroup.md) |  | [required] |

### Return type

[**crate::models::ClusterGroup**](ClusterGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_cluster_groups_delete

> virtualization_cluster_groups_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster group. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_cluster_groups_list

> crate::models::InlineResponse20056 virtualization_cluster_groups_list(id, name, slug, q, id__n, id__lte, id__lt, id__gte, id__gt, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, slug__n, slug__ic, slug__nic, slug__iew, slug__niew, slug__isw, slug__nisw, slug__ie, slug__nie, limit, offset)


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

[**crate::models::InlineResponse20056**](inline_response_200_56.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_cluster_groups_partial_update

> crate::models::ClusterGroup virtualization_cluster_groups_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster group. | [required] |
**data** | [**ClusterGroup**](ClusterGroup.md) |  | [required] |

### Return type

[**crate::models::ClusterGroup**](ClusterGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_cluster_groups_read

> crate::models::ClusterGroup virtualization_cluster_groups_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster group. | [required] |

### Return type

[**crate::models::ClusterGroup**](ClusterGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_cluster_groups_update

> crate::models::ClusterGroup virtualization_cluster_groups_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster group. | [required] |
**data** | [**ClusterGroup**](ClusterGroup.md) |  | [required] |

### Return type

[**crate::models::ClusterGroup**](ClusterGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_cluster_types_create

> crate::models::ClusterType virtualization_cluster_types_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**ClusterType**](ClusterType.md) |  | [required] |

### Return type

[**crate::models::ClusterType**](ClusterType.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_cluster_types_delete

> virtualization_cluster_types_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster type. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_cluster_types_list

> crate::models::InlineResponse20057 virtualization_cluster_types_list(id, name, slug, q, id__n, id__lte, id__lt, id__gte, id__gt, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, slug__n, slug__ic, slug__nic, slug__iew, slug__niew, slug__isw, slug__nisw, slug__ie, slug__nie, limit, offset)


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

[**crate::models::InlineResponse20057**](inline_response_200_57.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_cluster_types_partial_update

> crate::models::ClusterType virtualization_cluster_types_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster type. | [required] |
**data** | [**ClusterType**](ClusterType.md) |  | [required] |

### Return type

[**crate::models::ClusterType**](ClusterType.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_cluster_types_read

> crate::models::ClusterType virtualization_cluster_types_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster type. | [required] |

### Return type

[**crate::models::ClusterType**](ClusterType.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_cluster_types_update

> crate::models::ClusterType virtualization_cluster_types_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster type. | [required] |
**data** | [**ClusterType**](ClusterType.md) |  | [required] |

### Return type

[**crate::models::ClusterType**](ClusterType.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_clusters_create

> crate::models::Cluster virtualization_clusters_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableCluster**](WritableCluster.md) |  | [required] |

### Return type

[**crate::models::Cluster**](Cluster.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_clusters_delete

> virtualization_clusters_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_clusters_list

> crate::models::InlineResponse20058 virtualization_clusters_list(name, tenant_group_id, tenant_group, tenant_id, tenant, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, region_id, region, site_id, site, group_id, group, type_id, _type, tag, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, tenant_group_id__n, tenant_group__n, tenant_id__n, tenant__n, region_id__n, region__n, site_id__n, site__n, group_id__n, group__n, type_id__n, type__n, tag__n, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**tenant_group_id** | Option<**String**> |  |  |
**tenant_group** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**tenant** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**created__gte** | Option<**String**> |  |  |
**created__lte** | Option<**String**> |  |  |
**last_updated** | Option<**String**> |  |  |
**last_updated__gte** | Option<**String**> |  |  |
**last_updated__lte** | Option<**String**> |  |  |
**id__in** | Option<**String**> | Multiple values may be separated by commas. |  |
**q** | Option<**String**> |  |  |
**region_id** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |
**site_id** | Option<**String**> |  |  |
**site** | Option<**String**> |  |  |
**group_id** | Option<**String**> |  |  |
**group** | Option<**String**> |  |  |
**type_id** | Option<**String**> |  |  |
**_type** | Option<**String**> |  |  |
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
**tenant_group_id__n** | Option<**String**> |  |  |
**tenant_group__n** | Option<**String**> |  |  |
**tenant_id__n** | Option<**String**> |  |  |
**tenant__n** | Option<**String**> |  |  |
**region_id__n** | Option<**String**> |  |  |
**region__n** | Option<**String**> |  |  |
**site_id__n** | Option<**String**> |  |  |
**site__n** | Option<**String**> |  |  |
**group_id__n** | Option<**String**> |  |  |
**group__n** | Option<**String**> |  |  |
**type_id__n** | Option<**String**> |  |  |
**type__n** | Option<**String**> |  |  |
**tag__n** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20058**](inline_response_200_58.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_clusters_partial_update

> crate::models::Cluster virtualization_clusters_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster. | [required] |
**data** | [**WritableCluster**](WritableCluster.md) |  | [required] |

### Return type

[**crate::models::Cluster**](Cluster.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_clusters_read

> crate::models::Cluster virtualization_clusters_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster. | [required] |

### Return type

[**crate::models::Cluster**](Cluster.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_clusters_update

> crate::models::Cluster virtualization_clusters_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this cluster. | [required] |
**data** | [**WritableCluster**](WritableCluster.md) |  | [required] |

### Return type

[**crate::models::Cluster**](Cluster.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_interfaces_create

> crate::models::VirtualMachineInterface virtualization_interfaces_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableVirtualMachineInterface**](WritableVirtualMachineInterface.md) |  | [required] |

### Return type

[**crate::models::VirtualMachineInterface**](VirtualMachineInterface.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_interfaces_delete

> virtualization_interfaces_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this interface. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_interfaces_list

> crate::models::InlineResponse20059 virtualization_interfaces_list(id, name, enabled, mtu, q, virtual_machine_id, virtual_machine, mac_address, id__n, id__lte, id__lt, id__gte, id__gt, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, mtu__n, mtu__lte, mtu__lt, mtu__gte, mtu__gt, virtual_machine_id__n, virtual_machine__n, mac_address__n, mac_address__ic, mac_address__nic, mac_address__iew, mac_address__niew, mac_address__isw, mac_address__nisw, mac_address__ie, mac_address__nie, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**enabled** | Option<**String**> |  |  |
**mtu** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**virtual_machine_id** | Option<**String**> |  |  |
**virtual_machine** | Option<**String**> |  |  |
**mac_address** | Option<**String**> |  |  |
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
**mtu__n** | Option<**String**> |  |  |
**mtu__lte** | Option<**String**> |  |  |
**mtu__lt** | Option<**String**> |  |  |
**mtu__gte** | Option<**String**> |  |  |
**mtu__gt** | Option<**String**> |  |  |
**virtual_machine_id__n** | Option<**String**> |  |  |
**virtual_machine__n** | Option<**String**> |  |  |
**mac_address__n** | Option<**String**> |  |  |
**mac_address__ic** | Option<**String**> |  |  |
**mac_address__nic** | Option<**String**> |  |  |
**mac_address__iew** | Option<**String**> |  |  |
**mac_address__niew** | Option<**String**> |  |  |
**mac_address__isw** | Option<**String**> |  |  |
**mac_address__nisw** | Option<**String**> |  |  |
**mac_address__ie** | Option<**String**> |  |  |
**mac_address__nie** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20059**](inline_response_200_59.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_interfaces_partial_update

> crate::models::VirtualMachineInterface virtualization_interfaces_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this interface. | [required] |
**data** | [**WritableVirtualMachineInterface**](WritableVirtualMachineInterface.md) |  | [required] |

### Return type

[**crate::models::VirtualMachineInterface**](VirtualMachineInterface.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_interfaces_read

> crate::models::VirtualMachineInterface virtualization_interfaces_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this interface. | [required] |

### Return type

[**crate::models::VirtualMachineInterface**](VirtualMachineInterface.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_interfaces_update

> crate::models::VirtualMachineInterface virtualization_interfaces_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this interface. | [required] |
**data** | [**WritableVirtualMachineInterface**](WritableVirtualMachineInterface.md) |  | [required] |

### Return type

[**crate::models::VirtualMachineInterface**](VirtualMachineInterface.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_virtual_machines_create

> crate::models::VirtualMachineWithConfigContext virtualization_virtual_machines_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableVirtualMachineWithConfigContext**](WritableVirtualMachineWithConfigContext.md) |  | [required] |

### Return type

[**crate::models::VirtualMachineWithConfigContext**](VirtualMachineWithConfigContext.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_virtual_machines_delete

> virtualization_virtual_machines_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_virtual_machines_list

> crate::models::InlineResponse20060 virtualization_virtual_machines_list(id, name, cluster, vcpus, memory, disk, local_context_data, tenant_group_id, tenant_group, tenant_id, tenant, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, status, cluster_group_id, cluster_group, cluster_type_id, cluster_type, cluster_id, region_id, region, site_id, site, role_id, role, platform_id, platform, mac_address, tag, id__n, id__lte, id__lt, id__gte, id__gt, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, cluster__n, vcpus__n, vcpus__lte, vcpus__lt, vcpus__gte, vcpus__gt, memory__n, memory__lte, memory__lt, memory__gte, memory__gt, disk__n, disk__lte, disk__lt, disk__gte, disk__gt, tenant_group_id__n, tenant_group__n, tenant_id__n, tenant__n, status__n, cluster_group_id__n, cluster_group__n, cluster_type_id__n, cluster_type__n, cluster_id__n, region_id__n, region__n, site_id__n, site__n, role_id__n, role__n, platform_id__n, platform__n, mac_address__n, mac_address__ic, mac_address__nic, mac_address__iew, mac_address__niew, mac_address__isw, mac_address__nisw, mac_address__ie, mac_address__nie, tag__n, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**cluster** | Option<**String**> |  |  |
**vcpus** | Option<**String**> |  |  |
**memory** | Option<**String**> |  |  |
**disk** | Option<**String**> |  |  |
**local_context_data** | Option<**String**> |  |  |
**tenant_group_id** | Option<**String**> |  |  |
**tenant_group** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**tenant** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**created__gte** | Option<**String**> |  |  |
**created__lte** | Option<**String**> |  |  |
**last_updated** | Option<**String**> |  |  |
**last_updated__gte** | Option<**String**> |  |  |
**last_updated__lte** | Option<**String**> |  |  |
**id__in** | Option<**String**> | Multiple values may be separated by commas. |  |
**q** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**cluster_group_id** | Option<**String**> |  |  |
**cluster_group** | Option<**String**> |  |  |
**cluster_type_id** | Option<**String**> |  |  |
**cluster_type** | Option<**String**> |  |  |
**cluster_id** | Option<**String**> |  |  |
**region_id** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |
**site_id** | Option<**String**> |  |  |
**site** | Option<**String**> |  |  |
**role_id** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**platform_id** | Option<**String**> |  |  |
**platform** | Option<**String**> |  |  |
**mac_address** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
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
**cluster__n** | Option<**String**> |  |  |
**vcpus__n** | Option<**String**> |  |  |
**vcpus__lte** | Option<**String**> |  |  |
**vcpus__lt** | Option<**String**> |  |  |
**vcpus__gte** | Option<**String**> |  |  |
**vcpus__gt** | Option<**String**> |  |  |
**memory__n** | Option<**String**> |  |  |
**memory__lte** | Option<**String**> |  |  |
**memory__lt** | Option<**String**> |  |  |
**memory__gte** | Option<**String**> |  |  |
**memory__gt** | Option<**String**> |  |  |
**disk__n** | Option<**String**> |  |  |
**disk__lte** | Option<**String**> |  |  |
**disk__lt** | Option<**String**> |  |  |
**disk__gte** | Option<**String**> |  |  |
**disk__gt** | Option<**String**> |  |  |
**tenant_group_id__n** | Option<**String**> |  |  |
**tenant_group__n** | Option<**String**> |  |  |
**tenant_id__n** | Option<**String**> |  |  |
**tenant__n** | Option<**String**> |  |  |
**status__n** | Option<**String**> |  |  |
**cluster_group_id__n** | Option<**String**> |  |  |
**cluster_group__n** | Option<**String**> |  |  |
**cluster_type_id__n** | Option<**String**> |  |  |
**cluster_type__n** | Option<**String**> |  |  |
**cluster_id__n** | Option<**String**> |  |  |
**region_id__n** | Option<**String**> |  |  |
**region__n** | Option<**String**> |  |  |
**site_id__n** | Option<**String**> |  |  |
**site__n** | Option<**String**> |  |  |
**role_id__n** | Option<**String**> |  |  |
**role__n** | Option<**String**> |  |  |
**platform_id__n** | Option<**String**> |  |  |
**platform__n** | Option<**String**> |  |  |
**mac_address__n** | Option<**String**> |  |  |
**mac_address__ic** | Option<**String**> |  |  |
**mac_address__nic** | Option<**String**> |  |  |
**mac_address__iew** | Option<**String**> |  |  |
**mac_address__niew** | Option<**String**> |  |  |
**mac_address__isw** | Option<**String**> |  |  |
**mac_address__nisw** | Option<**String**> |  |  |
**mac_address__ie** | Option<**String**> |  |  |
**mac_address__nie** | Option<**String**> |  |  |
**tag__n** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20060**](inline_response_200_60.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_virtual_machines_partial_update

> crate::models::VirtualMachineWithConfigContext virtualization_virtual_machines_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**data** | [**WritableVirtualMachineWithConfigContext**](WritableVirtualMachineWithConfigContext.md) |  | [required] |

### Return type

[**crate::models::VirtualMachineWithConfigContext**](VirtualMachineWithConfigContext.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_virtual_machines_read

> crate::models::VirtualMachineWithConfigContext virtualization_virtual_machines_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |

### Return type

[**crate::models::VirtualMachineWithConfigContext**](VirtualMachineWithConfigContext.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## virtualization_virtual_machines_update

> crate::models::VirtualMachineWithConfigContext virtualization_virtual_machines_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this virtual machine. | [required] |
**data** | [**WritableVirtualMachineWithConfigContext**](WritableVirtualMachineWithConfigContext.md) |  | [required] |

### Return type

[**crate::models::VirtualMachineWithConfigContext**](VirtualMachineWithConfigContext.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

