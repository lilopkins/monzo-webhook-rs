use std::collections::HashMap;

enum_with_extra! {
    untagged CounterpartyOrNone,
    Counterparty(Counterparty),
    #[allow(clippy::zero_sized_map_values, reason = "this needs refactor when we establish type anyway")]
    None(HashMap<(), ()>),
}

struct_with_extra! { Counterparty,
    account_id: Option<String>,
    account_number: String,
    name: String,
    sort_code: String,
    user_id: String,
    beneficiary_account_type: Option<BeneficiaryAccountType>,
}

enum_with_extra! {
    "PascalCase" => BeneficiaryAccountType,
    Personal,
}
