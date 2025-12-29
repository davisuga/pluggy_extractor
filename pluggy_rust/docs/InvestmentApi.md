# \InvestmentApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**investment_transactions_list**](InvestmentApi.md#investment_transactions_list) | **GET** /investments/{id}/transactions | List investment transactions
[**investments_list**](InvestmentApi.md#investments_list) | **GET** /investments | List
[**investments_retrieve**](InvestmentApi.md#investments_retrieve) | **GET** /investments/{id} | Retrieve



## investment_transactions_list

> models::PageResponseInvestmentTransactions investment_transactions_list(id, page_size, page)
List investment transactions

Recovers all investment transactions for the investment provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Investment primary identifier | [required] |
**page_size** | Option<**f64**> | Page size for the paging request, default: 500 |  |
**page** | Option<**f64**> | Page number for the paging request, default: 1 |  |

### Return type

[**models::PageResponseInvestmentTransactions**](PageResponseInvestmentTransactions.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## investments_list

> models::InvestmentsList200Response investments_list(item_id, r#type, page_size, page)
List

Recovers all investments collected for the item provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item's primary identifier | [required] |
**r#type** | Option<**String**> | Investment's type to filter |  |
**page_size** | Option<**f64**> | Page size for the paging request, default: 500 |  |
**page** | Option<**f64**> | Page number for the paging request, default: 1 |  |

### Return type

[**models::InvestmentsList200Response**](investments_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## investments_retrieve

> models::Investment investments_retrieve(id)
Retrieve

Recovers the investment resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | investment primary identifier | [required] |

### Return type

[**models::Investment**](Investment.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

