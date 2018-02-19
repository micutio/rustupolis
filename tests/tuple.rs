use std::cmp::Ordering;
use std::collections::Bound;

#[macro_use]
extern crate rustupolis;
use rustupolis::tuple::E;

#[test]
fn test_emptiness() {
    // Some corner cases around empty tuples
    assert!(tuple![].is_empty());
    assert!(tuple![].is_defined());
    assert_eq!(tuple![], tuple![]);
    assert_eq!(tuple![E::T(tuple![])], tuple![E::T(tuple![])]);
    assert_ne!(tuple![E::T(tuple![])], tuple![]);
    assert!(tuple![].matches(&tuple![]));
    assert!(E::T(tuple![]).matches(&E::Any));
    assert!(E::Any.matches(&E::T(tuple![])));
    assert!(!E::T(tuple![]).matches(&E::T(tuple![E::I(42)])));
}

#[test]
fn test_defined() {
    assert!(!E::Any.is_defined());
    assert!(!E::None.is_defined());
    assert!(E::str("foo").is_defined());
    assert!(E::T(tuple![E::I(0)]).is_defined());
    assert!(!E::T(tuple![E::Any]).is_defined());
    assert!(!E::T(tuple![E::None]).is_defined());
}

#[test]
fn test_cmp() {
    assert_eq!(tuple![].cmp(&tuple![E::I(0)]), Ordering::Less);
    assert_eq!(tuple![].cmp(&tuple![]), Ordering::Equal);
    assert_eq!(E::Any.cmp(&E::T(tuple![])), Ordering::Less);
    assert_eq!(E::None.cmp(&E::T(tuple![])), Ordering::Greater);
    assert_eq!(
        E::T(tuple![E::I(0), E::I(1)]).cmp(&E::T(tuple![E::I(0), E::I(2)])),
        Ordering::Less
    );
    assert_eq!(
        E::T(tuple![E::I(0), E::I(1), E::I(0)]).cmp(&E::T(tuple![E::I(0), E::I(1)])),
        Ordering::Greater
    );
}

#[test]
fn test_range() {
    assert_eq!(
        tuple![E::T(tuple![E::Any])].range(),
        (
            Bound::Excluded(tuple![E::T(tuple![E::Any])]),
            Bound::Excluded(tuple![E::T(tuple![E::None])])
        )
    );
    assert_eq!(
        tuple![E::str("foo")].range(),
        (
            Bound::Included(tuple![E::str("foo")]),
            Bound::Excluded(tuple![E::str("foo")]),
        )
    );
    assert_eq!(
        tuple![E::str("foo"), E::T(tuple![E::I(0), E::Any]), E::Any].range(),
        (
            Bound::Excluded(tuple![E::str("foo"), E::T(tuple![E::I(0), E::Any]), E::Any]),
            Bound::Excluded(tuple![
                E::str("foo"),
                E::T(tuple![E::I(0), E::None]),
                E::None
            ]),
        )
    );
}

#[test]
fn nested_compare_with_empty() {
    assert!(
        !tuple![E::T(tuple![E::str("command"), E::Any]), E::T(tuple![])].matches(&tuple![
            E::T(tuple![E::str("command"), E::str("wobble")]),
            E::T(tuple![E::I(10)]),
        ])
    );
    assert!(!tuple![
        E::T(tuple![E::str("command"), E::str("wobble")]),
        E::T(tuple![E::I(10)]),
    ].matches(&tuple![
        E::T(tuple![E::str("command"), E::Any]),
        E::T(tuple![])
    ]));
    assert!(
        tuple![
            E::T(tuple![E::str("command"), E::str("wobble")]),
            E::T(tuple![]),
        ].matches(&tuple![
            E::T(tuple![E::str("command"), E::Any]),
            E::T(tuple![])
        ])
    );
    assert!(
        tuple![E::T(tuple![E::str("command"), E::str("wobble")]), E::Any,].matches(&tuple![
            E::T(tuple![E::str("command"), E::Any]),
            E::T(tuple![])
        ])
    );
}
