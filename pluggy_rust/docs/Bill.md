# Bill

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Primary identifier | 
**due_date** | **String** | Due date of the bill, displayed for payment by the customer | 
**total_amount** | **f64** | Total bill amount | 
**total_amount_currency_code** | **String** | Code referencing the currency of the bill | 
**minimum_payment_amount** | Option<**f64**> | Minimum payment amount of the bill | [optional]
**allows_installments** | Option<**bool**> | Indicates whether the bill allows installment payments (true) or not (false) | [optional]
**finance_charges** | [**Vec<models::BillFinanceCharge>**](BillFinanceCharge.md) | List of charges associated to the bill | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


