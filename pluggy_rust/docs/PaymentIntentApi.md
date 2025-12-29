# \PaymentIntentApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_intent_create**](PaymentIntentApi.md#payment_intent_create) | **POST** /payments/intents | Create
[**payment_intent_retrieve**](PaymentIntentApi.md#payment_intent_retrieve) | **GET** /payments/intents/{id} | Retrieve
[**payment_intents_list**](PaymentIntentApi.md#payment_intents_list) | **GET** /payments/intents | List



## payment_intent_create

> models::PaymentIntent payment_intent_create(create_payment_intent)
Create

Creates the payment intent resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_intent** | [**CreatePaymentIntent**](CreatePaymentIntent.md) |  | [required] |

### Return type

[**models::PaymentIntent**](PaymentIntent.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_intent_retrieve

> models::PaymentIntent payment_intent_retrieve(id)
Retrieve

Recovers the payment intent resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment intent primary identifier | [required] |

### Return type

[**models::PaymentIntent**](PaymentIntent.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_intents_list

> models::PaymentIntentsList200Response payment_intents_list(payment_request_id)
List

Recovers all created payment intents for the payment request provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_request_id** | **uuid::Uuid** | Payment request primary identifier | [required] |

### Return type

[**models::PaymentIntentsList200Response**](payment_intents_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

