# \PaymentCustomerApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_customer_create**](PaymentCustomerApi.md#payment_customer_create) | **POST** /payments/customers | Create
[**payment_customer_delete**](PaymentCustomerApi.md#payment_customer_delete) | **DELETE** /payments/customers/{id} | Delete
[**payment_customer_retrieve**](PaymentCustomerApi.md#payment_customer_retrieve) | **GET** /payments/customers/{id} | Retrieve
[**payment_customer_update**](PaymentCustomerApi.md#payment_customer_update) | **PATCH** /payments/customers/{id} | Update
[**payment_customers_list**](PaymentCustomerApi.md#payment_customers_list) | **GET** /payments/customers | List



## payment_customer_create

> models::PaymentCustomer payment_customer_create(create_payment_customer_request_body)
Create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_customer_request_body** | [**CreatePaymentCustomerRequestBody**](CreatePaymentCustomerRequestBody.md) |  | [required] |

### Return type

[**models::PaymentCustomer**](PaymentCustomer.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_customer_delete

> payment_customer_delete(id)
Delete

Deletes the payment customer resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment customer primary identifier | [required] |

### Return type

 (empty response body)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_customer_retrieve

> models::PaymentCustomer payment_customer_retrieve(id)
Retrieve

Recovers the payment customer resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment customer primary identifier | [required] |

### Return type

[**models::PaymentCustomer**](PaymentCustomer.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_customer_update

> models::PaymentCustomer payment_customer_update(id, create_or_update_payment_customer)
Update

Updates the payment customer resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment customer primary identifier | [required] |
**create_or_update_payment_customer** | [**CreateOrUpdatePaymentCustomer**](CreateOrUpdatePaymentCustomer.md) |  | [required] |

### Return type

[**models::PaymentCustomer**](PaymentCustomer.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_customers_list

> models::PaymentCustomersList200Response payment_customers_list(page_size, page, name, email, cpf, cnpj)
List

Recovers all created payment customers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**f64**> | Page size for the paging request, default: 500 |  |
**page** | Option<**f64**> | Page number for the paging request, default: 1 |  |
**name** | Option<**String**> | Filter payment customers by name |  |
**email** | Option<**String**> | Filter payment customers by email |  |
**cpf** | Option<**String**> | Filter payment customers by CPF |  |
**cnpj** | Option<**String**> | Filter payment customers by CNPJ |  |

### Return type

[**models::PaymentCustomersList200Response**](payment_customers_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

