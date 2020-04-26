# \CircuitsApi

All URIs are relative to *http://localhost:8000/api*

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

> crate::models::InlineResponse200 circuits_circuit_terminations_list(term_side, port_speed, upstream_speed, xconnect_id, q, circuit_id, site_id, site, limit, offset)


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

> crate::models::InlineResponse2001 circuits_circuit_types_list(id, name, slug, q, limit, offset)


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

> crate::models::InlineResponse2002 circuits_circuits_list(cid, install_date, commit_rate, tenant_group_id, tenant_group, tenant_id, tenant, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, provider_id, provider, type_id, _type, status, site_id, site, region_id, region, tag, limit, offset)


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

> crate::models::InlineResponse2003 circuits_providers_list(name, slug, asn, account, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, region_id, region, site_id, site, tag, limit, offset)


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

