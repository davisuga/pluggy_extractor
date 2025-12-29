# \PaymentRequestApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_request_create**](PaymentRequestApi.md#payment_request_create) | **POST** /payments/requests | Create
[**payment_request_create_pix_qr**](PaymentRequestApi.md#payment_request_create_pix_qr) | **POST** /payments/requests/pix-qr | Create PIX QR payment request
[**payment_request_delete**](PaymentRequestApi.md#payment_request_delete) | **DELETE** /payments/requests/{id} | Delete
[**payment_request_retrieve**](PaymentRequestApi.md#payment_request_retrieve) | **GET** /payments/requests/{id} | Retrieve
[**payment_request_update**](PaymentRequestApi.md#payment_request_update) | **PATCH** /payments/requests/{id} | Update
[**payment_requests_list**](PaymentRequestApi.md#payment_requests_list) | **GET** /payments/requests | List



## payment_request_create

> models::PaymentRequest payment_request_create(create_payment_request)
Create

Creates the payment request resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_request** | [**CreatePaymentRequest**](CreatePaymentRequest.md) |  | [required] |

### Return type

[**models::PaymentRequest**](PaymentRequest.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_request_create_pix_qr

> models::PaymentRequest payment_request_create_pix_qr(create_pix_qr_payment_request)
Create PIX QR payment request

Creates the PIX QR payment request resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_pix_qr_payment_request** | [**CreatePixQrPaymentRequest**](CreatePixQrPaymentRequest.md) |  | [required] |

### Return type

[**models::PaymentRequest**](PaymentRequest.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_request_delete

> payment_request_delete(id)
Delete

Deletes the payment request resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment request primary identifier | [required] |

### Return type

 (empty response body)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_request_retrieve

> models::PaymentRequest payment_request_retrieve(id)
Retrieve

Recovers the payment request resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment request primary identifier | [required] |

### Return type

[**models::PaymentRequest**](PaymentRequest.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_request_update

> models::CreatePaymentRequest payment_request_update(id, update_payment_request)
Update

Updates the payment request resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment request primary identifier | [required] |
**update_payment_request** | [**UpdatePaymentRequest**](UpdatePaymentRequest.md) |  | [required] |

### Return type

[**models::CreatePaymentRequest**](CreatePaymentRequest.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_requests_list

> models::PaymentRequestsList200Response payment_requests_list(page_size, page, from, to, customer, pix_key)
List

Recovers all created payment requests

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**f64**> | Page size for the paging request, default: 500 |  |
**page** | Option<**f64**> | Page number for the paging request, default: 1 |  |
**from** | Option<**String**> | Filter payment requests by start date. Returns only requests created **on or after** this date. |  |
**to** | Option<**String**> | Filter payment requests by end date. Returns only requests created **on or before** this date. |  |
**customer** | Option<**String**> | Filter payment requests with one customer attribute (name, email, CPF or CNPJ) |  |
**pix_key** | Option<**String**> | Filter payment requests by Pix Key |  |

### Return type

[**models::PaymentRequestsList200Response**](payment_requests_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

