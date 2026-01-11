use chrono::NaiveDate;

use crate::StringBoolean;

struct_with_extra! {
    /// A move of money between pots or accounts
    MoneyTransfer,
    external_id: String,
    move_money_transfer_id: Option<String>,
    pot_account_id: Option<String>,
    pot_id: String,
    pot_name: Option<String>,
    transaction_description_localised: Option<StringBoolean>,
    transaction_locale_country: Option<String>,
    trigger: MoneyTransferTrigger,
    user_id: Option<String>,
    /// Only present on a pot withdrawal
    money_transfer_id: Option<String>,
    /// Only present on a pot withdrawal
    pot_withdrawal_id: Option<String>,
    /// Only present on a pot deposit
    pot_deposit_id: Option<String>,
    covering_date: Option<NaiveDate>,
    #[serde(flatten)]
    hold_decision: Option<super::shared_metadata::HoldDecision>,
    #[serde(flatten)]
    ledger_details: super::ledger::LedgerDetails,
}

enum_with_extra! { "snake_case" => MoneyTransferTrigger,
    User,
    SavingsChallenge,
}
