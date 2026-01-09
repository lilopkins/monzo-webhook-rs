#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![allow(
    clippy::struct_excessive_bools,
    reason = "structs cannot be changed due to serialization"
)]

use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub use crate::string_boolean::StringBoolean;

#[cfg(feature = "decode_everything")]
pub type ExtraValues = HashMap<String, serde_json::Value>;

#[cfg(feature = "decode_everything")]
pub mod has_extra_data;
#[cfg(feature = "decode_everything")]
pub use has_extra_data::*;

/// Define an enum with an automatic `SomethingElse` variant that
/// consumes any fields when no other variants match, but only when the
/// `decode_everything` feature is enabled.
macro_rules! enum_with_extra {
    (
        $(#[$attrs: meta])*
        untagged $name: ident, $($(#[$variant_attrs: meta])* $variant: ident($variant_inner_ty: ty),)*
    ) => {
        $(#[$attrs])*
        #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
        #[serde(untagged)]
        pub enum $name {
            $($(#[$variant_attrs])* $variant($variant_inner_ty),)*

            /// The value didn't match anything currently parsed.
            #[cfg(feature = "decode_everything")]
            SomethingElse(crate::ExtraValues),
        }

        #[cfg(feature = "decode_everything")]
        impl crate::HasExtraData for $name {
            fn has_extra_data(&self) -> bool {
                match self {
                    Self::SomethingElse(_vals) => true,
                    $(Self::$variant(inner) => inner.has_extra_data(),)*
                }
            }
        }
    };
    (
        $case: expr => $(#[$attrs: meta])* $name: ident,
        $($(#[$variant_attrs: meta])* $variant: ident,)*
    ) => {
        $(#[$attrs])*
        #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
        #[serde(rename_all = $case)]
        pub enum $name {
            $($(#[$variant_attrs])* $variant,)*

            /// The value didn't match anything currently parsed.
            #[cfg(feature = "decode_everything")]
            #[serde(untagged)]
            SomethingElse(String),
        }

        #[cfg(feature = "decode_everything")]
        impl crate::HasExtraData for $name {
            fn has_extra_data(&self) -> bool {
                match self {
                    Self::SomethingElse(_value) => true,
                    _ => false,
                }
            }
        }
    };
}

/// Define a struct with an automatic `extra` field that consumes any
/// remaining fields, but only when the `decode_everything` feature is
/// enabled.
macro_rules! struct_with_extra {
    (
        no_extra $(#[$attrs: meta])*
        $name: ident, $($(#[$field_attrs: meta])* $field_name: ident: $field_typ: ty,)*
    ) => {
        $(#[$attrs])*
        #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
        pub struct $name {
            $($(#[$field_attrs])* pub $field_name: $field_typ,)*
        }

        #[cfg(feature = "decode_everything")]
        impl crate::HasExtraData for $name {
            fn has_extra_data(&self) -> bool {
                false $(|| self.$field_name.has_extra_data())*
            }
        }
    };
    (
        $(#[$attrs: meta])*
        $name: ident, $($(#[$field_attrs: meta])* $field_name: ident: $field_typ: ty,)*
    ) => {
        $(#[$attrs])*
        #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
        pub struct $name {
            $($(#[$field_attrs])* pub $field_name: $field_typ,)*

            /// The value had extra fields that weren't parsed into
            /// another field.
            #[cfg(feature = "decode_everything")]
            #[cfg_attr(feature = "decode_everything", serde(flatten))]
            pub extra: crate::ExtraValues,
        }

        #[cfg(feature = "decode_everything")]
        impl crate::HasExtraData for $name {
            fn has_extra_data(&self) -> bool {
                let this_has_extra = !self.extra.is_empty();
                this_has_extra
                    $(|| self.$field_name.has_extra_data())*
            }
        }
    };
}

#[cfg(test)]
mod tests;

pub mod counterparty;
pub mod merchant;
pub mod metadata;
pub mod string_boolean;

struct_with_extra! {
    /// The main webhook data type.
    Webhook,
    /// The type of webhook update.
    r#type: WebhookType,
    /// The webhook data
    data: WebhookData,
}

enum_with_extra! {
    "snake_case" => WebhookType,
    #[serde(rename = "transaction.created")]
    TransactionCreated,
    #[serde(rename = "transaction.updated")]
    TransactionUpdated,
}

struct_with_extra! { WebhookData,
    id: String,
    created: DateTime<Utc>,
    description: String,
    /// The amount of money in the transaction, in whole pence (or equivalent for foreign currency)
    amount: i64,
    /// The ISO 4127 currency code of [`Self::amount`]
    currency: String,
    is_load: bool,
    settled: SettledTimestamp,
    /// The amount of money in the transaction, in whole pence (or equivalent for foreign currency)
    local_amount: i64,
    /// The ISO 4127 currency code of [`Self::local_amount`]
    local_currency: String,
    merchant: Option<merchant::Merchant>,
    merchant_feedback_uri: String,
    notes: String,
    metadata: metadata::WebhookMetadata,
    category: String,
    updated: DateTime<Utc>,
    account_id: String,
    user_id: String,
    counterparty: counterparty::CounterpartyOrNone,
    scheme: Scheme,
    dedupe_id: String,
    originator: bool,
    include_in_spending: bool,
    can_be_excluded_from_breakdown: bool,
    can_be_made_subscription: bool,
    can_split_the_bill: bool,
    can_add_to_tab: bool,
    can_match_transactions_in_categorization: bool,
    amount_is_pending: bool,
    parent_account_id: String,
    categories: Option<HashMap<String, i64>>,
    // TODO The following fields are known about, but we don't know what types they hold
    attachments: (),
    atm_fees_detailed: (),
    #[allow(clippy::zero_sized_map_values, reason = "this needs refactor when we establish type anyway")]
    fees: HashMap<(), ()>,
    international: (),
    labels: (),
}

enum_with_extra! {
    untagged SettledTimestamp,
    Settled(DateTime<Utc>),
    /// If not yet settled, a string it returned, however it always seems to be empty.
    NotYetSettled(String),
}

enum_with_extra! { "snake_case" => Scheme,
    Mastercard,
    PayportFasterPayments,
    UkRetailPot,
    MonzoFlex,
    MonzoToMonzo,
}
