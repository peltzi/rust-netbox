# \SecretsApi

All URIs are relative to *http://localhost:8000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**secrets_choices_list**](SecretsApi.md#secrets_choices_list) | **get** /secrets/_choices/ | 
[**secrets_choices_read**](SecretsApi.md#secrets_choices_read) | **get** /secrets/_choices/{id}/ | 
[**secrets_generate_rsa_key_pair_list**](SecretsApi.md#secrets_generate_rsa_key_pair_list) | **get** /secrets/generate-rsa-key-pair/ | This endpoint can be used to generate a new RSA key pair. The keys are returned in PEM format.
[**secrets_get_session_key_create**](SecretsApi.md#secrets_get_session_key_create) | **post** /secrets/get-session-key/ | 
[**secrets_secret_roles_create**](SecretsApi.md#secrets_secret_roles_create) | **post** /secrets/secret-roles/ | 
[**secrets_secret_roles_delete**](SecretsApi.md#secrets_secret_roles_delete) | **delete** /secrets/secret-roles/{id}/ | 
[**secrets_secret_roles_list**](SecretsApi.md#secrets_secret_roles_list) | **get** /secrets/secret-roles/ | 
[**secrets_secret_roles_partial_update**](SecretsApi.md#secrets_secret_roles_partial_update) | **patch** /secrets/secret-roles/{id}/ | 
[**secrets_secret_roles_read**](SecretsApi.md#secrets_secret_roles_read) | **get** /secrets/secret-roles/{id}/ | 
[**secrets_secret_roles_update**](SecretsApi.md#secrets_secret_roles_update) | **put** /secrets/secret-roles/{id}/ | 
[**secrets_secrets_create**](SecretsApi.md#secrets_secrets_create) | **post** /secrets/secrets/ | 
[**secrets_secrets_delete**](SecretsApi.md#secrets_secrets_delete) | **delete** /secrets/secrets/{id}/ | 
[**secrets_secrets_list**](SecretsApi.md#secrets_secrets_list) | **get** /secrets/secrets/ | 
[**secrets_secrets_partial_update**](SecretsApi.md#secrets_secrets_partial_update) | **patch** /secrets/secrets/{id}/ | 
[**secrets_secrets_read**](SecretsApi.md#secrets_secrets_read) | **get** /secrets/secrets/{id}/ | 
[**secrets_secrets_update**](SecretsApi.md#secrets_secrets_update) | **put** /secrets/secrets/{id}/ | 



## secrets_choices_list

> secrets_choices_list()


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


## secrets_choices_read

> secrets_choices_read(id)


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


## secrets_generate_rsa_key_pair_list

> secrets_generate_rsa_key_pair_list()
This endpoint can be used to generate a new RSA key pair. The keys are returned in PEM format.

{         \"public_key\": \"<public key>\",         \"private_key\": \"<private key>\"     }

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


## secrets_get_session_key_create

> secrets_get_session_key_create()


Retrieve a temporary session key to use for encrypting and decrypting secrets via the API. The user's private RSA key is POSTed with the name `private_key`. An example:      curl -v -X POST -H \"Authorization: Token <token>\" -H \"Accept: application/json; indent=4\" \\     --data-urlencode \"private_key@<filename>\" https://netbox/api/secrets/get-session-key/  This request will yield a base64-encoded session key to be included in an `X-Session-Key` header in future requests:      {         \"session_key\": \"+8t4SI6XikgVmB5+/urhozx9O5qCQANyOk1MNe6taRf=\"     }  This endpoint accepts one optional parameter: `preserve_key`. If True and a session key exists, the existing session key will be returned instead of a new one.

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


## secrets_secret_roles_create

> crate::models::SecretRole secrets_secret_roles_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**SecretRole**](SecretRole.md) |  | [required] |

### Return type

[**crate::models::SecretRole**](SecretRole.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_secret_roles_delete

> secrets_secret_roles_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this secret role. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_secret_roles_list

> crate::models::InlineResponse20052 secrets_secret_roles_list(id, name, slug, q, limit, offset)


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

[**crate::models::InlineResponse20052**](inline_response_200_52.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_secret_roles_partial_update

> crate::models::SecretRole secrets_secret_roles_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this secret role. | [required] |
**data** | [**SecretRole**](SecretRole.md) |  | [required] |

### Return type

[**crate::models::SecretRole**](SecretRole.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_secret_roles_read

> crate::models::SecretRole secrets_secret_roles_read(id)


Call to super to allow for caching

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this secret role. | [required] |

### Return type

[**crate::models::SecretRole**](SecretRole.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_secret_roles_update

> crate::models::SecretRole secrets_secret_roles_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this secret role. | [required] |
**data** | [**SecretRole**](SecretRole.md) |  | [required] |

### Return type

[**crate::models::SecretRole**](SecretRole.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_secrets_create

> crate::models::Secret secrets_secrets_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableSecret**](WritableSecret.md) |  | [required] |

### Return type

[**crate::models::Secret**](Secret.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_secrets_delete

> secrets_secrets_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this secret. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_secrets_list

> crate::models::InlineResponse20053 secrets_secrets_list(name, created, created__gte, created__lte, last_updated, last_updated__gte, last_updated__lte, id__in, q, role_id, role, device_id, device, tag, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**created__gte** | Option<**String**> |  |  |
**created__lte** | Option<**String**> |  |  |
**last_updated** | Option<**String**> |  |  |
**last_updated__gte** | Option<**String**> |  |  |
**last_updated__lte** | Option<**String**> |  |  |
**id__in** | Option<**String**> | Multiple values may be separated by commas. |  |
**q** | Option<**String**> |  |  |
**role_id** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**device_id** | Option<**String**> |  |  |
**device** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::InlineResponse20053**](inline_response_200_53.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_secrets_partial_update

> crate::models::Secret secrets_secrets_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this secret. | [required] |
**data** | [**WritableSecret**](WritableSecret.md) |  | [required] |

### Return type

[**crate::models::Secret**](Secret.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_secrets_read

> crate::models::Secret secrets_secrets_read(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this secret. | [required] |

### Return type

[**crate::models::Secret**](Secret.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_secrets_update

> crate::models::Secret secrets_secrets_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this secret. | [required] |
**data** | [**WritableSecret**](WritableSecret.md) |  | [required] |

### Return type

[**crate::models::Secret**](Secret.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

