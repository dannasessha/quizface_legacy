use std::collections::HashMap;
pub fn valid_getinfo_annotation() -> HashMap<&'static str, &'static str> {
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
