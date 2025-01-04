use crate::schema::{Args, IDInfo};
use crate::utils::factor_size_hex_bits_color_from_text;

const ALPHA_NUM: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn parse_stripe(args: &Args) -> Option<IDInfo> {
    if args.id.chars().count() > 43 {
        return None;
    }
    let parts: &Vec<&str> = &args.id.split('_').collect();
    let prefix: String;
    let value: &str;
    match parts.split_last() {
        Some((last, elements)) => {
            prefix = elements.join("_");
            value = last;
        }
        None => return None,
    }
    if !value.chars().all(|c| ALPHA_NUM.contains(c)) {
        return None;
    }
    let version = match prefix.as_str() {
        "ac" => "Platform Client ID",
        "acct" => "Account ID",
        "aliacc" => "Alipay Account ID",
        "ba" => "Bank Account ID",
        "btok" => "Bank Token ID",
        "card" => "Card ID",
        "cbtxn" => "Customer Balance Transaction ID",
        "ch" => "Charge ID",
        "cn" => "Credit Note ID",
        "cs_live" => "Live Checkout Session ID",
        "cs_test" => "Test Checkout Session ID",
        "cus" => "Customer ID",
        "dp" => "Dispute ID",
        "evt" => "Event ID",
        "fee" => "Application Fee ID",
        "file" => "File ID",
        "fr" => "Application Fee Refund ID",
        "iauth" => "Issuing Authorization ID",
        "ic" => "Issuing Card ID",
        "ich" => "Issuing Card Holder ID",
        "idp" => "Issuing Dispute ID",
        "ii" => "Invoice Item ID",
        "il" => "Invoice Line Item ID",
        "in" => "Invoice ID",
        "ipi" => "Issuing Transaction ID",
        "link" => "File Link ID",
        "or" => "Order ID",
        "orret" => "Order Return ID",
        "person" => "Person ID",
        "pi" => "Payment Intent ID",
        "pk_live" => "Live public key",
        "pk_test" => "Test public key",
        "pm" => "Payment Method ID",
        "po" => "Payout ID",
        "price" => "Price ID",
        "prod" => "Product ID",
        "prv" => "Review ID",
        "pst_live" => "Live Connection token",
        "pst_test" => "Test Connection token",
        "py" => "Payment ID",
        "pyr" => "Payment Refund ID",
        "qt" => "Quote ID",
        "rcpt" => "Receipt ID",
        "re" => "Refund ID",
        "req" => "Request ID",
        "rk_live" => "Live restricted key",
        "rk_test" => "Test restricted key",
        "seti" => "Setup Intent ID",
        "si" => "Subscription Item ID",
        "sk_live" => "Live secret key",
        "sk_test" => "Test secret key",
        "sku" => "SKU ID",
        "sli" => "Subscription Line Item ID",
        "sqr" => "Scheduled Query Run ID",
        "src" => "Source ID",
        "sub" => "Subscription ID",
        "tml" => "Terminal Location ID",
        "tmr" => "Terminal Reader ID",
        "tok" => "Token ID",
        "trr" => "Transfer ID",
        "tu" => "Topup ID",
        "txi" => "Tax ID",
        "txn" => "Transaction ID",
        "txr" => "Tax Rate ID",
        "we" => "Webhook Endpoint ID",
        "whsec" => "Webhook Secret",
        _ => return None,
    };
    let (size, hex, bits, _) = factor_size_hex_bits_color_from_text(&args.id);
    let prefix_bits = prefix.chars().count() * 8;
    let code_bits = ((args.id.chars().count() * 8) - prefix_bits - 8) as u16;

    Some(IDInfo {
        id_type: "Stripe ID".to_string(),
        version: Some(version.to_string()),
        standard: args.id.clone(),
        size,
        entropy: code_bits,
        node1: Some(prefix),
        hex,
        bits,
        color_map: Some(format!(
            "{}{}{}",
            (0..prefix_bits).map(|_| "4").collect::<String>(),
            (0..8).map(|_| "0").collect::<String>(),
            (0..code_bits).map(|_| "2").collect::<String>(),
        )),
        ..Default::default()
    })
}
