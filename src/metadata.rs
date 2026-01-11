pub mod faster_payment;
pub mod flex_transaction;
pub mod ledger;
pub mod merchant_transaction;
pub mod money_transfer;
pub mod monzo_to_monzo;
pub mod shared_metadata;

// no_extra is safe here as spare data is picked up elsewhere
struct_with_extra! { no_extra WebhookMetadata,
    notes: Option<String>,
    #[serde(flatten)]
    subtype: WebhookMetadataSubtype,
}

enum_with_extra! {
    untagged WebhookMetadataSubtype,
    FasterPayment(faster_payment::FasterPayment),
    MonzoToMonzoTransfer(monzo_to_monzo::MonzoToMonzoTransfer),
    MoneyTransfer(money_transfer::MoneyTransfer),
    MerchantTransaction(merchant_transaction::MerchantTransaction),
    FlexTransaction(flex_transaction::FlexTransaction),
}
