struct_with_extra! {
    /// A move of money between pots or accounts
    MoneyTransfer,
    external_id: String,
    move_money_transfer_id: String,
    pot_account_id: String,
    pot_id: String,
    transaction_description_localised: String,
    transaction_locale_country: String,
    trigger: String,
    user_id: String,
    /// Only present on a pot withdrawal
    money_transfer_id: Option<String>,
    /// Only present on a pot withdrawal
    pot_withdrawal_id: Option<String>,
    /// Only present on a pot deposit
    pot_deposit_id: Option<String>,
    #[serde(flatten)]
    hold_decision: Option<super::shared_metadata::HoldDecision>,
    #[serde(flatten)]
    ledger_details: super::ledger::LedgerDetails,
}
