#![cfg(feature = "serde")]

use serde::{de::DeserializeOwned, Serialize};
use dana::{prelude::*, symbols::basic::*};


#[allow(dead_code)]
fn show_encode(value: impl Serialize + core::fmt::Display) {
    let json = serde_json::to_string_pretty(&value)
        .expect("failed to encode as JSON");

    eprintln!("{value} -> {json}");
}


/// Ensure that a [`Quantity`] converts losslessly to and from the given JSON.
fn test_json<U: Unit + DeserializeOwned + Serialize>(qty: Quantity<U>, json: &str) {
    let json_from_qty = serde_json::to_string(&qty)
        .expect("failed to encode as JSON");

    let qty_from_json: Quantity<U> = serde_json::from_str(json)
        .expect("failed to decode JSON");

    // show_encode(qty);

    assert_eq!(json, json_from_qty, "encoded incorrectly");
    assert_eq!(qty, qty_from_json, "decoded incorrectly");
}


/// Ensure that a [`Quantity`] does NOT match the given JSON.
fn test_fail<U: Unit + DeserializeOwned + Serialize>(qty: Quantity<U>, json: &str) {
    let json_from_qty = serde_json::to_string(&qty)
        .expect("failed to encode as JSON");

    let _err = serde_json::from_str::<Quantity<U>>(json)
        .expect_err("successfully decoded incorrect JSON");

    assert_ne!(json, json_from_qty, "encoded incorrectly");
}


#[test]
fn serde() {
    //  Test standard forms.
    test_json(qty![2.0 m],   r#"{"value":2.0,"unit":"Meter"}"#);
    test_json(qty![2.0/m],   r#"{"value":2.0,"unit":{"inv":"Meter"}}"#);
    test_json(qty![2.0 m^2], r#"{"value":2.0,"unit":{"base":"Meter","exp":2}}"#);
    test_json(qty![2.0 m^3], r#"{"value":2.0,"unit":{"base":"Meter","exp":3}}"#);
    test_json(qty![2.0 m/s], r#"{"value":2.0,"unit":{"div":["Meter","Second"]}}"#);
    test_json(qty![2.0 m*s], r#"{"value":2.0,"unit":{"mul":["Meter","Second"]}}"#);

    //  Test powers.
    test_json(qty![2.0 m].pow::<1>(), r#"{"value":2.0,"unit":{"base":"Meter","exp":1}}"#);
    test_json(qty![2.0 m].pow::<2>(), r#"{"value":4.0,"unit":{"base":"Meter","exp":2}}"#);
    test_json(qty![2.0 m].pow::<3>(), r#"{"value":8.0,"unit":{"base":"Meter","exp":3}}"#);

    //  Test anonymous.
    test_json(qty![2.0 m/s as ?], r#"{"value":2.0,"unit":1.0}"#);
    test_json(qty![2.0 mm/s as ?], r#"{"value":2.0,"unit":0.001}"#);
    test_json(qty![2.0 km/s as ?], r#"{"value":2.0,"unit":1000.0}"#);

    //  Ensure that compounds cannot be confused.
    test_fail(qty![2.0 m^2], r#"{"value":2.0,"unit":{"base":"Meter","exp":3}}"#);
    test_fail(qty![2.0 m^3], r#"{"value":2.0,"unit":{"base":"Meter","exp":2}}"#);
    test_fail(qty![2.0 m/s], r#"{"value":2.0,"unit":{"mul":["Meter","Second"]}}"#);
    test_fail(qty![2.0 m*s], r#"{"value":2.0,"unit":{"div":["Meter","Second"]}}"#);
}
