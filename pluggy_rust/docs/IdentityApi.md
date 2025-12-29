# \IdentityApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**identity_find_by_item**](IdentityApi.md#identity_find_by_item) | **GET** /identity | Find by item
[**identity_retrieve**](IdentityApi.md#identity_retrieve) | **GET** /identity/{id} | Retrieve



## identity_find_by_item

> models::IdentityResponse identity_find_by_item(item_id)
Find by item

Recovers identity of an item if available

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item's primary identifier | [required] |

### Return type

[**models::IdentityResponse**](IdentityResponse.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_retrieve

> models::IdentityResponse identity_retrieve(id)
Retrieve

Recovers the identity resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | identity primary identifier | [required] |

### Return type

[**models::IdentityResponse**](IdentityResponse.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

