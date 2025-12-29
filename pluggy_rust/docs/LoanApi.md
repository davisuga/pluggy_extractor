# \LoanApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**loans_list**](LoanApi.md#loans_list) | **GET** /loans | List
[**loans_retrieve**](LoanApi.md#loans_retrieve) | **GET** /loans/{id} | Retrieve



## loans_list

> models::LoansList200Response loans_list(item_id)
List

Recovers all loans collected for the item provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item's primary identifier | [required] |

### Return type

[**models::LoansList200Response**](loans_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## loans_retrieve

> models::Loan loans_retrieve(id)
Retrieve

Recovers the loan resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | loan primary identifier | [required] |

### Return type

[**models::Loan**](Loan.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

