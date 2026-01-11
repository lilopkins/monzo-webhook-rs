use crate::StringBoolean;

struct_with_extra! { MonzoToMonzoTransfer,
    device_fingerprint: String,
    money_transfer_id: String,
    monzo_to_monzo_transfer_id: String,
    client_idempotency_key: String,
    share_detected: StringBoolean,
    trn: String,
    ip_address_attempt: String,
    outbound_payment_trace_id: String,
    payment_source: String,
    addressed_by: AddressedBy,
    coach_detected: StringBoolean,
    transaction_description_localised: StringBoolean,
    transaction_locale_country: String,
    duplicate_payment_prompt_enabled: StringBoolean,
    confirmation_of_payee_requester_id: Option<String>,
    confirmation_of_payee_decision_id: Option<String>,
    #[serde(flatten)]
    hold_decision: Option<super::shared_metadata::HoldDecision>,
    #[serde(flatten)]
    ledger_details: super::ledger::LedgerDetails,
}

enum_with_extra! { "snake_case" => AddressedBy,
    ExternalUkBankAccount,
}
