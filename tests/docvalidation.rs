use quizface::utils::test;
use serde_json::Value;
use std::collections::HashSet;
use std::process::Command;
#[allow(dead_code)]
struct GetInfoResponseFixture {
    repr_bytes: Vec<u8>,
    repr_string: String,
    repr_json: Value,
    repr_keyset: HashSet<String>,
}
impl GetInfoResponseFixture {
    fn new() -> GetInfoResponseFixture {
        let repr_bytes = Command::new("zcash-cli")
            .arg("getinfo")
            .output()
            .unwrap()
            .stdout;
        let repr_string = String::from_utf8(repr_bytes.clone()).unwrap();
        let repr_json = serde_json::de::from_str(&repr_string).unwrap();
        let repr_keyset;
        if let Value::Object(objmap) = &repr_json {
            repr_keyset = objmap.keys().cloned().collect();
        } else {
            panic!()
        }
        GetInfoResponseFixture {
            repr_bytes,
            repr_string,
            repr_json,
            repr_keyset,
        }
    }
}
#[test]
#[ignore = "not yet implemented"]
fn validate_response_as_subset() {
    let response_fixture = GetInfoResponseFixture::new();
    use serde_json::Value;
    if let Value::Object(annotations) = test::valid_getinfo_annotation() {
        let testdata_keys: HashSet<String> =
            annotations.keys().cloned().collect();
        dbg!(&response_fixture.repr_keyset.difference(&testdata_keys));
        assert!(response_fixture
            .repr_keyset
            .difference(&testdata_keys)
            .cloned()
            .collect::<String>()
            .is_empty());
    }
}
