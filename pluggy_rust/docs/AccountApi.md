# \AccountApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_statements_list**](AccountApi.md#account_statements_list) | **GET** /accounts/{id}/statements | List account statements
[**accounts_list**](AccountApi.md#accounts_list) | **GET** /accounts | List
[**accounts_retrieve**](AccountApi.md#accounts_retrieve) | **GET** /accounts/{id} | Retrieve



## account_statements_list

> models::AccountStatementsList200Response account_statements_list(id)
List account statements

Recovers all statements collected for the account provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Account primary identifier | [required] |

### Return type

[**models::AccountStatementsList200Response**](account_statements_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_list

> models::AccountsList200Response accounts_list(item_id, r#type)
List

Recovers all accounts collected for the item provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item primary identifier | [required] |
**r#type** | Option<**String**> | Parameter to filter between bank accounts and credit accounts |  |

### Return type

[**models::AccountsList200Response**](accounts_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_retrieve

> models::Account accounts_retrieve(id)
Retrieve

Recovers the account resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Account primary identifier | [required] |

### Return type

[**models::Account**](Account.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

