# IssuedBoleto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Primary identifier | 
**amount** | **f64** | Boleto amount | 
**status** | **String** | Current status of the boleto | 
**seu_numero** | **String** | Your identifier for this boleto | 
**due_date** | **String** | Due date of the boleto | 
**payer** | [**models::IssuedBoletoPayer**](IssuedBoleto_payer.md) |  | 
**pix_qr** | Option<**String**> | PIX QR code for payment | [optional]
**digitable_line** | **String** | Boleto digitable line | 
**nosso_numero** | Option<**String**> | Bank's internal identifier for the boleto | [optional]
**barcode** | **String** | Boleto barcode | 
**boleto_connection_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the boleto connection used to create this boleto | 
**created_at** | **String** | Date when the boleto was created | 
**amount_paid** | Option<**f64**> | Amount that was paid for this boleto | [optional]
**payment_origin** | Option<**String**> | Origin of the payment when the boleto is paid | [optional]
**fine** | Option<[**models::IssuedBoletoFine**](IssuedBoleto_fine.md)> |  | [optional]
**interest** | Option<[**models::IssuedBoletoInterest**](IssuedBoleto_interest.md)> |  | [optional]
**paid_at** | Option<**String**> | Date when the boleto was paid | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


