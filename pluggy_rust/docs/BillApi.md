# \BillApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bills_list**](BillApi.md#bills_list) | **GET** /bills | List
[**bills_retrieve**](BillApi.md#bills_retrieve) | **GET** /bills/{id} | Retrieve



## bills_list

> models::BillsList200Response bills_list(account_id)
List

Recovers all credit card bills collected for the account provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** | Account's primary identifier | [required] |

### Return type

[**models::BillsList200Response**](bills_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bills_retrieve

> models::Bill bills_retrieve(id)
Retrieve

Recovers the bill resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Bill primary identifier | [required] |

### Return type

[**models::Bill**](Bill.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

