# \AutomaticPixApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_automatic_pix_schedule**](AutomaticPixApi.md#cancel_automatic_pix_schedule) | **POST** /payments/requests/{id}/automatic-pix/schedules/{scheduleId}/cancel | Cancel an Automatic PIX schedule
[**payment_request_cancel_automatic_pix_consent**](AutomaticPixApi.md#payment_request_cancel_automatic_pix_consent) | **POST** /payments/requests/{id}/automatic-pix/cancel | Cancel an automatic PIX consent
[**payment_request_create_automatic_pix**](AutomaticPixApi.md#payment_request_create_automatic_pix) | **POST** /payments/requests/automatic-pix | Create Automatic PIX payment request
[**payment_request_create_automatic_pix_schedule**](AutomaticPixApi.md#payment_request_create_automatic_pix_schedule) | **POST** /payments/requests/{id}/automatic-pix/schedule | Schedule Automatic PIX payment
[**payment_request_get_automatic_pix_schedule**](AutomaticPixApi.md#payment_request_get_automatic_pix_schedule) | **GET** /payments/requests/{requestId}/automatic-pix/schedules/{paymentId} | Get an automatic PIX scheduled payment
[**payment_request_get_automatic_pix_schedules**](AutomaticPixApi.md#payment_request_get_automatic_pix_schedules) | **GET** /payments/requests/{id}/automatic-pix/schedules | List Automatic PIX scheduled payments
[**retry_automatic_pix_schedule**](AutomaticPixApi.md#retry_automatic_pix_schedule) | **POST** /payments/requests/{id}/automatic-pix/schedules/{scheduleId}/retry | Retry an Automatic PIX schedule



## cancel_automatic_pix_schedule

> cancel_automatic_pix_schedule(id, schedule_id)
Cancel an Automatic PIX schedule

Cancels an Automatic PIX schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment request primary identifier | [required] |
**schedule_id** | **uuid::Uuid** | Automatic PIX schedule primary identifier | [required] |

### Return type

 (empty response body)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_request_cancel_automatic_pix_consent

> payment_request_cancel_automatic_pix_consent(id)
Cancel an automatic PIX consent

Cancels an automatic PIX consent

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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_request_create_automatic_pix

> models::PaymentRequest payment_request_create_automatic_pix(create_automatic_pix_payment_request)
Create Automatic PIX payment request

Creates a payment request where the payment is made using automatic PIX. Once consent is granted by the user, payments can be scheduled according to the rules defined in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_automatic_pix_payment_request** | [**CreateAutomaticPixPaymentRequest**](CreateAutomaticPixPaymentRequest.md) |  | [required] |

### Return type

[**models::PaymentRequest**](PaymentRequest.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_request_create_automatic_pix_schedule

> models::AutomaticPixPayment payment_request_create_automatic_pix_schedule(id, schedule_automatic_pix_payment_request)
Schedule Automatic PIX payment

Schedules an Automatic PIX payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment request primary identifier | [required] |
**schedule_automatic_pix_payment_request** | [**ScheduleAutomaticPixPaymentRequest**](ScheduleAutomaticPixPaymentRequest.md) |  | [required] |

### Return type

[**models::AutomaticPixPayment**](AutomaticPixPayment.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_request_get_automatic_pix_schedule

> models::AutomaticPixPayment payment_request_get_automatic_pix_schedule(request_id, payment_id)
Get an automatic PIX scheduled payment

Recovers an automatic PIX scheduled payment by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **uuid::Uuid** | Payment request primary identifier | [required] |
**payment_id** | **uuid::Uuid** | Automatic PIX scheduled payment primary identifier | [required] |

### Return type

[**models::AutomaticPixPayment**](AutomaticPixPayment.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_request_get_automatic_pix_schedules

> models::PaymentRequestGetAutomaticPixSchedules200Response payment_request_get_automatic_pix_schedules(id)
List Automatic PIX scheduled payments

Lists all Automatic PIX payments from a payment request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment request primary identifier | [required] |

### Return type

[**models::PaymentRequestGetAutomaticPixSchedules200Response**](payment_request_get_automatic_pix_schedules_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retry_automatic_pix_schedule

> retry_automatic_pix_schedule(id, schedule_id, retry_automatic_pix_payment_request)
Retry an Automatic PIX schedule

Retries an Automatic PIX schedule, only if the authorization accepts retries. The system allows up to 3 retry attempts. Requests must be submitted by 10pm on the day before the scheduled payment date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment request primary identifier | [required] |
**schedule_id** | **uuid::Uuid** | Automatic PIX schedule primary identifier | [required] |
**retry_automatic_pix_payment_request** | [**RetryAutomaticPixPaymentRequest**](RetryAutomaticPixPaymentRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

