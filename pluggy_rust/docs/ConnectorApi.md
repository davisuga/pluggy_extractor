# \ConnectorApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**connector_retrieve**](ConnectorApi.md#connector_retrieve) | **GET** /connectors/{id} | Retrieve
[**connectors_list**](ConnectorApi.md#connectors_list) | **GET** /connectors | List
[**connectors_validate**](ConnectorApi.md#connectors_validate) | **POST** /connectors/{id}/validate | Validate



## connector_retrieve

> models::Connector connector_retrieve(id, health_details)
Retrieve

This endpoint retrieves a specific connector.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | Connector primary identifier | [required] |
**health_details** | Option<**bool**> | Include health details about latest connections and percentage of errors (connection rate) |  |

### Return type

[**models::Connector**](Connector.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connectors_list

> models::ConnectorListResponse connectors_list(countries, types, name, sandbox, health_details, is_open_finance, supports_payment_initiation, supports_smart_transfers)
List

This endpoint retrieves all available connectors.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**countries** | Option<[**Vec<String>**](String.md)> | A list of countries of connectors to filter. |  |
**types** | Option<[**Vec<String>**](String.md)> | A list of types of connectors to filter. |  |
**name** | Option<**String**> | Name alike look up of the connector |  |
**sandbox** | Option<**bool**> | Include sandbox connectors if set to true (default: false). |  |
**health_details** | Option<**bool**> | Include health details about latest connections and percentage of errors (connection rate) |  |
**is_open_finance** | Option<**bool**> | Filter connectors by the `isOpenFinance` attribute. If not sent, it won't filter. |  |
**supports_payment_initiation** | Option<**bool**> | Filter connectors by the `supportsPaymentInitiation` attribute. If not sent, it won't filter. |  |
**supports_smart_transfers** | Option<**bool**> | Filter connectors by the `supportsSmartTransfers` attribute. If not sent, it won't filter. |  |

### Return type

[**models::ConnectorListResponse**](ConnectorListResponse.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connectors_validate

> models::ParameterValidationResponse connectors_validate(id, request_body)
Validate

Validates a connector parameters usign the connector validation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | Connector's primary identifier | [required] |
**request_body** | [**std::collections::HashMap<String, String>**](String.md) | Connector's input credentials in a key-value object. | [required] |

### Return type

[**models::ParameterValidationResponse**](ParameterValidationResponse.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

