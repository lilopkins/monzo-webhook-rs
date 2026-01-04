#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![allow(
    clippy::struct_excessive_bools,
    reason = "structs cannot be changed due to serialization"
)]

use chrono::{DateTime, Utc};
use serde::Deserialize;

use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[cfg(feature = "decode_everything")]
pub type ExtraValues = HashMap<String, serde_json::Value>;

/// The main webhook data type.
#[derive(Clone, Debug, Deserialize)]
pub struct Webhook {
    /// The type of webhook update.
    /// Always one of:
    /// - `transaction.created`
    /// - `transaction.updated`
    pub r#type: String,
    pub data: WebhookData,
    #[cfg(feature = "decode_everything")]
    #[cfg_attr(feature = "decode_everything", serde(flatten))]
    pub extra: ExtraValues,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebhookData {
    pub id: String,
    pub created: DateTime<Utc>,
    pub description: String,
    /// The amount of money in the transaction, in whole pence (or equivalent for foreign currency)
    pub amount: i64,
    /// The ISO 4127 currency code of [`Self::amount`]
    pub currency: String,
    pub is_load: bool,
    pub settled: SettledTimestamp,
    /// The amount of money in the transaction, in whole pence (or equivalent for foreign currency)
    pub local_amount: i64,
    /// The ISO 4127 currency code of [`Self::local_amount`]
    pub local_currency: String,
    pub merchant: Option<Merchant>,
    pub merchant_feedback_uri: String,
    pub notes: String,
    pub metadata: WebhookMetadata,
    pub category: String,
    pub updated: DateTime<Utc>,
    pub account_id: String,
    pub user_id: String,
    pub counterparty: CounterpartyOrNone,
    pub scheme: String,
    pub dedupe_id: String,
    pub originator: bool,
    pub include_in_spending: bool,
    pub can_be_excluded_from_breakdown: bool,
    pub can_be_made_subscription: bool,
    pub can_split_the_bill: bool,
    pub can_add_to_tab: bool,
    pub can_match_transactions_in_categorization: bool,
    pub amount_is_pending: bool,
    pub parent_account_id: String,
    pub categories: Option<HashMap<String, i64>>,
    #[cfg(feature = "decode_everything")]
    #[cfg_attr(feature = "decode_everything", serde(flatten))]
    pub extra: ExtraValues,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum SettledTimestamp {
    Settled(DateTime<Utc>),
    /// If not yet settled, a string it returned, however it always seems to be empty.
    NotYetSettled(String),
    #[cfg(feature = "decode_everything")]
    SomethingElse(std::collections::HashMap<String, serde_json::Value>),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Merchant {
    pub id: String,
    pub group_id: String,
    pub name: String,
    pub logo: String,
    pub emoji: String,
    pub category: String,
    pub online: bool,
    pub atm: bool,
    pub address: MerchantAddress,
    pub disable_feedback: bool,
    pub suggested_tags: String,
    pub metadata: MerchantMetadata,
    #[cfg(feature = "decode_everything")]
    #[cfg_attr(feature = "decode_everything", serde(flatten))]
    pub extra: ExtraValues,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MerchantAddress {
    pub short_formatted: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub zoom_level: u64,
    pub approximate: bool,
    pub formatted: String,
    pub address: String,
    pub region: String,
    pub country: String,
    pub postcode: String,
    #[cfg(feature = "decode_everything")]
    #[cfg_attr(feature = "decode_everything", serde(flatten))]
    pub extra: ExtraValues,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MerchantMetadata {
    pub suggested_tags: String,
    pub website: String,
    #[cfg(feature = "decode_everything")]
    #[cfg_attr(feature = "decode_everything", serde(flatten))]
    pub extra: ExtraValues,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebhookMetadata {
    #[serde(flatten)]
    pub subtype: WebhookMetadataSubtype,
    pub ledger_committed_timestamp_earliest: DateTime<Utc>,
    pub ledger_committed_timestamp_latest: DateTime<Utc>,
    #[cfg(feature = "decode_everything")]
    #[cfg_attr(feature = "decode_everything", serde(flatten))]
    pub extra: ExtraValues,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum WebhookMetadataSubtype {
    FasterPayment(FasterPayment),
    MoneyTransfer(MoneyTransfer),
    MerchantTransaction(MerchantTransaction),
    #[cfg(feature = "decode_everything")]
    SomethingElse(std::collections::HashMap<String, serde_json::Value>),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum CounterpartyOrNone {
    Counterparty(Counterparty),
    None {},
    #[cfg(feature = "decode_everything")]
    SomethingElse(std::collections::HashMap<String, serde_json::Value>),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Counterparty {
    pub account_number: String,
    pub name: String,
    pub sort_code: String,
    pub user_id: String,
    #[cfg(feature = "decode_everything")]
    #[cfg_attr(feature = "decode_everything", serde(flatten))]
    pub extra: ExtraValues,
}

/// A Faster Payments transaction
#[derive(Clone, Debug, Deserialize)]
pub struct FasterPayment {
    pub faster_payment: String,
    pub fps_fpid: String,
    pub fps_payment_id: String,
    pub insertion: String,
    pub notes: String,
    pub standin_correlation_id: String,
    pub trn: String,
    #[cfg(feature = "decode_everything")]
    #[cfg_attr(feature = "decode_everything", serde(flatten))]
    pub extra: ExtraValues,
}

/// A move of money between pots or accounts
#[derive(Clone, Debug, Deserialize)]
pub struct MoneyTransfer {
    #[serde(flatten)]
    pub subtype: MoneyTransferSubtype,
    pub external_id: String,
    pub ledger_insertion_id: String,
    pub move_money_transfer_id: String,
    pub pot_account_id: String,
    pub pot_id: String,
    pub transaction_description_localised: String,
    pub transaction_locale_country: String,
    pub trigger: String,
    pub user_id: String,
    #[cfg(feature = "decode_everything")]
    #[cfg_attr(feature = "decode_everything", serde(flatten))]
    pub extra: ExtraValues,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum MoneyTransferSubtype {
    PotWithdrawal(PotWithdrawal),
    PotDeposit(PotDeposit),
    #[cfg(feature = "decode_everything")]
    SomethingElse(std::collections::HashMap<String, serde_json::Value>),
}

#[derive(Clone, Debug, Deserialize)]
pub struct PotWithdrawal {
    pub money_transfer_id: String,
    pub pot_withdrawal_id: String,
    #[cfg(feature = "decode_everything")]
    #[cfg_attr(feature = "decode_everything", serde(flatten))]
    pub extra: ExtraValues,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PotDeposit {
    pub pot_deposit_id: String,
    #[cfg(feature = "decode_everything")]
    #[cfg_attr(feature = "decode_everything", serde(flatten))]
    pub extra: ExtraValues,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MerchantTransaction {
    pub mcc: String,
    pub token_transaction_identifier: String,
    pub tokenization_method: String,
    pub transaction_description_localised: String,
    pub transaction_locale_country: String,
    pub standin_correlation_id: String,
    pub token_unique_reference: String,
    pub ledger_insertion_id: String,
    pub mastercard_lifecycle_id: String,
    pub mastercard_approval_type: String,
    pub mastercard_auth_message_id: String,
    pub mastercard_card_id: String,
}
