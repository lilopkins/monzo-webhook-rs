use chrono::{DateTime, Utc};

struct_with_extra! { no_extra LedgerDetails,
    ledger_committed_timestamp_earliest: DateTime<Utc>,
    ledger_committed_timestamp_latest: DateTime<Utc>,
    ledger_insertion_id: Option<String>,
    ledger_entry_intent: Option<LedgerEntryIntent>,
    ledger_intent: Option<LedgerIntent>,
}

enum_with_extra! {
    "snake_case" => LedgerIntent,
    /// Money has been sent to another Monzo user
    #[serde(rename = "monzo-to-monzo.transferred")]
    MonzoToMonzoTransferred,
}

enum_with_extra! {
    "snake_case" => LedgerEntryIntent,
    /// Money is transferred to the Flex account as part of a repayment
    #[serde(rename = "monzo-flex.customer_makes_repayment_into_flex_main")]
    FlexRepayment,
    /// Money is transferred from the Flex account to the regular bank
    /// account
    #[serde(rename = "monzo-flex.customer_move_money_transfer_from_monzo_flex")]
    FlexTransferMoney,
}
