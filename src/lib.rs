pub mod utils;

pub fn annotate_identifier(ident_with_metadata: String) -> (String, String) {
    // find key. begin by selecting first str before
    // whitespace and eliminating leading whitespace.
    let mut temp_iter = ident_with_metadata.split_ascii_whitespace();
    let unparsed_key_str = match temp_iter.next() {
        Some(x) => x,
        None => panic!("error during command parsing"),
    };
    //dbg!(&unparsed_key_str);

    let unparsed_key_str_vec: Vec<&str> = unparsed_key_str.split('"').collect();

    // unparsed_key_str_vec should still contain leading "" element
    // and trailing ":" element, and be exactly 3 elements in length
    if &unparsed_key_str_vec.len() == &3 {
        // do nothing
    } else {
        panic!("unparsed_key_str_vec != 3")
    }

    // one more intermediate Vec
    let mut isolated_key_str_vec: Vec<&str> = Vec::new();
    for element in unparsed_key_str_vec {
        if element != "" && element != ":" {
            isolated_key_str_vec.push(element);
        }
    }

    // check that isolated_key_str_vec has exactly 1 element
    if &isolated_key_str_vec.len() != &1 {
        panic!("more than one element in isolated_key_str_vec !")
    }

    let key_str = isolated_key_str_vec[0];
    //dbg!(&key_str);

    // find 'keywords' in ident_with_metadata to eventually produce values
    // that match rust types in resulting HashMap.
    // to prevent extra detecting extra occurances, find only
    // these 'keywords' within first set of paranthesis.

    // split with an closure to support multiple 'splitters'
    let unparsed_value_str_vec: Vec<&str> = ident_with_metadata
        .split(|c| c == '(' || c == ')')
        .collect();

    // because unparsed_value_str_vec will have an element before
    // the first '(', and there may be more sets of parenthesis,
    // only the second element with is examined with [1].
    // if there are nested parenthesis this scheme will fail.
    // TODO possibly check for nested parenthesis?

    // determine if optional.
    let mut optional: bool = false;
    if unparsed_value_str_vec[1].contains("optional") {
        optional = true;
    }

    // create value collecting Vec, to then check vec
    // has only one valid value
    let mut value_collector_vec: Vec<&str> = Vec::new();

    // transforming for HashMap, provide values to vec
    if unparsed_value_str_vec[1].contains("numeric") {
        value_collector_vec.push("Decimal");
    }
    if unparsed_value_str_vec[1].contains("string") {
        value_collector_vec.push("String");
    }
    if unparsed_value_str_vec[1].contains("boolean") {
        value_collector_vec.push("bool");
    }

    if &value_collector_vec.len() != &1 {
        panic!("only 1 element allowed in value_collector_vec!")
    }

    let temp_value_string: String;
    let value_str: &str;
    // form value for Hashmap, cosidering the boolean optional
    if optional {
        temp_value_string = format!("Option<{}>", value_collector_vec[0]);
        value_str = &temp_value_string;
    } else {
        value_str = value_collector_vec[0];
    }
    dbg!(value_str);
    (key_str.to_string(), value_str.to_string())
}
