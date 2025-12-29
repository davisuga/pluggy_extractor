# LoanInstallments

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type_number_of_installments** | Option<**String**> | Type of total term of the contract referring to the type of credit informed | [optional]
**total_number_of_installments** | Option<**f64**> | Total term according to the type referring to the type of credit informed | [optional]
**type_contract_remaining** | Option<**String**> | Type of remaining term of the contract referring to the type of credit informed | [optional]
**contract_remaining_number** | Option<**f64**> | Remaining term according to the type referring to the credit type informed | [optional]
**paid_installments** | Option<**f64**> | Number of paid installments | [optional]
**due_installments** | Option<**f64**> | Number of due installments | [optional]
**past_due_installments** | Option<**f64**> | Number of overdue installments | [optional]
**balloon_payments** | Option<[**Vec<models::LoanInstallmentBalloonPayment>**](LoanInstallmentBalloonPayment.md)> | List that brings the due dates and value of the non-regular installments of the contract of the type of credit consulted | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


