# \CircuitsApi

All URIs are relative to *http://localhost:32815/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**circuits_choices_list**](CircuitsApi.md#circuits_choices_list) | **get** /circuits/_choices/ | 
[**circuits_choices_read**](CircuitsApi.md#circuits_choices_read) | **get** /circuits/_choices/{id}/ | 
[**circuits_circuit_terminations_create**](CircuitsApi.md#circuits_circuit_terminations_create) | **post** /circuits/circuit-terminations/ | 
[**circuits_circuit_terminations_delete**](CircuitsApi.md#circuits_circuit_terminations_delete) | **delete** /circuits/circuit-terminations/{id}/ | 
[**circuits_circuit_terminations_list**](CircuitsApi.md#circuits_circuit_terminations_list) | **get** /circuits/circuit-terminations/ | 
[**circuits_circuit_terminations_partial_update**](CircuitsApi.md#circuits_circuit_terminations_partial_update) | **patch** /circuits/circuit-terminations/{id}/ | 
[**circuits_circuit_terminations_read**](CircuitsApi.md#circuits_circuit_terminations_read) | **get** /circuits/circuit-terminations/{id}/ | 
[**circuits_circuit_terminations_update**](CircuitsApi.md#circuits_circuit_terminations_update) | **put** /circuits/circuit-terminations/{id}/ | 
[**circuits_circuit_types_create**](CircuitsApi.md#circuits_circuit_types_create) | **post** /circuits/circuit-types/ | 
[**circuits_circuit_types_delete**](CircuitsApi.md#circuits_circuit_types_delete) | **delete** /circuits/circuit-types/{id}/ | 
[**circuits_circuit_types_list**](CircuitsApi.md#circuits_circuit_types_list) | **get** /circuits/circuit-types/ | 
[**circuits_circuit_types_partial_update**](CircuitsApi.md#circuits_circuit_types_partial_update) | **patch** /circuits/circuit-types/{id}/ | 
[**circuits_circuit_types_read**](CircuitsApi.md#circuits_circuit_types_read) | **get** /circuits/circuit-types/{id}/ | 
[**circuits_circuit_types_update**](CircuitsApi.md#circuits_circuit_types_update) | **put** /circuits/circuit-types/{id}/ | 
[**circuits_circuits_create**](CircuitsApi.md#circuits_circuits_create) | **post** /circuits/circuits/ | 
[**circuits_circuits_delete**](CircuitsApi.md#circuits_circuits_delete) | **delete** /circuits/circuits/{id}/ | 
[**circuits_circuits_list**](CircuitsApi.md#circuits_circuits_list) | **get** /circuits/circuits/ | 
[**circuits_circuits_partial_update**](CircuitsApi.md#circuits_circuits_partial_update) | **patch** /circuits/circuits/{id}/ | 
[**circuits_circuits_read**](CircuitsApi.md#circuits_circuits_read) | **get** /circuits/circuits/{id}/ | 
[**circuits_circuits_update**](CircuitsApi.md#circuits_circuits_update) | **put** /circuits/circuits/{id}/ | 
[**circuits_providers_create**](CircuitsApi.md#circuits_providers_create) | **post** /circuits/providers/ | 
[**circuits_providers_delete**](CircuitsApi.md#circuits_providers_delete) | **delete** /circuits/providers/{id}/ | 
[**circuits_providers_graphs**](CircuitsApi.md#circuits_providers_graphs) | **get** /circuits/providers/{id}/graphs/ | 
[**circuits_providers_list**](CircuitsApi.md#circuits_providers_list) | **get** /circuits/providers/ | 
[**circuits_providers_partial_update**](CircuitsApi.md#circuits_providers_partial_update) | **patch** /circuits/providers/{id}/ | 
[**circuits_providers_read**](CircuitsApi.md#circuits_providers_read) | **get** /circuits/providers/{id}/ | 
[**circuits_providers_update**](CircuitsApi.md#circuits_providers_update) | **put** /circuits/providers/{id}/ | 



## circuits_choices_list

> circuits_choices_list()


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


## circuits_choices_read

> circuits_choices_read(id)


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


## circuits_circuit_terminations_create

> crate::models::CircuitTermination circuits_circuit_terminations_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableCircuitTermination**](WritableCircuitTermination.md) |  | [required] |

### Return type

[**crate::models::CircuitTermination**](CircuitTermination.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuit_terminations_delete

> circuits_circuit_terminations_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit termination. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuit_terminations_list

> crate::models::InlineResponse200 circuits_circuit_terminations_list(term_side, port_speed, upstream_speed, xconnect_id, q, circuit_id, site_id, site, term_side__n, port_speed__n, port_speed__lte, port_speed__lt, port_speed__gte, port_speed__gt, upstream_speed__n, upstream_speed__lte, upstream_speed__lt, upstream_speed__gte, upstream_speed__gt, xconnect_id__n, xconnect_id__ic, xconnect_id__nic, xconnect_id__iew, xconnect_id__niew, xconnect_id__isw, xconnect_id__nisw, xconnect_id__ie, xconnect_id__nie, circuit_id__n, site_id__n, site__n, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term_side** | Option<**String**> |  |  |
**port_speed** | Option<**String**> |  |  |
**upstream_speed** | Option<**String**> |  |  |
**xconnect_id** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**circuit_id** | Option<**String**> |  |  |
**site_id** | Option<**String**> |  |  |
**site** | Option<**String**> |  |  |
**term_side__n** | Option<**String**> |  |  |
**port_speed__n** | Option<**String**> |  |  |
**port_speed__lte** | Option<**String**> |  |  |
**port_speed__lt** | Option<**String**> |  |  |
**port_speed__gte** | Option<**String**> |  |  |
**port_speed__gt** | Option<**String**> |  |  |
**upstream_speed__n** | Option<**String**> |  |  |
**upstream_speed__lte** | Option<**String**> |  |  |
**upstream_speed__lt** | Option<**String**> |  |  |
**upstream_speed__gte** | Option<**String**> |  |  |
**upstream_speed__gt** | Option<**String**> |  |  |
**xconnect_id__n** | Option<**String**> |  |  |
**xconnect_id__ic** | Option<**String**> |  |  |
**xconnect_id__nic** | Option<**String**> |  |  |
**xconnect_id__iew** | Option<**String**> |  |  |
**xconnect_id__niew** | Option<**String**> |  |  |
**xconnect_id__isw** | Option<**String**> |  |  |
**xconnect_id__nisw** | Option<**String**> |  |  |
**xconnect_id__ie** | Option<**String**> |  |  |
**xconnect_id__nie** | Option<**String**> |  |  |
**circuit_id__n** | Option<**String**> |  |  |
**site_id__n** | Option<**String**> |  |  |
**site__n** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuit_terminations_partial_update

> crate::models::CircuitTermination circuits_circuit_terminations_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit termination. | [required] |
**data** | [**WritableCircuitTermination**](WritableCircuitTermination.md) |  | [required] |

### Return type

[**crate::models::CircuitTermination**](CircuitTermination.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuit_terminations_read

> crate::models::CircuitTermination circuits_circuit_terminations_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit termination. | [required] |

### Return type

[**crate::models::CircuitTermination**](CircuitTermination.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuit_terminations_update

> crate::models::CircuitTermination circuits_circuit_terminations_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit termination. | [required] |
**data** | [**WritableCircuitTermination**](WritableCircuitTermination.md) |  | [required] |

### Return type

[**crate::models::CircuitTermination**](CircuitTermination.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuit_types_create

> crate::models::CircuitType circuits_circuit_types_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**CircuitType**](CircuitType.md) |  | [required] |

### Return type

[**crate::models::CircuitType**](CircuitType.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuit_types_delete

> circuits_circuit_types_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit type. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuit_types_list

> crate::models::InlineResponse2001 circuits_circuit_types_list(id, name, slug, q, id__n, id__lte, id__lt, id__gte, id__gt, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, slug__n, slug__ic, slug__nic, slug__iew, slug__niew, slug__isw, slug__nisw, slug__ie, slug__nie, limit, offset)


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

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuit_types_partial_update

> crate::models::CircuitType circuits_circuit_types_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit type. | [required] |
**data** | [**CircuitType**](CircuitType.md) |  | [required] |

### Return type

[**crate::models::CircuitType**](CircuitType.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuit_types_read

> crate::models::CircuitType circuits_circuit_types_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit type. | [required] |

### Return type

[**crate::models::CircuitType**](CircuitType.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuit_types_update

> crate::models::CircuitType circuits_circuit_types_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit type. | [required] |
**data** | [**CircuitType**](CircuitType.md) |  | [required] |

### Return type

[**crate::models::CircuitType**](CircuitType.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuits_create

> crate::models::Circuit circuits_circuits_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableCircuit**](WritableCircuit.md) |  | [required] |

### Return type

[**crate::models::Circuit**](Circuit.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuits_delete

> circuits_circuits_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuits_list

> crate::models::InlineResponse2002 circuits_circuits_list(cid, install_date, commit_rate, tenant_group_id, tenant_group, tenant_id, tenant, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, provider_id, provider, type_id, _type, status, site_id, site, region_id, region, tag, cid__n, cid__ic, cid__nic, cid__iew, cid__niew, cid__isw, cid__nisw, cid__ie, cid__nie, install_date__n, install_date__lte, install_date__lt, install_date__gte, install_date__gt, commit_rate__n, commit_rate__lte, commit_rate__lt, commit_rate__gte, commit_rate__gt, tenant_group_id__n, tenant_group__n, tenant_id__n, tenant__n, provider_id__n, provider__n, type_id__n, type__n, status__n, site_id__n, site__n, region_id__n, region__n, tag__n, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | Option<**String**> |  |  |
**install_date** | Option<**String**> |  |  |
**commit_rate** | Option<**String**> |  |  |
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
**provider_id** | Option<**String**> |  |  |
**provider** | Option<**String**> |  |  |
**type_id** | Option<**String**> |  |  |
**_type** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**site_id** | Option<**String**> |  |  |
**site** | Option<**String**> |  |  |
**region_id** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**cid__n** | Option<**String**> |  |  |
**cid__ic** | Option<**String**> |  |  |
**cid__nic** | Option<**String**> |  |  |
**cid__iew** | Option<**String**> |  |  |
**cid__niew** | Option<**String**> |  |  |
**cid__isw** | Option<**String**> |  |  |
**cid__nisw** | Option<**String**> |  |  |
**cid__ie** | Option<**String**> |  |  |
**cid__nie** | Option<**String**> |  |  |
**install_date__n** | Option<**String**> |  |  |
**install_date__lte** | Option<**String**> |  |  |
**install_date__lt** | Option<**String**> |  |  |
**install_date__gte** | Option<**String**> |  |  |
**install_date__gt** | Option<**String**> |  |  |
**commit_rate__n** | Option<**String**> |  |  |
**commit_rate__lte** | Option<**String**> |  |  |
**commit_rate__lt** | Option<**String**> |  |  |
**commit_rate__gte** | Option<**String**> |  |  |
**commit_rate__gt** | Option<**String**> |  |  |
**tenant_group_id__n** | Option<**String**> |  |  |
**tenant_group__n** | Option<**String**> |  |  |
**tenant_id__n** | Option<**String**> |  |  |
**tenant__n** | Option<**String**> |  |  |
**provider_id__n** | Option<**String**> |  |  |
**provider__n** | Option<**String**> |  |  |
**type_id__n** | Option<**String**> |  |  |
**type__n** | Option<**String**> |  |  |
**status__n** | Option<**String**> |  |  |
**site_id__n** | Option<**String**> |  |  |
**site__n** | Option<**String**> |  |  |
**region_id__n** | Option<**String**> |  |  |
**region__n** | Option<**String**> |  |  |
**tag__n** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuits_partial_update

> crate::models::Circuit circuits_circuits_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit. | [required] |
**data** | [**WritableCircuit**](WritableCircuit.md) |  | [required] |

### Return type

[**crate::models::Circuit**](Circuit.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuits_read

> crate::models::Circuit circuits_circuits_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit. | [required] |

### Return type

[**crate::models::Circuit**](Circuit.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_circuits_update

> crate::models::Circuit circuits_circuits_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this circuit. | [required] |
**data** | [**WritableCircuit**](WritableCircuit.md) |  | [required] |

### Return type

[**crate::models::Circuit**](Circuit.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_providers_create

> crate::models::Provider circuits_providers_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**Provider**](Provider.md) |  | [required] |

### Return type

[**crate::models::Provider**](Provider.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_providers_delete

> circuits_providers_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this provider. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_providers_graphs

> crate::models::Provider circuits_providers_graphs(id)


A convenience method for rendering graphs for a particular provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this provider. | [required] |

### Return type

[**crate::models::Provider**](Provider.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_providers_list

> crate::models::InlineResponse2003 circuits_providers_list(name, slug, asn, account, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, region_id, region, site_id, site, tag, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, slug__n, slug__ic, slug__nic, slug__iew, slug__niew, slug__isw, slug__nisw, slug__ie, slug__nie, asn__n, asn__lte, asn__lt, asn__gte, asn__gt, account__n, account__ic, account__nic, account__iew, account__niew, account__isw, account__nisw, account__ie, account__nie, region_id__n, region__n, site_id__n, site__n, tag__n, limit, offset)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |
**asn** | Option<**String**> |  |  |
**account** | Option<**String**> |  |  |
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
**asn__n** | Option<**String**> |  |  |
**asn__lte** | Option<**String**> |  |  |
**asn__lt** | Option<**String**> |  |  |
**asn__gte** | Option<**String**> |  |  |
**asn__gt** | Option<**String**> |  |  |
**account__n** | Option<**String**> |  |  |
**account__ic** | Option<**String**> |  |  |
**account__nic** | Option<**String**> |  |  |
**account__iew** | Option<**String**> |  |  |
**account__niew** | Option<**String**> |  |  |
**account__isw** | Option<**String**> |  |  |
**account__nisw** | Option<**String**> |  |  |
**account__ie** | Option<**String**> |  |  |
**account__nie** | Option<**String**> |  |  |
**region_id__n** | Option<**String**> |  |  |
**region__n** | Option<**String**> |  |  |
**site_id__n** | Option<**String**> |  |  |
**site__n** | Option<**String**> |  |  |
**tag__n** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_providers_partial_update

> crate::models::Provider circuits_providers_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this provider. | [required] |
**data** | [**Provider**](Provider.md) |  | [required] |

### Return type

[**crate::models::Provider**](Provider.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_providers_read

> crate::models::Provider circuits_providers_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this provider. | [required] |

### Return type

[**crate::models::Provider**](Provider.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## circuits_providers_update

> crate::models::Provider circuits_providers_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this provider. | [required] |
**data** | [**Provider**](Provider.md) |  | [required] |

### Return type

[**crate::models::Provider**](Provider.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

