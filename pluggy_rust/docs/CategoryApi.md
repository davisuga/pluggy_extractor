# \CategoryApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**categories_list**](CategoryApi.md#categories_list) | **GET** /categories | List
[**categories_retrieve**](CategoryApi.md#categories_retrieve) | **GET** /categories/{id} | Retrieve
[**client_category_rules_create**](CategoryApi.md#client_category_rules_create) | **POST** /categories/rules | Create Category Rule
[**client_category_rules_list**](CategoryApi.md#client_category_rules_list) | **GET** /categories/rules | List Category Rules



## categories_list

> Vec<models::Category> categories_list(parent_id)
List

Recovers all categories active from the data categorization. Can be filtered by the parentId of the category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_id** | Option<**String**> | Parent's primary identifier |  |

### Return type

[**Vec<models::Category>**](Category.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## categories_retrieve

> models::Category categories_retrieve(id)
Retrieve

Recovers the category resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | category primary identifier | [required] |

### Return type

[**models::Category**](Category.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## client_category_rules_create

> models::ClientCategoryRule client_category_rules_create(create_client_category_rule)
Create Category Rule

Create a single category rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_client_category_rule** | [**CreateClientCategoryRule**](CreateClientCategoryRule.md) |  | [required] |

### Return type

[**models::ClientCategoryRule**](ClientCategoryRule.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## client_category_rules_list

> Vec<models::PageResponseCategoryRules> client_category_rules_list()
List Category Rules

Recovers category rules

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PageResponseCategoryRules>**](PageResponseCategoryRules.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

