use crate::StringBoolean;

struct_with_extra! { FlexTransaction,
    monzo_flex_id: String,
    flex_transaction_type: FlexTransactionType,
    exclude_from_breakdown: StringBoolean,
    notes: String,
    fee_amount: Option<String>,
    instalment_plan_integrity_hash: Option<String>,
    backing_loan_id: Option<String>,
    instalment_plan_id: Option<String>,
    transfer_id: Option<String>,
    #[serde(flatten)]
    ledger_details: super::ledger::LedgerDetails,
}

enum_with_extra! {
    "snake_case" => FlexTransactionType,
    Repayment,
    FlexMoveMoneyTransferCurrentAccountSide,
}
