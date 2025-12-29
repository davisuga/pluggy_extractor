# \ConsentApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**consent_retrieve**](ConsentApi.md#consent_retrieve) | **GET** /consents/{id} | Retrieve
[**consents_list**](ConsentApi.md#consents_list) | **GET** /consents | List



## consent_retrieve

> models::Consent consent_retrieve(id)
Retrieve

Recovers the consent resource by it's id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Consent primary identifier | [required] |

### Return type

[**models::Consent**](Consent.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## consents_list

> models::PageResponseConsents consents_list(item_id)
List

Recovers all consents given to the item provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item primary identifier | [required] |

### Return type

[**models::PageResponseConsents**](PageResponseConsents.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

