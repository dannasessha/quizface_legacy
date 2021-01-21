use std::collections::HashMap;
pub const ENFORCE_EXTRACTED: &str = r#"           (object) progress toward enforcing the softfork rules for new-version blocks
"status": xx,       (boolean) true if threshold reached
"found": xx,        (numeric) number of blocks with the new version found
"required": xx,     (numeric) number of blocks required to trigger
"window": xx,       (numeric) maximum size of examined window of recent blocks
}"#;
pub const INTERMEDIATE_REPR_ENFORCE: [(&str, &str); 4] = [
    ("status", "bool"),
    ("found", "Decimal"),
    ("required", "Decimal"),
    ("window", "Decimal"),
];

pub const HELP_GETBLOCKCHAININFO_COMPLETE: &str = r##"getblockchaininfo
Returns an object containing various state info regarding block chain processing.

Note that when the chain tip is at the last block before a network upgrade activation,
consensus.chaintip != consensus.nextblock.

Result:
{
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

Examples:
> zcash-cli getblockchaininfo
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id":"curltest", "method": "getblockchaininfo", "params": [] }' -H 'content-type: text/plain;' http://127.0.0.1:8232/
"##;

pub const UPGRADES_IN_OBJ_EXTRACTED: &str = r##"getblockchaininfo

Result:
{
    "upgrades": {                (object) status of network upgrades
     "xxxx" : {                (string) branch ID of the upgrade
        "name": "xxxx",        (string) name of upgrade
        "activationheight": xxxxxx,  (numeric) block height of activation
        "status": "xxxx",      (string) status of upgrade
        "info": "xxxx",        (string) additional information about upgrade
     }, ...
  }
}

Examples:
asd
"##;

pub const GETBLOCKCHAININFO_SOFTFORK_FRAGMENT: &str = r##"getblockchaininfo

Result:
{
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
}

Examples:
"##;

pub const GETBLOCKCHAININFO_ENFORCE_AND_REJECT_FRAGMENT: &str = r##"getblockchaininfo

Result:
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
 }

Examples:
"##;

pub const HELP_GETBLOCKCHAININFO_RESULT: &str = r#"{
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

pub const SIMPLIFIED_SOFTFORK: &str = r#"{
        "id": "xxxx",        (string) name of softfork
        "version": xx,         (numeric) block version
        "enforce": {           (object) progress toward enforcing the softfork rules for new-version blocks
           "status": xx,       (boolean) true if threshold reached
           "found": xx,        (numeric) number of blocks with the new version found
           "required": xx,     (numeric) number of blocks required to trigger
           "window": xx,       (numeric) maximum size of examined window of recent blocks
        },
     }"#;
pub const SOFTFORK_EXTRACT_JSON: &str = r##"{
    "enforce":
        "{\"found\":\"Decimal\",\"required\":\"Decimal\",\"status\":\"bool\",\"window\":\"Decimal\"},",
    "id":
        "String",
    "version":
        "Decimal"
}
"##;
pub const GETBLOCKCHAININFO_REJECT_FRAGMENT: &str = r##"getblockchaininfo
Returns an object containing various state info regarding block chain processing.
XXX

Result:
{
    "reject": { ... }      (object) progress toward rejecting pre-softfork blocks (same fields as "enforce")
}

Examples:
> zcash-cli getblockchaininfo XXX
"##;
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
Examples:
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
Examples:
"#;

pub const EXTRABRACKETS1_HELP_GETINFO: &str = r#"
getinfo with extra brackets in disorder before result section!
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
Examples:
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
Examples:
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

pub fn simple_unnested_json_generator() -> serde_json::Value {
    let simple_nested_json = serde_json::json!({
    "outer_id": "String",
    });
    simple_nested_json
}

pub const SIMPLE_UNNESTED: &str = r#"{ 
   "outer_id": "xxxx.xxx", (string) extra unimportant text
}"#;

pub const SIMPLE_UNNESTED_FULL: &str = r#"
a_command
Result:
{ 
   "outer_id": "xxxx.xxx", (string) extra unimportant text
}

Examples:
b
"#;

pub const SIMPLE_UNNESTED_RESULT: &str = r#"{"outer_id":"String"}"#;

pub fn simple_nested_json_generator() -> serde_json::Value {
    let simple_nested_json = serde_json::json!({
    "outer_id":
    {"inner_id": "String",}
    });
    simple_nested_json
}

pub const SIMPLE_NESTED: &str = r#"{ 
    "outer_id": {
        "inner_id": "xxxx",      (string) extra unimportant text
    }
}"#;

pub const SIMPLE_NESTED_FULL: &str = r#"
a_command
Result:
{ 
    "outer_id": {
        "inner_id": "xxxx",      (string) extra unimportant text
    }
}
Examples:
b
"#;

pub const SIMPLE_NESTED_RESULT: &str = r#"{"outer_id":{"inner_id":"String"}}"#;

pub const SIMPLE_UNNESTED_GETBLOCKCHAININFO: &str = r#"{ 
     "name": "xxxx",        (string) name of upgrade
}
"#;

pub const SIMPLE_UNNESTED_GETBLOCKCHAININFO_RESULT: &str =
    r#"{"name":"String"}"#;

pub const SPECIAL_NESTED_GETBLOCKCHAININFO: &str = r#"{ 
     "xxxx" : {                (string) branch ID of the upgrade
        "name": "xxxx",        (string) name of upgrade
   }
}
"#;

pub const SPECIAL_NESTED_GETBLOCKCHAININFO_RESULT: &str =
    r#"{"String":{"name":"String"}}"#;

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

pub fn getinfo_export() -> serde_json::Value {
    let getinfo_serde_json_value = serde_json::json!({
        "version": "Decimal",
        "protocolversion": "Decimal",
        "walletversion": "Decimal",
        "balance": "Decimal",
        "blocks": "Decimal",
        "timeoffset": "Decimal",
        "connections": "Decimal",
        "proxy": "Option<String>",
        "difficulty": "Decimal",
        "testnet": "bool",
        "keypoololdest": "Decimal",
        "keypoolsize": "Decimal",
        "unlocked_until": "Decimal",
        "paytxfee": "Decimal",
        "relayfee": "Decimal",
        "errors": "String",
    });
    getinfo_serde_json_value
}
pub fn getblockchaininfo_export() -> serde_json::Value {
    let getblockchaininfo_serde_json_value = serde_json::json!({
    "chain": "String",
    "blocks": "Decimal",
    "initial_block_download_complete": "bool",
    "headers": "Decimal",
    "bestblockhash": "String",
    "difficulty": "Decimal",
    "verificationprogress": "Decimal",
    "estimatedheight": "Decimal",
    "chainwork": "String",
    "size_on_disk": "Decimal",
    "commitments": "Decimal",
    "softforks": [
       {
          "id": "String",
          "version": "Decimal",
          "enforce": {
             "status": "bool",
             "found": "Decimal",
             "required": "Decimal",
             "window": "Decimal"
          },
          "reject": {
             "status": "bool",
             "found": "Decimal",
             "required": "Decimal",
             "window": "Decimal"
          },
       }
    ],
    "upgrades": {
       "String": {
          "name": "String",
          "activationheight": "Decimal",
          "status": "String",
          "info": "String"
       }
    },
    "consensus": {
       "chaintip": "String",
       "nextblock": "String"
    }
      });
    getblockchaininfo_serde_json_value
}
