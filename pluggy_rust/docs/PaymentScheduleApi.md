# \PaymentScheduleApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_schedules_cancel**](PaymentScheduleApi.md#payment_schedules_cancel) | **POST** /payments/requests/{id}/schedules/cancel | Cancel Payment Schedule Authorization
[**payment_schedules_cancel_specific**](PaymentScheduleApi.md#payment_schedules_cancel_specific) | **POST** /payments/requests/{id}/schedules/{scheduleId}/cancel | Cancel Payment Schedule
[**payment_schedules_list**](PaymentScheduleApi.md#payment_schedules_list) | **GET** /payments/requests/{id}/schedules | List Schedules



## payment_schedules_cancel

> payment_schedules_cancel(id)
Cancel Payment Schedule Authorization



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


## payment_schedules_cancel_specific

> payment_schedules_cancel_specific(id, schedule_id)
Cancel Payment Schedule



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment request primary identifier | [required] |
**schedule_id** | **uuid::Uuid** | Payment schedule primary identifier | [required] |

### Return type

 (empty response body)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_schedules_list

> models::PaymentSchedulesList200Response payment_schedules_list(id)
List Schedules

Recovers all scheduled payments from a payment request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment request primary identifier | [required] |

### Return type

[**models::PaymentSchedulesList200Response**](payment_schedules_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

