# \ItemsApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**items_create**](ItemsApi.md#items_create) | **POST** /items | Create
[**items_delete**](ItemsApi.md#items_delete) | **DELETE** /items/{id} | Delete
[**items_disable_autosync**](ItemsApi.md#items_disable_autosync) | **PATCH** /items/{id}/disable-auto-sync | Disable item auto sync
[**items_retrieve**](ItemsApi.md#items_retrieve) | **GET** /items/{id} | Retrieve
[**items_send_mfa**](ItemsApi.md#items_send_mfa) | **POST** /items/{id}/mfa | Send MFA
[**items_update**](ItemsApi.md#items_update) | **PATCH** /items/{id} | Update



## items_create

> models::Item items_create(create_item)
Create

Creates a item and syncs all the products with the financial institution, using as credentials the sent parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_item** | [**CreateItem**](CreateItem.md) |  | [required] |

### Return type

[**models::Item**](Item.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## items_delete

> models::ICountResponse items_delete(id)
Delete

Delete the item by its primary identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Item primary identifier | [required] |

### Return type

[**models::ICountResponse**](ICountResponse.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## items_disable_autosync

> models::Item items_disable_autosync(id)
Disable item auto sync

When client disables auto sync, the item will not be updated automatically anymore, until the client force an item update.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Item primary identifier | [required] |

### Return type

[**models::Item**](Item.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## items_retrieve

> models::Item items_retrieve(id)
Retrieve

Recovers the item resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Item primary identifier | [required] |

### Return type

[**models::Item**](Item.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## items_send_mfa

> models::Item items_send_mfa(id, request_body)
Send MFA

When item is Waiting User Input, this method allows to submit multi-factor authentication value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Item primary identifier | [required] |
**request_body** | [**std::collections::HashMap<String, String>**](String.md) |  | [required] |

### Return type

[**models::Item**](Item.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## items_update

> models::Item items_update(id, update_item)
Update

Triggers new syncronization for the Item, optionally updating the stored credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Item primary identifier | [required] |
**update_item** | Option<[**UpdateItem**](UpdateItem.md)> | Update item request |  |

### Return type

[**models::Item**](Item.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

