use std::collections::HashMap;
pub fn fake_parse_getinfo() -> HashMap<&'static str, &'static str> {
    [
        ("version", "Decimal"),
        ("protocolversion", "Decimal"),
        ("walletversion", "Decimal"),
        ("balance", "Decimal"),
        ("blocks", "Decimal"),
        ("timeoffset", "Decimal"),
        ("connections", "Decimal"),
        ("proxy", "Option<String>"),
        ("difficulty", "Decimal"),
        ("testnet", "bool"),
        ("keypoololdest", "Decimal"),
        ("keypoolsize", "Decimal"),
        ("unlocked_until", "Decimal"),
        ("paytxfee", "Decimal"),
        ("relayfee", "Decimal"),
        ("errors", "String"),
    ]
    .iter()
    .cloned()
    .collect()
}
