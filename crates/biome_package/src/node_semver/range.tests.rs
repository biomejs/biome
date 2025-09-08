use super::*;

#[test]
fn test_exact_range() {
    let range = "1.2.3".parse::<Range>().unwrap();

    assert!(range.includes(&"1.2.3".parse().unwrap()));
    assert!(!range.includes(&"1.2.4".parse().unwrap()));
}

#[test]
fn test_tilde_range() {
    let range = "~1.2.3".parse::<Range>().unwrap();

    assert!(range.includes(&"1.2.3".parse().unwrap()));
    assert!(range.includes(&"1.2.4".parse().unwrap()));
    assert!(range.includes(&"1.2.10".parse().unwrap()));
    assert!(!range.includes(&"1.3.0".parse().unwrap()));
    assert!(!range.includes(&"2.0.0".parse().unwrap()));
}

#[test]
fn test_caret_range() {
    let range = "^1.2.3".parse::<Range>().unwrap();

    assert!(range.includes(&"1.2.3".parse().unwrap()));
    assert!(range.includes(&"1.2.4".parse().unwrap()));
    assert!(range.includes(&"1.3.0".parse().unwrap()));
    assert!(range.includes(&"1.9.9".parse().unwrap()));
    assert!(!range.includes(&"2.0.0".parse().unwrap()));
    assert!(!range.includes(&"1.2.2".parse().unwrap()));
}

#[test]
fn test_caret_range_zero_major() {
    let range = "^0.2.3".parse::<Range>().unwrap();

    assert!(range.includes(&"0.2.3".parse().unwrap()));
    assert!(range.includes(&"0.2.4".parse().unwrap()));
    assert!(!range.includes(&"0.3.0".parse().unwrap()));
    assert!(!range.includes(&"1.0.0".parse().unwrap()));
}

#[test]
fn test_caret_range_zero_minor() {
    let range = "^0.0.3".parse::<Range>().unwrap();

    assert!(range.includes(&"0.0.3".parse().unwrap()));
    assert!(!range.includes(&"0.0.4".parse().unwrap()));
    assert!(!range.includes(&"0.1.0".parse().unwrap()));
}

#[test]
fn test_comparison_operators() {
    let range = ">=1.2.3".parse::<Range>().unwrap();
    assert!(range.includes(&"1.2.3".parse().unwrap()));
    assert!(range.includes(&"1.2.4".parse().unwrap()));
    assert!(range.includes(&"2.0.0".parse().unwrap()));
    assert!(!range.includes(&"1.2.2".parse().unwrap()));

    let range = ">1.2.3".parse::<Range>().unwrap();
    assert!(!range.includes(&"1.2.3".parse().unwrap()));
    assert!(range.includes(&"1.2.4".parse().unwrap()));

    let range = "<=1.2.3".parse::<Range>().unwrap();
    assert!(range.includes(&"1.2.3".parse().unwrap()));
    assert!(range.includes(&"1.2.2".parse().unwrap()));
    assert!(!range.includes(&"1.2.4".parse().unwrap()));

    let range = "<1.2.3".parse::<Range>().unwrap();
    assert!(!range.includes(&"1.2.3".parse().unwrap()));
    assert!(range.includes(&"1.2.2".parse().unwrap()));
}

#[test]
fn test_hyphen_range() {
    let range = "1.2.3 - 2.3.4".parse::<Range>().unwrap();

    assert!(range.includes(&"1.2.3".parse().unwrap()));
    assert!(range.includes(&"1.5.0".parse().unwrap()));
    assert!(range.includes(&"2.3.4".parse().unwrap()));
    assert!(!range.includes(&"1.2.2".parse().unwrap()));
    assert!(!range.includes(&"2.3.5".parse().unwrap()));
}

#[test]
fn test_compound_range() {
    let range = ">=1.2.7 <1.3.0".parse::<Range>().unwrap();

    assert!(range.includes(&"1.2.7".parse().unwrap()));
    assert!(range.includes(&"1.2.8".parse().unwrap()));
    assert!(range.includes(&"1.2.99".parse().unwrap()));
    assert!(!range.includes(&"1.2.6".parse().unwrap()));
    assert!(!range.includes(&"1.3.0".parse().unwrap()));
}

#[test]
fn test_or_range() {
    let range = "1.2.7 || >=1.2.9 <2.0.0".parse::<Range>().unwrap();

    assert!(range.includes(&"1.2.7".parse().unwrap()));
    assert!(!range.includes(&"1.2.8".parse().unwrap()));
    assert!(range.includes(&"1.2.9".parse().unwrap()));
    assert!(range.includes(&"1.4.6".parse().unwrap()));
    assert!(!range.includes(&"2.0.0".parse().unwrap()));
}

#[test]
fn test_comprehensive_range_include() {
    // Test cases from node-semver range-include fixture (filtered for our implementation)
    let include_cases = vec![
        // Basic exact matches
        ("1.0.0", "1.0.0"),
        ("*", "1.2.3"),
        (">=1.0.0", "1.0.0"),
        (">=1.0.0", "1.0.1"),
        (">=1.0.0", "1.1.0"),
        (">1.0.0", "1.0.1"),
        (">1.0.0", "1.1.0"),
        ("<=2.0.0", "2.0.0"),
        ("<=2.0.0", "1.9999.9999"),
        ("<=2.0.0", "0.2.9"),
        ("<2.0.0", "1.9999.9999"),
        ("<2.0.0", "0.2.9"),
        (">= 1.0.0", "1.0.0"),
        (">=  1.0.0", "1.0.1"),
        (">=   1.0.0", "1.1.0"),
        ("> 1.0.0", "1.0.1"),
        (">  1.0.0", "1.1.0"),
        ("<=   2.0.0", "2.0.0"),
        ("<= 2.0.0", "1.9999.9999"),
        ("<=  2.0.0", "0.2.9"),
        ("<    2.0.0", "1.9999.9999"),
        ("<\t2.0.0", "0.2.9"),
        (">=0.1.97", "v0.1.97"),
        (">=0.1.97", "0.1.97"),
        ("0.1.20 || 1.2.4", "1.2.4"),
        (">=0.2.3 || <0.0.1", "0.0.0"),
        (">=0.2.3 || <0.0.1", "0.2.3"),
        (">=0.2.3 || <0.0.1", "0.2.4"),
        ("*", "1.2.3"),
        ("~2.4.0", "2.4.0"),
        ("~2.4.0", "2.4.5"),
        ("~1.0.0", "1.0.5"),
        ("~1.0.3", "1.0.12"),
        (">=1.0.0", "1.0.0"),
        ("<1.2.0", "1.1.1"),
        ("~1.2.1 >=1.2.3", "1.2.3"),
        ("~1.2.1 =1.2.3", "1.2.3"),
        ("~1.2.1 1.2.3", "1.2.3"),
        (">=1.2.1 1.2.3", "1.2.3"),
        ("1.2.3 >=1.2.1", "1.2.3"),
        (">=1.2.3 >=1.2.1", "1.2.3"),
        (">=1.2.1 >=1.2.3", "1.2.3"),
        (">=1.2.0", "1.2.8"),
        ("^1.2.3", "1.8.1"),
        ("^0.1.2", "0.1.2"),
        ("^1.2.0", "1.4.2"),
    ];

    for (range_str, version_str) in include_cases {
        let range: Range = range_str.parse().unwrap();
        let version: Version = version_str.parse().unwrap();

        assert!(
            range.includes(&version),
            "Range '{range_str}' should include version '{version_str}' but doesn't"
        );
    }
}

#[test]
fn test_comprehensive_range_exclude() {
    // Test cases from node-semver range-exclude fixture (filtered for our implementation)
    let exclude_cases = vec![
        ("1.0.0 - 2.0.0", "2.2.3"),
        ("^1.2.3", "2.0.0"),
        ("^1.2.3", "1.2.0"),
        ("^2.0.0", "1.1.1"),
        ("^2.0.0", "1.2.9"),
        ("^1.4.2", "1.4.1"),
        (">=1.2.0", "1.1.1"),
        ("2.0.0", "1.1.2"),
        ("2.3.0", "2.4.1"),
        ("~2.4.0", "2.5.0"),
        ("~2.4.0", "2.3.9"),
        ("~1.0.0", "0.2.3"),
        ("~1.0.0", "1.1.0"),
        ("<1.0.0", "1.0.0"),
        (">=1.2.0", "1.1.1"),
    ];

    for (range_str, version_str) in exclude_cases {
        let range: Range = range_str.parse().unwrap();
        let version: Version = version_str.parse().unwrap();

        assert!(
            !range.includes(&version),
            "Range '{range_str}' should exclude version '{version_str}' but doesn't"
        );
    }
}

#[test]
fn test_intersects() {
    let intersects_cases = vec![
        ("^1.2.3", "~1.2.5", true),
        (">=1.2.0", "<=1.3.0", true),
        (">=1.2.0", "<=2.0.0", true),
        ("*", "<=1.3.0", true),
        ("<=1.2.3", "=1.2.3", true),
        ("<=1.3.0", "*", true),
        ("1.2", "1.2", true),
        ("1.2", "1.2.1", true),
        ("1.2-alpha.4", "1.2.1-alpha.4", true),
        ("^1.2.3", "^2.0.0", false),
        ("<1.2.3", "=1.2.3", false),
        ("1.2", "1.3", false),
        ("1.2", "1.3.1", false),
        ("1.2.0", "1.2.1-alpha.1", false),
        ("1.2.1", "1.2.1-alpha.1", false),
        ("1.2.1", "1.3", false),
        ("^0.1.2", "^0.2.0", false),
        ("^2.2.3", "~1.2.5", false),
        ("^1.2.3", "~2.2.5", false),
        ("~1.2.3", "~1.3.2", false),
        (">=1.3.0", "<=1.2.0", false),
        (">=1.3.0", "<1.3.0", false),
    ];

    for (range1_str, range2_str, should_intersect) in intersects_cases {
        let range1: Range = range1_str.parse().unwrap();
        let range2: Range = range2_str.parse().unwrap();

        if should_intersect {
            assert!(
                range1.intersects(&range2),
                "Range '{range1_str}' should intersect with range '{range2_str}' but doesn't"
            );
            assert!(
                range2.intersects(&range1),
                "Range '{range2_str}' should intersect with range '{range1_str}' but doesn't, even though the other way around does"
            );
        } else {
            assert!(
                !range1.intersects(&range2),
                "Range '{range1_str}' should not intersect with range '{range2_str}' but does"
            );
            assert!(
                !range2.intersects(&range1),
                "Range '{range2_str}' should not intersect with range '{range1_str}' but does, even though the other way around doesn't"
            );
        }
    }
}

#[test]
fn test_range_edge_cases() {
    // Test wildcard ranges
    let wildcard_range: Range = "*".parse().unwrap();
    let version: Version = "1.2.3".parse().unwrap();

    assert!(wildcard_range.includes(&version));

    // Test basic range satisfaction
    let basic_cases = vec![
        (">=1.0.0", "1.5.0", true),
        (">=1.0.0", "0.9.0", false),
        ("<2.0.0", "1.2.5", true),
        ("<2.0.0", "2.1.0", false),
        ("~1.2.3", "1.2.7", true),
        ("~1.2.3", "1.3.0", false),
    ];

    for (range_str, version_str, should_satisfy) in basic_cases {
        let range: Range = range_str.parse().unwrap();
        let version: Version = version_str.parse().unwrap();

        assert_eq!(
            range.includes(&version),
            should_satisfy,
            "Range '{range_str}' satisfaction of '{version_str}' should be {should_satisfy}"
        );
    }
}

// https://github.com/felipesere/node-semver-rs/blob/main/src/range.rs#L180
macro_rules! range_parse_tests {
        ($($name:ident => $vals:expr),+ ,$(,)?) => {
            $(
                #[test]
                fn $name() {
                    let [input, expected] = $vals;
                    let parsed = Range::from_str(input).expect("unable to parse");
                    assert_eq!(parsed.to_string(), expected, "unexpected range from '{input}'");
                }
            )+
        }

    }

range_parse_tests![
    // [input, parsed and then `to_string`ed]
    exact => ["1.0.0", "1.0.0"],
    major_minor_patch_range => ["1.0.0 - 2.0.0", ">=1.0.0 <=2.0.0"],
    only_major_versions =>  ["1 - 2", ">=1.0.0 <3.0.0-0"],
    only_major_and_minor => ["1.0 - 2.0", ">=1.0.0 <2.1.0-0"],
    mixed_major_minor => ["1.2 - 3.4.5", ">=1.2.0 <=3.4.5"],
    mixed_major_minor_2 => ["1.2.3 - 3.4", ">=1.2.3 <3.5.0-0"],
    minor_minor_range => ["1.2 - 3.4", ">=1.2.0 <3.5.0-0"],
    single_sided_only_major => ["1", ">=1.0.0 <2.0.0-0"],
    single_sided_lower_equals_bound =>  [">=1.0.0", ">=1.0.0"],
    single_sided_lower_equals_bound_2 => [">=0.1.97", ">=0.1.97"],
    single_sided_lower_bound => [">1.0.0", ">1.0.0"],
    single_sided_upper_equals_bound => ["<=2.0.0", "<=2.0.0"],
    single_sided_upper_equals_bound_with_minor => ["<=2.0", "<=2.0.0-0"],
    single_sided_upper_bound => ["<2.0.0", "<2.0.0"],
    major_and_minor => ["2.3", ">=2.3.0 <2.4.0-0"],
    major_dot_x => ["2.x", ">=2.0.0 <3.0.0-0"],
    x_and_asterisk_version => ["2.x.x", ">=2.0.0 <3.0.0-0"],
    patch_x => ["1.2.x", ">=1.2.0 <1.3.0-0"],
    minor_asterisk_patch_asterisk => ["2.*.*", ">=2.0.0 <3.0.0-0"],
    patch_asterisk => ["1.2.*", ">=1.2.0 <1.3.0-0"],
    caret_zero => ["^0", "<1.0.0-0"],
    caret_zero_minor => ["^0.1", ">=0.1.0 <0.2.0-0"],
    caret_one => ["^1.0", ">=1.0.0 <2.0.0-0"],
    caret_minor => ["^1.2", ">=1.2.0 <2.0.0-0"],
    caret_patch => ["^0.0.1", ">=0.0.1 <0.0.2-0"],
    caret_with_patch =>   ["^0.1.2", ">=0.1.2 <0.2.0-0"],
    caret_with_patch_2 => ["^1.2.3", ">=1.2.3 <2.0.0-0"],
    tilde_one => ["~1", ">=1.0.0 <2.0.0-0"],
    tilde_minor => ["~1.0", ">=1.0.0 <1.1.0-0"],
    tilde_minor_2 => ["~2.4", ">=2.4.0 <2.5.0-0"],
    tilde_with_greater_than_patch => ["~>3.2.1", ">=3.2.1 <3.3.0-0"],
    tilde_major_minor_zero => ["~1.1.0", ">=1.1.0 <1.2.0-0"],
    grater_than_equals_one => [">=1", ">=1.0.0"],
    greater_than_one => [">1", ">=2.0.0"],
    less_than_one_dot_two => ["<1.2", "<1.2.0-0"],
    greater_than_one_dot_two => [">1.2", ">=1.3.0"],
    greater_than_with_prerelease => [">1.1.0-beta-10", ">1.1.0-beta-10"],
    either_one_version_or_the_other => ["0.1.20 || 1.2.4", "0.1.20||1.2.4"],
    either_one_version_range_or_another => [">=0.2.3 || <0.0.1", ">=0.2.3||<0.0.1"],
    either_x_version_works => ["1.2.x || 2.x", ">=1.2.0 <1.3.0-0||>=2.0.0 <3.0.0-0"],
    either_asterisk_version_works => ["1.2.* || 2.*", ">=1.2.0 <1.3.0-0||>=2.0.0 <3.0.0-0"],
    one_two_three_or_greater_than_four => ["1.2.3 || >4", "1.2.3||>=5.0.0"],
    any_version_asterisk => ["*", "*"],
    any_version_x => ["x", "*"],
    empty_alternatives => ["||", "*"],
    wildcard_alternative => [">=1.0.0 || *", "*"],
    whitespace_1 => [">= 1.0.0", ">=1.0.0"],
    whitespace_2 => [">=  1.0.0", ">=1.0.0"],
    whitespace_3 => [">=   1.0.0", ">=1.0.0"],
    whitespace_4 => ["> 1.0.0", ">1.0.0"],
    whitespace_5 => [">  1.0.0", ">1.0.0"],
    whitespace_6 => ["<=   2.0.0", "<=2.0.0"],
    whitespace_7 => ["<= 2.0.0", "<=2.0.0"],
    whitespace_8 => ["<=  2.0.0", "<=2.0.0"],
    whitespace_9 => ["<    2.0.0", "<2.0.0"],
    whitespace_10 => ["<\t2.0.0", "<2.0.0"],
    whitespace_11 => ["^ 1", ">=1.0.0 <2.0.0-0"],
    whitespace_12 => ["~> 1", ">=1.0.0 <2.0.0-0"],
    whitespace_13 => ["~ 1.0", ">=1.0.0 <1.1.0-0"],
    beta          => ["^0.0.1-beta", ">=0.0.1-beta <0.0.2-0"],
    beta_tilde => ["~1.2.3-beta", ">=1.2.3-beta <1.3.0-0"],
    beta_4        => ["^1.2.3-beta.4", ">=1.2.3-beta.4 <2.0.0-0"],
    pre_release_on_both => ["1.0.0-alpha - 2.0.0-beta", ">=1.0.0-alpha <=2.0.0-beta"],
    single_sided_lower_bound_with_pre_release => [">1.0.0-alpha", ">1.0.0-alpha"],
    space_separated1 => [">=1.2.3 <4.5.6", ">=1.2.3 <4.5.6"],
    garbage1 => ["1.2.3 foo", "1.2.3"],
    garbage2 => ["foo 1.2.3", "1.2.3"],
    garbage3 => ["~1.y 1.2.3", "1.2.3"],
    garbage4 => ["1.2.3 ~1.y", "1.2.3"],
    caret_weird => ["^ 1.2 ^ 1", ">=1.2.0 <2.0.0-0"],
    loose_eq1 => ["=0.7", ">=0.7.0 <0.8.0-0"],
    loose_eq2 => ["=1", ">=1.0.0 <2.0.0-0"],
    consistent => ["^1.0.1", ">=1.0.1 <2.0.0-0"],
    consistent2 => [">=1.0.1 <2.0.0-0", ">=1.0.1 <2.0.0-0"],

    // FIXME: The following test cases were supported by node_semver, but are
    //        not supported by Biome. They look invalid anyway, but we may want
    //        to reconsider if bugs are reported for these.
    // loose1 => [">01.02.03", ">1.2.3"],
    // loose2 => ["~1.2.3beta", ">=1.2.3-beta <1.3.0-0"],
];
