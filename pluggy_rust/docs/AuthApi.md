# \AuthApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_create**](AuthApi.md#auth_create) | **POST** /auth | Create API Key
[**connect_token_create**](AuthApi.md#connect_token_create) | **POST** /connect_token | Create Connect Token



## auth_create

> models::AuthResponse auth_create(auth_request)
Create API Key

Validate clientId and clientSecret and return an API Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_request** | [**AuthRequest**](AuthRequest.md) |  | [required] |

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connect_token_create

> models::ConnectTokenResponse connect_token_create(connect_token_request)
Create Connect Token

Creates a connect token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connect_token_request** | Option<[**ConnectTokenRequest**](ConnectTokenRequest.md)> | Create connect token payload |  |

### Return type

[**models::ConnectTokenResponse**](ConnectTokenResponse.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

