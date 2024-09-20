use super::JsonRef;

#[test]
fn json_no_refs() {
    let no_ref_example = serde_json::json!({"properties": {"prop1": {"title": "proptitle"}}});

    let mut jsonref = JsonRef::new();

    let mut input = no_ref_example.clone();

    jsonref.deref_value(&mut input).unwrap();

    assert_eq!(input, no_ref_example)
}

#[test]
fn json_with_recursion() {
    let mut simple_refs_example = serde_json::json!(
        {"properties": {"prop1": {"$ref": "#"}}}
    );

    let simple_refs_expected = serde_json::json!(
        {"properties": {"prop1": {"properties": {"prop1": {}}}}
        }
    );

    let mut jsonref = JsonRef::new();
    jsonref.deref_value(&mut simple_refs_example).unwrap();
    jsonref.set_reference_key("__reference__");

    println!(
        "{}",
        serde_json::to_string_pretty(&simple_refs_example).unwrap()
    );

    assert_eq!(simple_refs_example, simple_refs_expected)
}

#[test]
fn json_with_multiple_recursion() {
    let mut simple_refs_example = serde_json::json!(
        {"properties": {
            "prop0": {"$ref": "#"},
            "prop1": {"$ref": "#/properties/prop0"},
            "prop2": 5
        }}
    );

    let simple_refs_expected = serde_json::json!({
      "properties": {
        "prop0": {
          "properties": {
            "prop0": {},
            "prop1": {},
            "prop2": 5
          }
        },
        "prop1": {
          "properties": {
            "prop0": {},
            "prop1": {},
            "prop2": 5
          }
        },
        "prop2": 5
      }
    });

    let mut jsonref = JsonRef::new();
    jsonref.deref_value(&mut simple_refs_example).unwrap();
    jsonref.set_reference_key("__reference__");

    println!(
        "{}",
        serde_json::to_string_pretty(&simple_refs_example).unwrap()
    );

    assert_eq!(simple_refs_example, simple_refs_expected)
}

#[test]
fn simple_from_url() {
    let mut simple_refs_example = serde_json::json!(
        {"properties": {"prop1": {"title": "name"},
                        "prop2": {"$ref": "https://gist.githubusercontent.com/kindly/35a631d33792413ed8e34548abaa9d61/raw/b43dc7a76cc2a04fde2a2087f0eb389099b952fb/test.json", "title": "old_title"}}
        }
    );

    let simple_refs_expected = serde_json::json!(
        {"properties": {"prop1": {"title": "name"},
                        "prop2": {"title": "title from url", "__reference__": {"title": "old_title"}}}
        }
    );

    let mut jsonref = JsonRef::new();
    jsonref.set_reference_key("__reference__");
    jsonref.deref_value(&mut simple_refs_example).unwrap();

    assert_eq!(simple_refs_example, simple_refs_expected)
}

#[test]
fn nested_with_ref_from_url() {
    let mut simple_refs_example = serde_json::json!(
        {"properties": {"prop1": {"title": "name"},
                        "prop2": {"$ref": "https://gist.githubusercontent.com/kindly/35a631d33792413ed8e34548abaa9d61/raw/0a691c035251f742e8710f71ba92ead307823385/test_nested.json"}}
        }
    );

    let simple_refs_expected = serde_json::json!(
        {"properties": {"prop1": {"title": "name"},
                        "prop2": {"__reference__": {},
                                  "title": "title from url",
                                  "properties": {"prop1": {"title": "sub property title in url"},
                                                 "prop2": {"__reference__": {}, "title": "sub property title in url"}}
                        }}
        }
    );

    let mut jsonref = JsonRef::new();
    jsonref.set_reference_key("__reference__");
    jsonref.deref_value(&mut simple_refs_example).unwrap();

    assert_eq!(simple_refs_example, simple_refs_expected)
}

#[test]
fn nested_ref_from_local_file() {
    let mut jsonref = JsonRef::new();
    jsonref.set_reference_key("__reference__");
    let file_example = jsonref
        .deref_file(
            &std::path::PathBuf::from_iter(["fixtures", "nested_relative", "base.json"].iter())
                .into_os_string(),
        )
        .unwrap();

    let file = std::fs::File::open(
        &std::path::PathBuf::from_iter(["fixtures", "nested_relative", "expected.json"].iter())
            .into_os_string(),
    )
    .unwrap();
    let file_expected: serde_json::Value = serde_json::from_reader(file).unwrap();

    println!("{}", serde_json::to_string_pretty(&file_example).unwrap());

    assert_eq!(file_example, file_expected)
}
