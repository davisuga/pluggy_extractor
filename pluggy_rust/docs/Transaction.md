# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Primary identifier of the transaction | 
**description** | **String** | Clean description of the transaction | 
**currency_code** | **String** | Currency ISO code | 
**amount** | **f64** | Transaction amount | 
**amount_in_account_currency** | Option<**f64**> | Transaction amount in Account's Currency. Only present if the transaction is in a different currency than the account's currency | [optional]
**date** | **String** | Date when the transaction was made | 
**r#type** | Option<**String**> | Type of the transaction. DEBIT (outflow) or CREDIT (inflow) | [optional]
**balance** | Option<**f64**> | Balance after the transaction | [optional]
**provider_code** | Option<**String**> | Institution provided code | [optional]
**status** | Option<**String**> | Status of the movement. POSTED / PENDING | [optional]
**category** | Option<**String**> | Category of the transaction (e.g. Restaurants, Education). See the Transaction Categorization section in our guides. | [optional]
**category_id** | Option<**String**> | Id of the transaction category. Can be used to identify the category in the Categories endpoint | [optional]
**payment_data** | Option<[**models::PaymentData**](PaymentData.md)> |  | [optional]
**credit_card_metadata** | Option<[**models::CreditCardMetadata**](CreditCardMetadata.md)> |  | [optional]
**merchant** | Option<[**models::Merchant**](Merchant.md)> |  | [optional]
**operation_type** | Option<**String**> | Type of operation classified by the institution. | [optional]
**provider_id** | Option<**String**> | Provider's identifier for the transaction. Only returned for Open Finance connectors. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


