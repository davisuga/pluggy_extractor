# \MerchantApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**merchants_get_by_cnpj**](MerchantApi.md#merchants_get_by_cnpj) | **GET** /merchants | Get merchants by CNPJ list



## merchants_get_by_cnpj

> models::MerchantsGetByCnpj200Response merchants_get_by_cnpj(cnpjs)
Get merchants by CNPJ list

Retrieves merchant information for a list of CNPJs. Returns an object containing found merchants, valid CNPJs that were not found, and invalid CNPJs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cnpjs** | Option<**String**> | Comma-separated list of CNPJs |  |

### Return type

[**models::MerchantsGetByCnpj200Response**](merchants_get_by_cnpj_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

