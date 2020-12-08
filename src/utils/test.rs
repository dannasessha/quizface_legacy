use std::collections::HashMap;
pub const ENFORCE_EXTRACTED: &str = r#""enforce": {           (object) progress toward enforcing the softfork rules for new-version blocks
"status": xx,       (boolean) true if threshold reached
"found": xx,        (numeric) number of blocks with the new version found
"required": xx,     (numeric) number of blocks required to trigger
"window": xx,       (numeric) maximum size of examined window of recent blocks
}"#;
pub const HELP_GETBLOCKCHAININFO: &str = r#"{
  "chain": "xxxx",        (string) current network name as defined in BIP70 (main, test, regtest)
  "blocks": xxxxxx,         (numeric) the current number of blocks processed in the server
  "initial_block_download_complete": xx, (boolean) true if the initial download of the blockchain is complete
  "headers": xxxxxx,        (numeric) the current number of headers we have validated
  "bestblockhash": "...", (string) the hash of the currently best block
  "difficulty": xxxxxx,     (numeric) the current difficulty
  "verificationprogress": xxxx, (numeric) estimate of verification progress [0..1]
  "estimatedheight": xxxx,  (numeric) if syncing, the estimated height of the chain, else the current best height
  "chainwork": "xxxx"     (string) total amount of work in active chain, in hexadecimal
  "size_on_disk": xxxxxx,       (numeric) the estimated size of the block and undo files on disk
  "commitments": xxxxxx,    (numeric) the current number of note commitments in the commitment tree
  "softforks": [            (array) status of softforks in progress
     {
        "id": "xxxx",        (string) name of softfork
        "version": xx,         (numeric) block version
        "enforce": {           (object) progress toward enforcing the softfork rules for new-version blocks
           "status": xx,       (boolean) true if threshold reached
           "found": xx,        (numeric) number of blocks with the new version found
           "required": xx,     (numeric) number of blocks required to trigger
           "window": xx,       (numeric) maximum size of examined window of recent blocks
        },
        "reject": { ... }      (object) progress toward rejecting pre-softfork blocks (same fields as "enforce")
     }, ...
  ],
  "upgrades": {                (object) status of network upgrades
     "xxxx" : {                (string) branch ID of the upgrade
        "name": "xxxx",        (string) name of upgrade
        "activationheight": xxxxxx,  (numeric) block height of activation
        "status": "xxxx",      (string) status of upgrade
        "info": "xxxx",        (string) additional information about upgrade
     }, ...
  },
  "consensus": {               (object) branch IDs of the current and upcoming consensus rules
     "chaintip": "xxxxxxxx",   (string) branch ID used to validate the current chain tip
     "nextblock": "xxxxxxxx"   (string) branch ID that the next block will be validated under
  }
}
"#;
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

pub const EXTRABRACKETS1_HELP_GETINFO: &str = r#"
getinfo with extra brackets in disorder before Result:
}

{ } { { }
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

pub const EXTRABRACKETS2_HELP_GETINFO: &str = r#"
getinfo with extra brackets in disorder after Examples:
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
}{ {} { { }
{ } { { }{
    {{{}{{}{{
"#;

pub const EXTRABRACKETS3_HELP_GETINFO: &str = r#"
getinfo with brackets in the middle of output lines, 
including badly formed brackets
Result:
{
  "version": {xxxxx,}           (numeric) the server version
  "protocolversion": {xxxxx,}   (numeric) the protocol version
  "walletversion": {xxxxx,}     (numeric) the wallet version
  "balance": {xxxxxxx,}         (numeric) the total Zcash balance of the wallet
  "blocks": {xxxxxx,}           (numeric) the current number of blocks processed in the server
  "timeoffset": {xxxxx,}        (numeric) the time offset (deprecated; always 0)
  "connections": {{{xxxxx,}}}       (numeric) the number of connections
  "proxy": }{}{{{{"host:port",  }{{}}}{{   (string, optional) the proxy used by the server
  "difficulty": }}xxxxxx,}}       (numeric) the current difficulty
  "testnet": {{true|false,{{      (boolean) if the server is using testnet or not
  "keypoololdest": }xxxxxx,{    (numeric) the timestamp (seconds since GMT epoch) of the oldest pre-generated key in the key pool
  "keypoolsize": xxxx,{}{{        (numeric) how many new keys are pre-generated
  "unlocked_until": }}{ttt,      (numeric) the timestamp in seconds since epoch (midnight Jan 1 1970 GMT) that the wallet is unlocked for transfers, or 0 if the wallet is locked
  "paytxfee": x.xxxx,  }{}       (numeric) the transaction fee set in ZEC/kB
  "relayfee": x.xxxx,         (numeric){} }minimum relay fee for non-free transactions in ZEC/kB{
  "errors": "..."           (string) {any error messages}
}
"#;

pub const MORE_BRACKET_PAIRS_HELP_GETINFO: &str = r#"
getinfo with two sets of curly brackets
Result:
{
  "version": xxxxx,           (numeric) the server version
  "protocolversion": xxxxx,   (numeric) the protocol version
  "walletversion": xxxxx,     (numeric) the wallet version
  "balance": xxxxxxx,         (numeric) the total Zcash balance of the wallet
  "blocks": xxxxxx,           (numeric) the current number of blocks processed in the server
  "timeoffset": xxxxx,        (numeric) the time offset (deprecated; always 0)
}
{
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

pub const EXTRA_START_BRACKET_HELP_GETINFO: &str = r#"
getinfo with two sets of curly brackets
Result:
{
  "version": xxxxx,           (numeric) the server version
  "protocolversion": xxxxx,   (numeric) the protocol version
  "walletversion": xxxxx,     (numeric) the wallet version
  "balance": xxxxxxx,         (numeric) the total Zcash balance of the wallet
  "blocks": xxxxxx,           (numeric) the current number of blocks processed in the server
  "timeoffset": xxxxx,        (numeric) the time offset (deprecated; always 0)
{
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

pub const EXTRA_END_BRACKET_HELP_GETINFO: &str = r#"
getinfo with two sets of curly brackets
Result:
{
  "version": xxxxx,           (numeric) the server version
  "protocolversion": xxxxx,   (numeric) the protocol version
  "walletversion": xxxxx,     (numeric) the wallet version
  "balance": xxxxxxx,         (numeric) the total Zcash balance of the wallet
  "blocks": xxxxxx,           (numeric) the current number of blocks processed in the server
  "timeoffset": xxxxx,        (numeric) the time offset (deprecated; always 0)
}
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

pub const NO_RESULT_HELP_GETINFO: &str = r#"
getinfo with no Result:

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
pub const NO_END_BRACKET_HELP_GETINFO: &str = r#"
getinfo with no closing bracket
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

"#;

pub const NO_START_BRACKET_HELP_GETINFO: &str = r#"
getinfo with no beginning bracket
Result:

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

pub fn valid_getinfo_annotation() -> serde_json::Value {
    serde_json::json!([
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
    .collect::<HashMap<String, String>>())
}
