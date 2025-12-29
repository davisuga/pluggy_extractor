# \PaymentRecipientApi

All URIs are relative to *https://api.pluggy.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_recipient_create**](PaymentRecipientApi.md#payment_recipient_create) | **POST** /payments/recipients | Create
[**payment_recipient_delete**](PaymentRecipientApi.md#payment_recipient_delete) | **DELETE** /payments/recipients/{id} | Delete
[**payment_recipient_institutions_retrieve**](PaymentRecipientApi.md#payment_recipient_institutions_retrieve) | **GET** /payments/recipients/institutions/{id} | Retrieve Institution
[**payment_recipient_retrieve**](PaymentRecipientApi.md#payment_recipient_retrieve) | **GET** /payments/recipients/{id} | Retrieve
[**payment_recipient_update**](PaymentRecipientApi.md#payment_recipient_update) | **PATCH** /payments/recipients/{id} | Update
[**payment_recipients_institution_list**](PaymentRecipientApi.md#payment_recipients_institution_list) | **GET** /payments/recipients/institutions | List Institutions
[**payment_recipients_list**](PaymentRecipientApi.md#payment_recipients_list) | **GET** /payments/recipients | List



## payment_recipient_create

> models::PaymentRecipient payment_recipient_create(create_payment_recipient)
Create

Creates the payment recipient resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_recipient** | [**CreatePaymentRecipient**](CreatePaymentRecipient.md) |  | [required] |

### Return type

[**models::PaymentRecipient**](PaymentRecipient.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_recipient_delete

> payment_recipient_delete(id)
Delete

Deletes the payment recipient resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment recipient primary identifier | [required] |

### Return type

 (empty response body)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_recipient_institutions_retrieve

> models::PaymentInstitution payment_recipient_institutions_retrieve(id)
Retrieve Institution

Recovers the payment institution resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment institution primary identifier | [required] |

### Return type

[**models::PaymentInstitution**](PaymentInstitution.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_recipient_retrieve

> models::PaymentRecipient payment_recipient_retrieve(id)
Retrieve

Recovers the payment recipient resource by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment recipient primary identifier | [required] |

### Return type

[**models::PaymentRecipient**](PaymentRecipient.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_recipient_update

> models::PaymentRecipient payment_recipient_update(id, update_payment_recipient)
Update

Updates the payment recipient resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Payment recipient primary identifier | [required] |
**update_payment_recipient** | [**UpdatePaymentRecipient**](UpdatePaymentRecipient.md) |  | [required] |

### Return type

[**models::PaymentRecipient**](PaymentRecipient.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_recipients_institution_list

> models::PaymentRecipientsInstitutionList200Response payment_recipients_institution_list(page_size, page, name)
List Institutions

Recovers all created payment institutions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**f64**> | Page size for the paging request, default: 500 |  |
**page** | Option<**f64**> | Page number for the paging request, default: 1 |  |
**name** | Option<**String**> | Filter institutions by name |  |

### Return type

[**models::PaymentRecipientsInstitutionList200Response**](payment_recipients_institution_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_recipients_list

> models::PaymentRecipientsList200Response payment_recipients_list(page_size, page, is_default, pix_key, name)
List

Recovers all created payment recipients

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**f64**> | Page size for the paging request, default: 500 |  |
**page** | Option<**f64**> | Page number for the paging request, default: 1 |  |
**is_default** | Option<**bool**> | Filter connectors by the `isDefault` attribute. If not sent, it won't filter. |  |
**pix_key** | Option<**String**> | Filter payment recipient by Pix key |  |
**name** | Option<**String**> | Filter payment recipient by name |  |

### Return type

[**models::PaymentRecipientsList200Response**](payment_recipients_list_200_response.md)

### Authorization

[default](../README.md#default)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

