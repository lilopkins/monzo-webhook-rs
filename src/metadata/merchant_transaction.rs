use crate::StringBoolean;

struct_with_extra! { MerchantTransaction,
    mcc: String,
    /// Seems to only be present for transactions made via NFC, i.e.
    /// Google Pay, Apple Pay, Samsung Wallet, etc.
    token_transaction_identifier: Option<String>,
    /// Seems to only be present for transactions made via NFC, i.e.
    /// Google Pay, Apple Pay, Samsung Wallet, etc.
    tokenization_method: Option<String>,
    /// Seems to only be present for transactions made via NFC, i.e.
    /// Google Pay, Apple Pay, Samsung Wallet, etc.
    token_unique_reference: Option<String>,
    transaction_description_localised: StringBoolean,
    transaction_locale_country: String,
    standin_correlation_id: String,
    mastercard_lifecycle_id: String,
    mastercard_approval_type: MastercardApprovalType,
    mastercard_auth_message_id: String,
    mastercard_card_id: String,
    /// Set once the transaction has been cleared, often the next
    /// working day
    mastercard_clearing_message_id: Option<String>,
    /// This field is present (and [`StringBoolean::True`]) when the
    /// transaction is approved to be reduced later, for example fuel
    /// pay-at-pump transactions.
    mastercard_partial_approval_supported: Option<StringBoolean>,
    /// Present during online transactions
    card_acceptor_contact_number: Option<String>,
    /// Present during online transactions
    card_acceptor_website: Option<String>,
    /// The ID of the transaction if you decide to round up into a
    /// savings account
    coin_jar_transaction: Option<String>,
    #[serde(flatten)]
    ledger_details: super::ledger::LedgerDetails,
}

enum_with_extra! { "snake_case" => MastercardApprovalType,
    Full,
}
