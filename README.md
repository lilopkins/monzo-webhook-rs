# Monzo Webhook

Serde-compatible structs containing the data from a Monzo bank webhook.

## Usage

When implemented a receiver for a Monzo bank webhook, you can use these
structs. For example, with axum:

```notest
async fn test_webhook_parse(Json(data): Json<monzo_webhook::Webhook>) -> String {
    format!("{data:#?}")
}
```

## Testing

This has been tested against the following types of transactions:

- Faster Payments in- and outbound
- Pot and Account Deposits and Withdrawals
- Contactless purchases

### This needs confirmation that it works with:

- ATM withdrawals
- ATM deposits
- Deposits
- Cheques
- Online purchases
- Online refunds
- Chip & PIN purchases
- Chip & PIN refunds
- Contactless refunds
- Direct Debits
- Standing Orders
- BACS Credits
- BACS Debits
- CHAPS Transfers
- International Transfers via Swift
- Cash deposits
- Fees and Charges
- Interest Paid
- Interest Earned

If you have any JSON webhook payloads of these types of transactions, I
would appreciate seeing them. You are recommended to scrub any sensitive
data or ID numbers first, then please open an issue.
