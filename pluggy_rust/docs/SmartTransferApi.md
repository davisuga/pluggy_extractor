# \SmartTransferApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**smart_tranfers_preauthorizations_list**](SmartTransferApi.md#smart_tranfers_preauthorizations_list) | **GET** /smart-transfers/preauthorizations | List preauthorizations
[**smart_transfer_payment_create**](SmartTransferApi.md#smart_transfer_payment_create) | **POST** /smart-transfers/payments | Create payment
[**smart_transfer_paymentretrieve**](SmartTransferApi.md#smart_transfer_paymentretrieve) | **GET** /smart-transfers/payments/{id} | Retrieve payment
[**smart_transfer_preauthorization_create**](SmartTransferApi.md#smart_transfer_preauthorization_create) | **POST** /smart-transfers/preauthorizations | Create preauthorization
[**smart_transfer_preauthorization_retrieve**](SmartTransferApi.md#smart_transfer_preauthorization_retrieve) | **GET** /smart-transfers/preauthorizations/{id} | Retrieve preauthorization



## smart_tranfers_preauthorizations_list

> models::SmartTranfersPreauthorizationsList200Response smart_tranfers_preauthorizations_list(page_size, page)
List preauthorizations

Recovers all created preauthorizations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**f64**> | Page size for the paging request, default: 500 |  |
**page** | Option<**f64**> | Page number for the paging request, default: 1 |  |

### Return type

[**models::SmartTranfersPreauthorizationsList200Response**](smart_tranfers_preauthorizations_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## smart_transfer_payment_create

> models::SmartTransferPayment smart_transfer_payment_create(create_smart_transfer_payment)
Create payment

Creates the smart transfer payment resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_smart_transfer_payment** | [**CreateSmartTransferPayment**](CreateSmartTransferPayment.md) |  | [required] |

### Return type

[**models::SmartTransferPayment**](SmartTransferPayment.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## smart_transfer_paymentretrieve

> models::SmartTransferPayment smart_transfer_paymentretrieve(id)
Retrieve payment

Recovers the smart transfer payment resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment primary identifier | [required] |

### Return type

[**models::SmartTransferPayment**](SmartTransferPayment.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## smart_transfer_preauthorization_create

> models::SmartTransferPreauthorization smart_transfer_preauthorization_create(create_smart_transfer_preauthorization)
Create preauthorization

Creates the smart transfer preauthorization resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_smart_transfer_preauthorization** | [**CreateSmartTransferPreauthorization**](CreateSmartTransferPreauthorization.md) |  | [required] |

### Return type

[**models::SmartTransferPreauthorization**](SmartTransferPreauthorization.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## smart_transfer_preauthorization_retrieve

> models::SmartTransferPreauthorization smart_transfer_preauthorization_retrieve(id)
Retrieve preauthorization

Recovers the smart transfer preauthorization resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Preauthorization primary identifier | [required] |

### Return type

[**models::SmartTransferPreauthorization**](SmartTransferPreauthorization.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

