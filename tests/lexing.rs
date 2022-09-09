#[macro_use]
extern crate rustupolis as rt;

use rt::lexing::Lexer;
use rt::tuple::E;

#[test]
fn test_lexer() {
    let inputs = vec![
        "",
        "(1)",
        "(2.0, 3.15, \"steeze\")",
        "(1, 2.0, \"3.15\", _, 666), (\"foo\", \"bar\")",
        "(1, 2.0, \"3.15\", _, 666), (\"foo\", \"bar\"), ((1, -2, -3), (-4.1, 5.1, \"6.1\"))",
    ];

    let expected = vec![
        vec![],
        vec![rt::tuple!(E::I(1))],
        vec![rt::tuple!(
            E::D(2.0),
            E::D(3.15),
            E::S("steeze".to_string())
        )],
        vec![
            rt::tuple!(
                E::I(1),
                E::D(2.0),
                E::S("3.15".to_string()),
                E::Any,
                E::I(666)
            ),
            rt::tuple!(E::S("foo".to_string()), E::S("bar".to_string())),
        ],
        vec![
            rt::tuple!(
                E::I(1),
                E::D(2.0),
                E::S("3.15".to_string()),
                E::Any,
                E::I(666)
            ),
            rt::tuple!(E::S("foo".to_string()), E::S("bar".to_string())),
            rt::tuple!(
                E::T(rt::tuple!(E::I(1), E::I(-2), E::I(-3))),
                E::T(rt::tuple!(E::D(-4.1), E::D(5.1), E::S("6.1".to_string()))),
            ),
        ],
    ];

    for (i, e) in inputs.iter().zip(expected.iter()) {
        check_output(i, e);
    }
}

fn check_output(input: &str, expected: &[rt::tuple::Tuple]) {
    let output_tuples: Vec<rt::Tuple> = Lexer::new(input).collect();

    if output_tuples.len() != expected.len() {
        panic!(
            "output and expected are of diffent length!\noutput: {:#?}\nexpected: {:#?}",
            output_tuples, expected
        );
    } else {
        let mut is_different = false;
        for (o, e) in output_tuples.iter().zip(expected.iter()) {
            if o != e {
                is_different = true;
                break;
            }
        }

        if is_different {
            panic!(
                "output and expected are not equal!\noutput: {:#?}\nexpected: {:#?}",
                output_tuples, expected
            );
        }
    }
}
