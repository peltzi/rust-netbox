# \IpamApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ipam_aggregates_create**](IpamApi.md#ipam_aggregates_create) | **post** /ipam/aggregates/ | 
[**ipam_aggregates_delete**](IpamApi.md#ipam_aggregates_delete) | **delete** /ipam/aggregates/{id}/ | 
[**ipam_aggregates_list**](IpamApi.md#ipam_aggregates_list) | **get** /ipam/aggregates/ | 
[**ipam_aggregates_partial_update**](IpamApi.md#ipam_aggregates_partial_update) | **patch** /ipam/aggregates/{id}/ | 
[**ipam_aggregates_read**](IpamApi.md#ipam_aggregates_read) | **get** /ipam/aggregates/{id}/ | 
[**ipam_aggregates_update**](IpamApi.md#ipam_aggregates_update) | **put** /ipam/aggregates/{id}/ | 
[**ipam_choices_list**](IpamApi.md#ipam_choices_list) | **get** /ipam/_choices/ | 
[**ipam_choices_read**](IpamApi.md#ipam_choices_read) | **get** /ipam/_choices/{id}/ | 
[**ipam_ip_addresses_create**](IpamApi.md#ipam_ip_addresses_create) | **post** /ipam/ip-addresses/ | 
[**ipam_ip_addresses_delete**](IpamApi.md#ipam_ip_addresses_delete) | **delete** /ipam/ip-addresses/{id}/ | 
[**ipam_ip_addresses_list**](IpamApi.md#ipam_ip_addresses_list) | **get** /ipam/ip-addresses/ | 
[**ipam_ip_addresses_partial_update**](IpamApi.md#ipam_ip_addresses_partial_update) | **patch** /ipam/ip-addresses/{id}/ | 
[**ipam_ip_addresses_read**](IpamApi.md#ipam_ip_addresses_read) | **get** /ipam/ip-addresses/{id}/ | 
[**ipam_ip_addresses_update**](IpamApi.md#ipam_ip_addresses_update) | **put** /ipam/ip-addresses/{id}/ | 
[**ipam_prefixes_available_ips_create**](IpamApi.md#ipam_prefixes_available_ips_create) | **post** /ipam/prefixes/{id}/available-ips/ | 
[**ipam_prefixes_available_ips_read**](IpamApi.md#ipam_prefixes_available_ips_read) | **get** /ipam/prefixes/{id}/available-ips/ | 
[**ipam_prefixes_available_prefixes_create**](IpamApi.md#ipam_prefixes_available_prefixes_create) | **post** /ipam/prefixes/{id}/available-prefixes/ | A convenience method for returning available child prefixes within a parent.
[**ipam_prefixes_available_prefixes_read**](IpamApi.md#ipam_prefixes_available_prefixes_read) | **get** /ipam/prefixes/{id}/available-prefixes/ | A convenience method for returning available child prefixes within a parent.
[**ipam_prefixes_create**](IpamApi.md#ipam_prefixes_create) | **post** /ipam/prefixes/ | 
[**ipam_prefixes_delete**](IpamApi.md#ipam_prefixes_delete) | **delete** /ipam/prefixes/{id}/ | 
[**ipam_prefixes_list**](IpamApi.md#ipam_prefixes_list) | **get** /ipam/prefixes/ | 
[**ipam_prefixes_partial_update**](IpamApi.md#ipam_prefixes_partial_update) | **patch** /ipam/prefixes/{id}/ | 
[**ipam_prefixes_read**](IpamApi.md#ipam_prefixes_read) | **get** /ipam/prefixes/{id}/ | 
[**ipam_prefixes_update**](IpamApi.md#ipam_prefixes_update) | **put** /ipam/prefixes/{id}/ | 
[**ipam_rirs_create**](IpamApi.md#ipam_rirs_create) | **post** /ipam/rirs/ | 
[**ipam_rirs_delete**](IpamApi.md#ipam_rirs_delete) | **delete** /ipam/rirs/{id}/ | 
[**ipam_rirs_list**](IpamApi.md#ipam_rirs_list) | **get** /ipam/rirs/ | 
[**ipam_rirs_partial_update**](IpamApi.md#ipam_rirs_partial_update) | **patch** /ipam/rirs/{id}/ | 
[**ipam_rirs_read**](IpamApi.md#ipam_rirs_read) | **get** /ipam/rirs/{id}/ | 
[**ipam_rirs_update**](IpamApi.md#ipam_rirs_update) | **put** /ipam/rirs/{id}/ | 
[**ipam_roles_create**](IpamApi.md#ipam_roles_create) | **post** /ipam/roles/ | 
[**ipam_roles_delete**](IpamApi.md#ipam_roles_delete) | **delete** /ipam/roles/{id}/ | 
[**ipam_roles_list**](IpamApi.md#ipam_roles_list) | **get** /ipam/roles/ | 
[**ipam_roles_partial_update**](IpamApi.md#ipam_roles_partial_update) | **patch** /ipam/roles/{id}/ | 
[**ipam_roles_read**](IpamApi.md#ipam_roles_read) | **get** /ipam/roles/{id}/ | 
[**ipam_roles_update**](IpamApi.md#ipam_roles_update) | **put** /ipam/roles/{id}/ | 
[**ipam_services_create**](IpamApi.md#ipam_services_create) | **post** /ipam/services/ | 
[**ipam_services_delete**](IpamApi.md#ipam_services_delete) | **delete** /ipam/services/{id}/ | 
[**ipam_services_list**](IpamApi.md#ipam_services_list) | **get** /ipam/services/ | 
[**ipam_services_partial_update**](IpamApi.md#ipam_services_partial_update) | **patch** /ipam/services/{id}/ | 
[**ipam_services_read**](IpamApi.md#ipam_services_read) | **get** /ipam/services/{id}/ | 
[**ipam_services_update**](IpamApi.md#ipam_services_update) | **put** /ipam/services/{id}/ | 
[**ipam_vlan_groups_create**](IpamApi.md#ipam_vlan_groups_create) | **post** /ipam/vlan-groups/ | 
[**ipam_vlan_groups_delete**](IpamApi.md#ipam_vlan_groups_delete) | **delete** /ipam/vlan-groups/{id}/ | 
[**ipam_vlan_groups_list**](IpamApi.md#ipam_vlan_groups_list) | **get** /ipam/vlan-groups/ | 
[**ipam_vlan_groups_partial_update**](IpamApi.md#ipam_vlan_groups_partial_update) | **patch** /ipam/vlan-groups/{id}/ | 
[**ipam_vlan_groups_read**](IpamApi.md#ipam_vlan_groups_read) | **get** /ipam/vlan-groups/{id}/ | 
[**ipam_vlan_groups_update**](IpamApi.md#ipam_vlan_groups_update) | **put** /ipam/vlan-groups/{id}/ | 
[**ipam_vlans_create**](IpamApi.md#ipam_vlans_create) | **post** /ipam/vlans/ | 
[**ipam_vlans_delete**](IpamApi.md#ipam_vlans_delete) | **delete** /ipam/vlans/{id}/ | 
[**ipam_vlans_list**](IpamApi.md#ipam_vlans_list) | **get** /ipam/vlans/ | 
[**ipam_vlans_partial_update**](IpamApi.md#ipam_vlans_partial_update) | **patch** /ipam/vlans/{id}/ | 
[**ipam_vlans_read**](IpamApi.md#ipam_vlans_read) | **get** /ipam/vlans/{id}/ | 
[**ipam_vlans_update**](IpamApi.md#ipam_vlans_update) | **put** /ipam/vlans/{id}/ | 
[**ipam_vrfs_create**](IpamApi.md#ipam_vrfs_create) | **post** /ipam/vrfs/ | 
[**ipam_vrfs_delete**](IpamApi.md#ipam_vrfs_delete) | **delete** /ipam/vrfs/{id}/ | 
[**ipam_vrfs_list**](IpamApi.md#ipam_vrfs_list) | **get** /ipam/vrfs/ | 
[**ipam_vrfs_partial_update**](IpamApi.md#ipam_vrfs_partial_update) | **patch** /ipam/vrfs/{id}/ | 
[**ipam_vrfs_read**](IpamApi.md#ipam_vrfs_read) | **get** /ipam/vrfs/{id}/ | 
[**ipam_vrfs_update**](IpamApi.md#ipam_vrfs_update) | **put** /ipam/vrfs/{id}/ | 



## ipam_aggregates_create

> crate::models::Aggregate ipam_aggregates_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableAggregate**](WritableAggregate.md) |  | [required] |

### Return type

[**crate::models::Aggregate**](Aggregate.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_aggregates_delete

> ipam_aggregates_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this aggregate. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_aggregates_list

> crate::models::InlineResponse20043 ipam_aggregates_list(family, date_added, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, prefix, rir_id, rir, tag, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**family** | Option<**String**> |  |  |
**date_added** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**created__gte** | Option<**String**> |  |  |
**created__lte** | Option<**String**> |  |  |
**last_updated** | Option<**String**> |  |  |
**last_updated__gte** | Option<**String**> |  |  |
**last_updated__lte** | Option<**String**> |  |  |
**id__in** | Option<**String**> | Multiple values may be separated by commas. |  |
**q** | Option<**String**> |  |  |
**prefix** | Option<**String**> |  |  |
**rir_id** | Option<**String**> |  |  |
**rir** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20043**](inline_response_200_43.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_aggregates_partial_update

> crate::models::Aggregate ipam_aggregates_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this aggregate. | [required] |
**data** | [**WritableAggregate**](WritableAggregate.md) |  | [required] |

### Return type

[**crate::models::Aggregate**](Aggregate.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_aggregates_read

> crate::models::Aggregate ipam_aggregates_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this aggregate. | [required] |

### Return type

[**crate::models::Aggregate**](Aggregate.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_aggregates_update

> crate::models::Aggregate ipam_aggregates_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this aggregate. | [required] |
**data** | [**WritableAggregate**](WritableAggregate.md) |  | [required] |

### Return type

[**crate::models::Aggregate**](Aggregate.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_choices_list

> ipam_choices_list()


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


## ipam_choices_read

> ipam_choices_read(id)


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


## ipam_ip_addresses_create

> crate::models::IpAddress ipam_ip_addresses_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableIpAddress**](WritableIpAddress.md) |  | [required] |

### Return type

[**crate::models::IpAddress**](IPAddress.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_ip_addresses_delete

> ipam_ip_addresses_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this IP address. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_ip_addresses_list

> crate::models::InlineResponse20044 ipam_ip_addresses_list(family, dns_name, tenant_group_id, tenant_group, tenant_id, tenant, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, parent, address, mask_length, vrf_id, vrf, device, device_id, virtual_machine_id, virtual_machine, interface, interface_id, assigned_to_interface, status, role, tag, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**family** | Option<**String**> |  |  |
**dns_name** | Option<**String**> |  |  |
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
**parent** | Option<**String**> |  |  |
**address** | Option<**String**> |  |  |
**mask_length** | Option<**f32**> |  |  |
**vrf_id** | Option<**String**> |  |  |
**vrf** | Option<**String**> |  |  |
**device** | Option<**String**> |  |  |
**device_id** | Option<**String**> |  |  |
**virtual_machine_id** | Option<**String**> |  |  |
**virtual_machine** | Option<**String**> |  |  |
**interface** | Option<**String**> |  |  |
**interface_id** | Option<**String**> |  |  |
**assigned_to_interface** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20044**](inline_response_200_44.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_ip_addresses_partial_update

> crate::models::IpAddress ipam_ip_addresses_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this IP address. | [required] |
**data** | [**WritableIpAddress**](WritableIpAddress.md) |  | [required] |

### Return type

[**crate::models::IpAddress**](IPAddress.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_ip_addresses_read

> crate::models::IpAddress ipam_ip_addresses_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this IP address. | [required] |

### Return type

[**crate::models::IpAddress**](IPAddress.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_ip_addresses_update

> crate::models::IpAddress ipam_ip_addresses_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this IP address. | [required] |
**data** | [**WritableIpAddress**](WritableIpAddress.md) |  | [required] |

### Return type

[**crate::models::IpAddress**](IPAddress.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_prefixes_available_ips_create

> crate::models::Prefix ipam_prefixes_available_ips_create(id, data)


A convenience method for returning available IP addresses within a prefix. By default, the number of IPs returned will be equivalent to PAGINATE_COUNT. An arbitrary limit (up to MAX_PAGE_SIZE, if set) may be passed, however results will not be paginated.  The advisory lock decorator uses a PostgreSQL advisory lock to prevent this API from being invoked in parallel, which results in a race condition where multiple insertions can occur.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this prefix. | [required] |
**data** | [**WritablePrefix**](WritablePrefix.md) |  | [required] |

### Return type

[**crate::models::Prefix**](Prefix.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_prefixes_available_ips_read

> crate::models::Prefix ipam_prefixes_available_ips_read(id)


A convenience method for returning available IP addresses within a prefix. By default, the number of IPs returned will be equivalent to PAGINATE_COUNT. An arbitrary limit (up to MAX_PAGE_SIZE, if set) may be passed, however results will not be paginated.  The advisory lock decorator uses a PostgreSQL advisory lock to prevent this API from being invoked in parallel, which results in a race condition where multiple insertions can occur.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this prefix. | [required] |

### Return type

[**crate::models::Prefix**](Prefix.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_prefixes_available_prefixes_create

> crate::models::Prefix ipam_prefixes_available_prefixes_create(id, data)
A convenience method for returning available child prefixes within a parent.

The advisory lock decorator uses a PostgreSQL advisory lock to prevent this API from being invoked in parallel, which results in a race condition where multiple insertions can occur.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this prefix. | [required] |
**data** | [**WritablePrefix**](WritablePrefix.md) |  | [required] |

### Return type

[**crate::models::Prefix**](Prefix.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_prefixes_available_prefixes_read

> crate::models::Prefix ipam_prefixes_available_prefixes_read(id)
A convenience method for returning available child prefixes within a parent.

The advisory lock decorator uses a PostgreSQL advisory lock to prevent this API from being invoked in parallel, which results in a race condition where multiple insertions can occur.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this prefix. | [required] |

### Return type

[**crate::models::Prefix**](Prefix.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_prefixes_create

> crate::models::Prefix ipam_prefixes_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritablePrefix**](WritablePrefix.md) |  | [required] |

### Return type

[**crate::models::Prefix**](Prefix.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_prefixes_delete

> ipam_prefixes_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this prefix. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_prefixes_list

> crate::models::InlineResponse20045 ipam_prefixes_list(family, is_pool, tenant_group_id, tenant_group, tenant_id, tenant, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, prefix, within, within_include, contains, mask_length, vrf_id, vrf, region_id, region, site_id, site, vlan_id, vlan_vid, role_id, role, status, tag, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**family** | Option<**String**> |  |  |
**is_pool** | Option<**String**> |  |  |
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
**prefix** | Option<**String**> |  |  |
**within** | Option<**String**> |  |  |
**within_include** | Option<**String**> |  |  |
**contains** | Option<**String**> |  |  |
**mask_length** | Option<**f32**> |  |  |
**vrf_id** | Option<**String**> |  |  |
**vrf** | Option<**String**> |  |  |
**region_id** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |
**site_id** | Option<**String**> |  |  |
**site** | Option<**String**> |  |  |
**vlan_id** | Option<**String**> |  |  |
**vlan_vid** | Option<**f32**> |  |  |
**role_id** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20045**](inline_response_200_45.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_prefixes_partial_update

> crate::models::Prefix ipam_prefixes_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this prefix. | [required] |
**data** | [**WritablePrefix**](WritablePrefix.md) |  | [required] |

### Return type

[**crate::models::Prefix**](Prefix.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_prefixes_read

> crate::models::Prefix ipam_prefixes_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this prefix. | [required] |

### Return type

[**crate::models::Prefix**](Prefix.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_prefixes_update

> crate::models::Prefix ipam_prefixes_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this prefix. | [required] |
**data** | [**WritablePrefix**](WritablePrefix.md) |  | [required] |

### Return type

[**crate::models::Prefix**](Prefix.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_rirs_create

> crate::models::Rir ipam_rirs_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**Rir**](Rir.md) |  | [required] |

### Return type

[**crate::models::Rir**](RIR.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_rirs_delete

> ipam_rirs_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this RIR. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_rirs_list

> crate::models::InlineResponse20046 ipam_rirs_list(name, slug, is_private, q, id__in, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |
**is_private** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**id__in** | Option<**String**> | Multiple values may be separated by commas. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20046**](inline_response_200_46.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_rirs_partial_update

> crate::models::Rir ipam_rirs_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this RIR. | [required] |
**data** | [**Rir**](Rir.md) |  | [required] |

### Return type

[**crate::models::Rir**](RIR.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_rirs_read

> crate::models::Rir ipam_rirs_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this RIR. | [required] |

### Return type

[**crate::models::Rir**](RIR.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_rirs_update

> crate::models::Rir ipam_rirs_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this RIR. | [required] |
**data** | [**Rir**](Rir.md) |  | [required] |

### Return type

[**crate::models::Rir**](RIR.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_roles_create

> crate::models::Role ipam_roles_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**Role**](Role.md) |  | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_roles_delete

> ipam_roles_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this role. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_roles_list

> crate::models::InlineResponse20047 ipam_roles_list(id, name, slug, q, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20047**](inline_response_200_47.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_roles_partial_update

> crate::models::Role ipam_roles_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this role. | [required] |
**data** | [**Role**](Role.md) |  | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_roles_read

> crate::models::Role ipam_roles_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this role. | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_roles_update

> crate::models::Role ipam_roles_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this role. | [required] |
**data** | [**Role**](Role.md) |  | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_services_create

> crate::models::Service ipam_services_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableService**](WritableService.md) |  | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_services_delete

> ipam_services_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this service. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_services_list

> crate::models::InlineResponse20048 ipam_services_list(id, name, protocol, port, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, q, device_id, device, virtual_machine_id, virtual_machine, tag, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**protocol** | Option<**String**> |  |  |
**port** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**created__gte** | Option<**String**> |  |  |
**created__lte** | Option<**String**> |  |  |
**last_updated** | Option<**String**> |  |  |
**last_updated__gte** | Option<**String**> |  |  |
**last_updated__lte** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**device_id** | Option<**String**> |  |  |
**device** | Option<**String**> |  |  |
**virtual_machine_id** | Option<**String**> |  |  |
**virtual_machine** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20048**](inline_response_200_48.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_services_partial_update

> crate::models::Service ipam_services_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this service. | [required] |
**data** | [**WritableService**](WritableService.md) |  | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_services_read

> crate::models::Service ipam_services_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this service. | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_services_update

> crate::models::Service ipam_services_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this service. | [required] |
**data** | [**WritableService**](WritableService.md) |  | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlan_groups_create

> crate::models::VlanGroup ipam_vlan_groups_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableVlanGroup**](WritableVlanGroup.md) |  | [required] |

### Return type

[**crate::models::VlanGroup**](VLANGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlan_groups_delete

> ipam_vlan_groups_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VLAN group. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlan_groups_list

> crate::models::InlineResponse20049 ipam_vlan_groups_list(id, name, slug, q, region_id, region, site_id, site, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**region_id** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |
**site_id** | Option<**String**> |  |  |
**site** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20049**](inline_response_200_49.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlan_groups_partial_update

> crate::models::VlanGroup ipam_vlan_groups_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VLAN group. | [required] |
**data** | [**WritableVlanGroup**](WritableVlanGroup.md) |  | [required] |

### Return type

[**crate::models::VlanGroup**](VLANGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlan_groups_read

> crate::models::VlanGroup ipam_vlan_groups_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VLAN group. | [required] |

### Return type

[**crate::models::VlanGroup**](VLANGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlan_groups_update

> crate::models::VlanGroup ipam_vlan_groups_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VLAN group. | [required] |
**data** | [**WritableVlanGroup**](WritableVlanGroup.md) |  | [required] |

### Return type

[**crate::models::VlanGroup**](VLANGroup.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlans_create

> crate::models::Vlan ipam_vlans_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableVlan**](WritableVlan.md) |  | [required] |

### Return type

[**crate::models::Vlan**](VLAN.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlans_delete

> ipam_vlans_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VLAN. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlans_list

> crate::models::InlineResponse20050 ipam_vlans_list(vid, name, tenant_group_id, tenant_group, tenant_id, tenant, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, region_id, region, site_id, site, group_id, group, role_id, role, status, tag, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vid** | Option<**String**> |  |  |
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
**role_id** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20050**](inline_response_200_50.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlans_partial_update

> crate::models::Vlan ipam_vlans_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VLAN. | [required] |
**data** | [**WritableVlan**](WritableVlan.md) |  | [required] |

### Return type

[**crate::models::Vlan**](VLAN.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlans_read

> crate::models::Vlan ipam_vlans_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VLAN. | [required] |

### Return type

[**crate::models::Vlan**](VLAN.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vlans_update

> crate::models::Vlan ipam_vlans_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VLAN. | [required] |
**data** | [**WritableVlan**](WritableVlan.md) |  | [required] |

### Return type

[**crate::models::Vlan**](VLAN.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vrfs_create

> crate::models::Vrf ipam_vrfs_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableVrf**](WritableVrf.md) |  | [required] |

### Return type

[**crate::models::Vrf**](VRF.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vrfs_delete

> ipam_vrfs_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VRF. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vrfs_list

> crate::models::InlineResponse20051 ipam_vrfs_list(name, rd, enforce_unique, tenant_group_id, tenant_group, tenant_id, tenant, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, tag, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**rd** | Option<**String**> |  |  |
**enforce_unique** | Option<**String**> |  |  |
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
**tag** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20051**](inline_response_200_51.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vrfs_partial_update

> crate::models::Vrf ipam_vrfs_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VRF. | [required] |
**data** | [**WritableVrf**](WritableVrf.md) |  | [required] |

### Return type

[**crate::models::Vrf**](VRF.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vrfs_read

> crate::models::Vrf ipam_vrfs_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VRF. | [required] |

### Return type

[**crate::models::Vrf**](VRF.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ipam_vrfs_update

> crate::models::Vrf ipam_vrfs_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this VRF. | [required] |
**data** | [**WritableVrf**](WritableVrf.md) |  | [required] |

### Return type

[**crate::models::Vrf**](VRF.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

