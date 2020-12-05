use std::collections::HashMap;
pub const HELP_GETINFO: &str = r#"
getinfo
Returns an object containing various state info.

Result:
{
  "version": xxxxx,           (numeric) the server version
  "protocolversion": xxxxx,   (numeric) the protocol version
  "walletversion": xxxxx,     (numeric) the wallet version
  "balance": xxxxxxx,         (numeric) the total Zcash balance of the wallet
  "blocks": xxxxxx,           (numeric) the current number of blocks processed in the server
  "timeoffset": xxxxx,        (numeric) the time offset (deprecated; always 0)
  "connections": xxxxx,       (numeric) the number of connections
  "proxy": "host:port",     (string, optional) the proxy used by the server
  "difficulty": xxxxxx,       (numeric) the current difficulty
  "testnet": true|false,      (boolean) if the server is using testnet or not
  "keypoololdest": xxxxxx,    (numeric) the timestamp (seconds since GMT epoch) of the oldest pre-generated key in the key pool
  "keypoolsize": xxxx,        (numeric) how many new keys are pre-generated
  "unlocked_until": ttt,      (numeric) the timestamp in seconds since epoch (midnight Jan 1 1970 GMT) that the wallet is unlocked for transfers, or 0 if the wallet is locked
  "paytxfee": x.xxxx,         (numeric) the transaction fee set in ZEC/kB
  "relayfee": x.xxxx,         (numeric) minimum relay fee for non-free transactions in ZEC/kB
  "errors": "..."           (string) any error messages
}

Examples:
> zcash-cli getinfo
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id":"curltest", "method": "getinfo", "params": [] }' -H 'content-type: text/plain;' http://127.0.0.1:8232/

"#;

pub const LBRACKETY_HELP_GETINFO: &str = r#"
getinfo with an extra 
{
Returns an object containing various state info.

Result:
{
  "version": xxxxx,           (numeric) the server version
  "protocolversion": xxxxx,   (numeric) the protocol version
  "walletversion": xxxxx,     (numeric) the wallet version
  "balance": xxxxxxx,         (numeric) the total Zcash balance of the wallet
  "blocks": xxxxxx,           (numeric) the current number of blocks processed in the server
  "timeoffset": xxxxx,        (numeric) the time offset (deprecated; always 0)
  "connections": xxxxx,       (numeric) the number of connections
  "proxy": "host:port",     (string, optional) the proxy used by the server
  "difficulty": xxxxxx,       (numeric) the current difficulty
  "testnet": true|false,      (boolean) if the server is using testnet or not
  "keypoololdest": xxxxxx,    (numeric) the timestamp (seconds since GMT epoch) of the oldest pre-generated key in the key pool
  "keypoolsize": xxxx,        (numeric) how many new keys are pre-generated
  "unlocked_until": ttt,      (numeric) the timestamp in seconds since epoch (midnight Jan 1 1970 GMT) that the wallet is unlocked for transfers, or 0 if the wallet is locked
  "paytxfee": x.xxxx,         (numeric) the transaction fee set in ZEC/kB
  "relayfee": x.xxxx,         (numeric) minimum relay fee for non-free transactions in ZEC/kB
  "errors": "..."           (string) any error messages
}
"#;

pub const RBRACKETY_HELP_GETINFO: &str = r#"
getinfo with an extra 
}
Returns an object containing various state info.

Result:
{
  "version": xxxxx,           (numeric) the server version
  "protocolversion": xxxxx,   (numeric) the protocol version
  "walletversion": xxxxx,     (numeric) the wallet version
  "balance": xxxxxxx,         (numeric) the total Zcash balance of the wallet
  "blocks": xxxxxx,           (numeric) the current number of blocks processed in the server
  "timeoffset": xxxxx,        (numeric) the time offset (deprecated; always 0)
  "connections": xxxxx,       (numeric) the number of connections
  "proxy": "host:port",     (string, optional) the proxy used by the server
  "difficulty": xxxxxx,       (numeric) the current difficulty
  "testnet": true|false,      (boolean) if the server is using testnet or not
  "keypoololdest": xxxxxx,    (numeric) the timestamp (seconds since GMT epoch) of the oldest pre-generated key in the key pool
  "keypoolsize": xxxx,        (numeric) how many new keys are pre-generated
  "unlocked_until": ttt,      (numeric) the timestamp in seconds since epoch (midnight Jan 1 1970 GMT) that the wallet is unlocked for transfers, or 0 if the wallet is locked
  "paytxfee": x.xxxx,         (numeric) the transaction fee set in ZEC/kB
  "relayfee": x.xxxx,         (numeric) minimum relay fee for non-free transactions in ZEC/kB
  "errors": "..."           (string) any error messages
}
"#;

pub fn valid_getinfo_annotation() -> HashMap<String, String> {
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
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect()
}
