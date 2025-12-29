# Loan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Primary identifier | 
**item_id** | [**uuid::Uuid**](uuid::Uuid.md) | Identifier of the item linked to the loan | 
**contract_number** | Option<**String**> | Contract number given by the contracting institution | [optional]
**ipoc_code** | Option<**String**> | Standard contract number - IPOC (Identificação Padronizada da Operação de Crédito) | [optional]
**product_name** | **String** | Denomination/Identification of the name of the credit operation disclosed to the customer | 
**r#type** | Option<**String**> | Loan type (https://openbanking-brasil.github.io/openapi/swagger-apis/loans/?urls.primaryName=2.0.1#model-EnumContractProductSubTypeLoans) | [optional]
**date** | **String** | Date when the loan data was collected | 
**contract_date** | Option<**String**> | Date when the loan was contracted | [optional]
**disbursement_dates** | Option<[**Vec<String>**](string.md)> | Disbursement date of the contracted amount | [optional]
**settlement_date** | Option<**String**> | Loan settlement date | [optional]
**contract_amount** | Option<**f64**> | Loan contracted value | [optional]
**currency_code** | **String** | Code referencing the currency of the loan | 
**due_date** | Option<**String**> | Loan due date | [optional]
**installment_periodicity** | Option<**String**> | Installments regular frequency | [optional]
**installment_periodicity_additional_info** | Option<**String**> | Mandatory field to complement the information regarding the regular payment frequency when installmentPeriodicity has value 'OTHERS' | [optional]
**first_installment_due_date** | Option<**String**> | First installment due date | [optional]
**cet** | Option<**f64**> | CET - Custo Efetivo Total must be expressed as an annual percentage rate and incorporates all charges and expenses incurred in credit operations (interest rate, but also tariffs, taxes, insurance and other expenses charged) | [optional]
**amortization_scheduled** | Option<**String**> | Amortization system (https://openbanking-brasil.github.io/openapi/swagger-apis/loans/?urls.primaryName=2.0.1#model-EnumContractAmortizationScheduled) | [optional]
**amortization_scheduled_additional_info** | Option<**String**> | Mandatory field to complement the information regarding the scheduled amortization when it has value 'OTHERS' | [optional]
**cnpj_consignee** | Option<**String**> | Consignor CNPJ | [optional]
**interest_rates** | Option<[**Vec<models::LoanInterestRate>**](LoanInterestRate.md)> |  | [optional]
**contracted_fees** | Option<[**Vec<models::LoanContractedFee>**](LoanContractedFee.md)> | List that brings the information of the tariffs agreed in the contract. | [optional]
**contracted_finance_charges** | Option<[**Vec<models::LoanContractedFinanceCharge>**](LoanContractedFinanceCharge.md)> | List that brings the charges agreed in the contract | [optional]
**warranties** | Option<[**Vec<models::LoanWarranty>**](LoanWarranty.md)> |  | [optional]
**installments** | Option<[**models::LoanInstallments**](LoanInstallments.md)> |  | [optional]
**payments** | Option<[**models::LoanPayments**](LoanPayments.md)> | Loan contract payment data | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


