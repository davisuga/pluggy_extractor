# \WebhookApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**webhooks_create**](WebhookApi.md#webhooks_create) | **POST** /webhooks | Create
[**webhooks_delete**](WebhookApi.md#webhooks_delete) | **DELETE** /webhooks/{id} | Delete
[**webhooks_list**](WebhookApi.md#webhooks_list) | **GET** /webhooks | List
[**webhooks_retrieve**](WebhookApi.md#webhooks_retrieve) | **GET** /webhooks/{id} | Retrieve
[**webhooks_update**](WebhookApi.md#webhooks_update) | **PATCH** /webhooks/{id} | Update



## webhooks_create

> models::Webhook webhooks_create(create_webhook)
Create

Creates a webhook attached to the specific event and provides the notification url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_webhook** | [**CreateWebhook**](CreateWebhook.md) | Expects the following webhooks parameters: event: One of the event types that are supported. url: An https url that will receive the POST of the event. headers: optional key-value pairs to send with the POST of the event. | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_delete

> models::ICountResponse webhooks_delete(id)
Delete

Deletes a webhook listener by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | webhook primary identifier | [required] |

### Return type

[**models::ICountResponse**](ICountResponse.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_list

> models::WebhooksList200Response webhooks_list()
List

Retrieves all Webhooks associated with your application

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebhooksList200Response**](webhooks_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_retrieve

> models::Webhook webhooks_retrieve(id)
Retrieve

Retrieves a specific webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | webhook primary identifier | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_update

> models::Webhook webhooks_update(id, create_webhook)
Update

Updates a webhook event and/or url listener. Once updated all events that are triggered will replicate the updated logic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | webhook primary identifier | [required] |
**create_webhook** | [**CreateWebhook**](CreateWebhook.md) | Expects the following webhooks parameters: event: One of the event types that are supported. url: An https url that will receive the POST of the event. headers: optional key-value pairs to send with the POST of the event. | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

