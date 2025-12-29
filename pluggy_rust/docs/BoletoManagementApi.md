# \BoletoManagementApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**boleto_cancel**](BoletoManagementApi.md#boleto_cancel) | **POST** /boletos/{id}/cancel | Cancel Boleto
[**boleto_connection_create**](BoletoManagementApi.md#boleto_connection_create) | **POST** /boleto-connections | Connect boleto credentials
[**boleto_connection_create_from_item**](BoletoManagementApi.md#boleto_connection_create_from_item) | **POST** /boleto-connections/from-item | Create boleto connection from Item
[**boleto_create**](BoletoManagementApi.md#boleto_create) | **POST** /boletos | Issue Boleto
[**boleto_get**](BoletoManagementApi.md#boleto_get) | **GET** /boletos/{id} | Get Boleto



## boleto_cancel

> models::IssuedBoleto boleto_cancel(id)
Cancel Boleto

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Boleto primary identifier | [required] |

### Return type

[**models::IssuedBoleto**](IssuedBoleto.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## boleto_connection_create

> models::BoletoConnection boleto_connection_create(create_boleto_connection)
Connect boleto credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_boleto_connection** | [**CreateBoletoConnection**](CreateBoletoConnection.md) |  | [required] |

### Return type

[**models::BoletoConnection**](BoletoConnection.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## boleto_connection_create_from_item

> models::BoletoConnection boleto_connection_create_from_item(create_boleto_connection_from_item)
Create boleto connection from Item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_boleto_connection_from_item** | [**CreateBoletoConnectionFromItem**](CreateBoletoConnectionFromItem.md) |  | [required] |

### Return type

[**models::BoletoConnection**](BoletoConnection.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## boleto_create

> models::IssuedBoleto boleto_create(create_boleto)
Issue Boleto

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_boleto** | [**CreateBoleto**](CreateBoleto.md) |  | [required] |

### Return type

[**models::IssuedBoleto**](IssuedBoleto.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## boleto_get

> models::IssuedBoleto boleto_get(id)
Get Boleto

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Boleto primary identifier | [required] |

### Return type

[**models::IssuedBoleto**](IssuedBoleto.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

