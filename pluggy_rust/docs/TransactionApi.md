# \TransactionApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transactions_list**](TransactionApi.md#transactions_list) | **GET** /transactions | List
[**transactions_retrieve**](TransactionApi.md#transactions_retrieve) | **GET** /transactions/{id} | Retrieve
[**transactions_update**](TransactionApi.md#transactions_update) | **PATCH** /transactions/{id} | Update



## transactions_list

> models::PageResponseTransactions transactions_list(account_id, ids, from, to, page_size, page, bill_id, created_at_from)
List

Recovers all transactions collected for the acount provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **uuid::Uuid** | Account primary identifier | [required] |
**ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Array of transaction identifiers. If defined, 'from' and 'to' parameters will be discarded |  |
**from** | Option<**String**> | Filter greater than date. Format (yyyy-mm-dd) |  |
**to** | Option<**String**> | Filter lower than date. Format (yyyy-mm-dd) |  |
**page_size** | Option<**f64**> | Page size for the paging request, default: 500 |  |
**page** | Option<**f64**> | Page number for the paging request, default: 1 |  |
**bill_id** | Option<**uuid::Uuid**> | Credit Card Bill's primary identifier, if account is a credit card. |  |
**created_at_from** | Option<**String**> | Filter greater than createdAt. Format (yyyy-mm-ddThh:mm:ss.000Z) |  |

### Return type

[**models::PageResponseTransactions**](PageResponseTransactions.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_retrieve

> models::Transaction transactions_retrieve(id)
Retrieve

Recovers the transaction resource by it's id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | transaction primary identifier | [required] |

### Return type

[**models::Transaction**](Transaction.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_update

> models::Transaction transactions_update(id, update_transaction)
Update

Update the transaction's category by it's id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | transaction primary identifier | [required] |
**update_transaction** | [**UpdateTransaction**](UpdateTransaction.md) | New category identifier | [required] |

### Return type

[**models::Transaction**](Transaction.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

