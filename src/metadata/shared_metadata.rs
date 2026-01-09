struct_with_extra! { no_extra HoldDecision,
    hold_decision_duration: String,
    hold_decision_status: HoldDecisionStatus,
}

enum_with_extra! { "snake_case" => HoldDecisionStatus,
    #[serde(rename = "decision_status.released")]
    Released,
}
