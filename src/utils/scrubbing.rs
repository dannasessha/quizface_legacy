#[macro_export]
macro_rules! scrub {
    ($cmd_name:expr, $result_data:expr) => {{
        if $cmd_name == "getblockchaininfo".to_string() {
            scrub_getblockchaininfo($result_data)
        } else if $cmd_name == "getchaintips".to_string() {
            scrub_getchaintips($result_data)
        } else if $cmd_name == "getaddressmempool".to_string() {
            scrub_getaddressmempool($result_data)
        } else if $cmd_name == "getblockdeltas".to_string() {
            scrub_getblockdeltas($result_data)
        } else if $cmd_name == "getspentinfo".to_string() {
            scrub_getspentinfo($result_data)
        } else if $cmd_name == "submitblock".to_string() {
            scrub_submitblock($result_data)
        } else if $cmd_name == "listaccounts".to_string() {
            scrub_listaccounts($result_data)
        } else if $cmd_name == "listreceivedbyaccount".to_string() {
            scrub_listreceivedbyaccount($result_data)
        } else if $cmd_name == "listreceivedbyaddress".to_string() {
            scrub_listreceivedbyaddress($result_data)
        } else if $cmd_name == "listtransactions".to_string() {
            scrub_listtransactions($result_data)
        } else if $cmd_name == "z_listreceivedbyaddress".to_string() {
            scrub_z_listreceivedbyaddress($result_data)
        } else if $cmd_name == "z_getoperationstatus".to_string() {
            scrub_z_getoperationstatus($result_data)
        } else if $cmd_name == "z_getoperationresult".to_string() {
            scrub_z_getoperationresult($result_data)
        } else if $cmd_name == "getaddressdeltas".to_string() {
            scrub_getaddressdeltas($result_data)
        } else {
            $result_data
        }
    }};
}

pub fn scrub_getblockchaininfo(raw: String) -> String {
    raw.replace("[0..1]", "").replace(
        "{ ... }      (object) progress toward rejecting pre-softfork blocks",
        "{
\"status\": (boolean)
\"found\": (numeric)
\"required\": (numeric)
\"window\": (numeric)
}").replace("(same fields as \"enforce\")", "").replace(", ...", "")
}

pub fn scrub_getchaintips(raw: String) -> String {
    raw.replace(
            r#"Possible values for status:
1.  "invalid"               This branch contains at least one invalid block
2.  "headers-only"          Not all blocks for this branch are available, but the headers are valid
3.  "valid-headers"         All blocks are available for this branch, but they were never fully validated
4.  "valid-fork"            This branch is not part of the active chain, but is fully validated
5.  "active"                This is the tip of the active main chain, which is certainly valid"#, "")
.replace(r#""height": xxxx,
"#, r#""height": xxxx,         (numeric) height of the chain tip
"#).replace(r#""hash": "xxxx",
"#, r#""hash": "xxxx",         (string) block hash of the tip
"#)
}

pub fn scrub_getaddressmempool(raw: String) -> String {
    raw.replace(r#"number"#, r#"numeric"#)
}

pub fn scrub_getblockdeltas(raw: String) -> String {
    raw.replace(r#"hex string"#, r#"hexadecimal"#)
        .replace(r#"hexstring"#, r#"hexadecimal"#)
}
pub fn scrub_getspentinfo(raw: String) -> String {
    raw.replace(r#"number"#, r#"numeric"#).replace(
        r#"  ,...
"#,
        r#""#,
    )
}

pub fn scrub_submitblock(raw: String) -> String {
    raw.replace(r#"duplicate" - node already has valid copy of block
"duplicate-invalid" - node already has block, but it is invalid
"duplicate-inconclusive" - node already has block but has not validated it
"inconclusive" - node has not validated the block, it may not be on the node's current best chain
"rejected" - block was rejected as invalid
For more information on submitblock parameters and results, see: https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki#block-submission"#,
r#"duplicate": (boolean) node already has valid copy of block
"duplicate-invalid": (boolean) node already has block, but it is invalid
"duplicate-inconclusive": (boolean) node already has block but has not validated it
"inconclusive": (boolean)node has not validated the block, it may not be on the node's current best chain
"rejected": (boolean) block was rejected as invalid"#)
}

pub fn scrub_listaccounts(raw: String) -> String {
    raw.replace(r#"                      (json object where keys are account names, and values are numeric balances"#, "")
        .replace(r#"  ...
"#, "")
}

pub fn scrub_listreceivedbyaccount(raw: String) -> String {
    raw.replace(r#"bool"#, "boolean").replace(
        r#"  ,...
"#,
        "",
    )
}
pub fn scrub_listreceivedbyaddress(raw: String) -> String {
    raw.replace(r#"bool"#, "boolean").replace(
        r#"  ,...
"#,
        "",
    )
}

pub fn scrub_listtransactions(raw: String) -> String {
    raw.lines()
        .filter(|l| !l.starts_with("                                         "))
        .fold(String::new(), |mut accumulator, new| {
            accumulator.push_str(new);
            accumulator.push_str("\n");
            accumulator
        })
}

pub fn scrub_z_listreceivedbyaddress(raw: String) -> String {
    raw.replace(r#" (sprout) : n,"#, r#": n, <sprout> "#)
        .replace(r#" (sapling) : n,"#, r#": n, <sapling> "#)
}

pub fn scrub_z_getoperationstatus(raw: String) -> String {
    raw.replace(
        r#"(array) A list of JSON objects"#,
        r#"(INSUFFICIENT) A list of JSON objects"#,
    )
}

pub fn scrub_z_getoperationresult(raw: String) -> String {
    raw.replace(
        r#"(array) A list of JSON objects"#,
        r#"(INSUFFICIENT) A list of JSON objects"#,
    )
}
pub fn scrub_getaddressdeltas(raw: String) -> String {
    raw.split(r#"(or, if chainInfo is true):"#)
        .collect::<Vec<&str>>()[1]
        .trim()
        .to_string()
        .replace(r#""deltas":"#, r#""alsoStandalone<deltas>":"#)
        .replace(
            r#"        "satoshis"    (number) The difference of zatoshis
        "txid"        (string) The related txid
        "index"       (number) The related input or output index
        "height"      (number) The block height
        "address"     (string)  The address base58check encoded"#,
            r#"        "satoshis":   (numeric) The difference of zatoshis
        "txid":       (string) The related txid
        "index":      (numeric) The related input or output index
        "height":     (numeric) The block height
        "address":    (string)  The address base58check encoded"#,
        )
        .replace(", ...", "")
        .replace(
            r#"  "start":
    {
      "hash"          (string)  The start block hash
      "height"        (numeric) The height of the start block
    }"#,
            r#"  "start":
    {
      "hash":         (string)  The start block hash
      "height":       (numeric) The height of the start block
    }"#,
        )
        .replace(
            r#"  "end":
    {
      "hash"          (string)  The end block hash
      "height"        (numeric) The height of the end block
    }"#,
            r#"  "end":
    {
      "hash":         (string)  The end block hash
      "height":       (numeric) The height of the end block
    }"#,
        )
}
