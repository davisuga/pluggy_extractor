# LoanPaymentRelease

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_over_parcel_payment** | Option<**bool**> | Identifies whether it is an agreed payment (false) or a one-time payment (true) | [optional]
**installment_id** | Option<**String**> | Installment identifier, responsibility of each transmitting Institution | [optional]
**paid_date** | Option<**String**> | Effective date of payment referring to the contract of the credit modality consulted | [optional]
**currency_code** | Option<**String**> | Code referencing the currency of the payment | [optional]
**paid_amount** | Option<**f64**> | Payment amount referring to the contract of the credit modality consulted | [optional]
**over_parcel** | Option<[**models::LoanPaymentReleaseOverParcel**](LoanPaymentReleaseOverParcel.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


