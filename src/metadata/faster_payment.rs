use crate::StringBoolean;

struct_with_extra! {
    /// A Faster Payments transaction
    FasterPayment,
    faster_payment: StringBoolean,
    fps_fpid: String,
    fps_payment_id: String,
    insertion: String,
    standin_correlation_id: String,
    trn: String,
    #[serde(flatten)]
    ledger_details: super::ledger::LedgerDetails,
    client_idempotency_key: Option<String>,
    confirmation_of_payee_decision_id: Option<String>,
    confirmation_of_payee_requester_id: Option<String>,
    money_transfer_id: Option<String>,
    money_transfer_originating_id: Option<String>,
    duplicate_payment_prompt_enabled: Option<StringBoolean>,
    outbound_payment_trace_id: Option<String>,
    payment_source: Option<PaymentSource>,
    device_fingerprint: Option<String>,
    coach_detected: Option<StringBoolean>,
    faster_payment_initiator: Option<FasterPaymentInitiator>,
    #[serde(rename = "fps.trn")]
    fps_trn: Option<String>,
    share_detected: Option<StringBoolean>,
    notification_on_settle: Option<StringBoolean>,
    /// Present when on a potential series of recurring payments
    series_id: Option<String>,
    /// Present when on a potential series of recurring payments
    series_iteration_count: Option<String>,
    #[serde(flatten)]
    hold_decision: Option<super::shared_metadata::HoldDecision>,
    action_code: Option<String>,
}

enum_with_extra! { "snake_case" => PaymentSource,
    Payments,
}

enum_with_extra! { "snake_case" => FasterPaymentInitiator,
    Customer,
    ScheduledPayment,
}
