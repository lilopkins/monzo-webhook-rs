struct_with_extra! { no_extra Decline,
    decline_reason: DeclineReason,
    decline_copy: String,
}

enum_with_extra! { "SCREAMING_SNAKE_CASE" => DeclineReason,
    InsufficientFunds,
}
