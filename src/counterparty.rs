use std::collections::HashMap;

enum_with_extra! {
    untagged CounterpartyOrNone,
    Counterparty(Counterparty),
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
