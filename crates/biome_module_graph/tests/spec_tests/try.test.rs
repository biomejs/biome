use super::*;
use biome_module_graph::{
    ExpressionTypeInput, infer_expression_type,
    type_inference::{
        NormalizedExpressionTypeRequest, TypeInferenceCaller, execute_type_inference_request,
    },
};

fn marked_range(source: &str, marker: &str, expression: &str) -> TextRange {
    let marker = format!("/*{marker}*/");
    let mut occurrences = source.match_indices(&marker);
    let (offset, _) = occurrences.next().expect("source marker must exist");
    assert!(occurrences.next().is_none(), "source marker must be unique");
    let start = offset + marker.len();
    assert!(source[start..].starts_with(expression));
    TextRange::at(
        start.try_into().unwrap(),
        expression.len().try_into().unwrap(),
    )
}

fn narrowing_db(source: &str) -> (TestModuleDb, ModuleInfo) {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    (db, module)
}

/// Names each member of `ty`, sorted: an interface by its name, Biome's
/// Unknown as `Unknown`, and any other type by its formatted text on one
/// line, such as `string` or `null`.
fn variant_names(db: &TestModuleDb, ty: InferredTypeData) -> Vec<String> {
    if ty == InferredTypeData::Unknown {
        return vec!["Unknown".to_owned()];
    }
    let variants = match ty {
        InferredTypeData::Union(union) => union.types(db).to_vec(),
        _ => vec![ty],
    };
    let mut names: Vec<_> = variants
        .into_iter()
        .map(|ty| {
            let text = format_inferred_type(db, ty);
            let first = text.lines().next().unwrap_or_default();
            if let Some(name) = first
                .strip_prefix("interface \"")
                .and_then(|name| name.split('"').next())
            {
                return name.to_owned();
            }
            text.lines().map(str::trim).collect::<Vec<_>>().join(" ")
        })
        .collect();
    names.sort();
    names
}

/// Declarations shared by the tests.
const DECLARATIONS: &str = r#"
    declare function g(): void;
    declare function check(): boolean;
    declare function assertString(input: unknown): asserts input is string;
    declare function guard(input: unknown): input is string;
"#;

/// Asserts the type of each `(marker, expression, expected)` read in `source`,
/// which follows [`DECLARATIONS`], by the names of its members, as
/// [`variant_names`] gives them, and that complete module inference gives
/// each read the type that a lookup gives it.
fn assert_reads(source: &str, reads: &[(&str, &str, &[&str])]) {
    let source = format!("{DECLARATIONS}{source}");
    let lookups: Vec<_> = {
        let (db, module) = narrowing_db(&source);
        reads
            .iter()
            .map(|(marker, expression, _)| {
                let range = marked_range(&source, marker, expression);
                infer_expression_type(&db, ExpressionTypeInput::new(&db, module, range))
                    .map(|ty| format_inferred_type(&db, ty))
            })
            .collect()
    };
    let (db, module) = narrowing_db(&source);
    let complete = infer_module_types(&db, module).unwrap();
    let mismatches: Vec<_> = reads
        .iter()
        .zip(lookups)
        .flat_map(|((marker, expression, expected), lookup)| {
            let range = marked_range(&source, marker, expression);
            let ty = execute_type_inference_request(
                &db,
                TypeInferenceCaller::new("test", "try"),
                NormalizedExpressionTypeRequest::new(module, range),
            )
            .unwrap_or_else(|| panic!("expression at {marker} must be inferred"));
            let actual = variant_names(&db, ty);
            let complete = Some(format_inferred_type(&db, complete.expressions[&range]));
            [
                (actual != *expected)
                    .then(|| format!("{marker}: expected {expected:?}, got {actual:?}")),
                (complete != lookup).then(|| {
                    format!("{marker}: module inference gives {complete:?}, lookups {lookup:?}")
                }),
            ]
        })
        .flatten()
        .collect();
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m4/try.ts` L4, L19, and `m4-l3/extra.ts` L15, L18,
// L30, L31): an exception can leave the try block at any point, so the catch
// clause starts from the state before the statement, and a test in the try
// block never narrows it. A test in the catch clause narrows the rest of it.
#[test]
fn catch_starts_from_the_state_before_try() {
    assert_reads(
        r#"
        function f1(x: string | null) { try { if (x === null) throw 0; /*try*/x; g(); } catch { /*catch*/x; } }
        function f16(x: string | null) { try { if (x === null) throw 0; } catch { /*catchBeforeReturn*/x; return; } finally { } }
        function f19(x: string | null) { try { if (x !== null) { g(); return; } } catch { /*catchAfterReturn*/x; return; } }
        function e8(x: string | null) { try { if (x === null) throw 0; } catch { /*catchAfterThrow*/x; return; } }
        function e9(x: string | null) { try { g(); } catch { if (x === null) throw 0; /*inCatch*/x; } }
        function e12(x: string | null) { if (x === null) return; try { g(); } catch (e) { /*beforeTry*/x; } }
        "#,
        &[
            ("try", "x", &["string"]),
            ("catch", "x", &["null", "string"]),
            ("catchBeforeReturn", "x", &["null", "string"]),
            ("catchAfterReturn", "x", &["null", "string"]),
            ("catchAfterThrow", "x", &["null", "string"]),
            ("inCatch", "x", &["string"]),
            ("beforeTry", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m4/try.ts` L5, L6, L9, L22, and `m4-l3/extra.ts` L8,
// L9, L13 to L15, L18, L22, L23, L28): the code after the statement continues
// from the ends of the try block and of the catch clause, so it keeps the
// tests of a block whose other paths leave the function, and a `break` from
// the try block reaches its target directly.
#[test]
fn the_code_after_try_continues_from_the_normal_exits() {
    assert_reads(
        r#"
        function f2(x: string | null) { try { if (x === null) return; g(); } catch { return; } /*catchReturns*/x; }
        function f3(x: string | null) { try { if (x === null) return; g(); } catch { } /*catchEnds*/x; }
        function f6(x: string | null) { try { g(); } catch { if (x === null) return; } /*catchTest*/x; }
        function f19(x: string | null) { try { if (x !== null) { g(); return; } } catch { return; } /*tryEnd*/x; }
        function e2(value: string | number) { try { assertString(value); /*assertion*/value; } finally {} }
        function e3(value: string | number) { try { if (guard(value)) { /*guard*/value; } } finally {} }
        function e7(x: string | null) { done: { try { if (x === null) break done; } catch { return; } /*inBlock*/x; } /*afterBlock*/x; }
        function e8(x: string | null) { try { if (x === null) throw 0; } catch { return; } /*afterThrow*/x; }
        function e9(x: string | null) { try { g(); } catch { if (x === null) throw 0; } /*catchThrows*/x; }
        function e12(x: string | null) { if (x === null) return; try { g(); } catch (e) { } /*afterCatch*/x; }
        function e16(x: string | null) { try { return; } catch { if (x === null) return; } /*onlyCatch*/x; }
        function e17() { try { g(); } catch (err) { if (err instanceof Error) { /*error*/err; return; } /*notError*/err; } }
        function e21(x: string | null) { try { check(); } catch (error) { check(); } finally { check(); } if (x !== null) /*elsewhere*/x; }
        "#,
        &[
            ("catchReturns", "x", &["string"]),
            ("catchEnds", "x", &["null", "string"]),
            ("catchTest", "x", &["null", "string"]),
            ("tryEnd", "x", &["null"]),
            ("assertion", "value", &["string"]),
            ("guard", "value", &["string"]),
            ("inBlock", "x", &["string"]),
            ("afterBlock", "x", &["null", "string"]),
            ("afterThrow", "x", &["string"]),
            ("catchThrows", "x", &["null", "string"]),
            ("afterCatch", "x", &["string"]),
            ("onlyCatch", "x", &["string"]),
            ("error", "err", &["instanceof Error"]),
            ("notError", "err", &["unknown"]),
            ("elsewhere", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m4/try.ts` L7, L8, L10, L14 to L17, L19, L24, L25, and
// `m4-l3/extra.ts` L7, L10, L11, L16, L19, L21, L24, L25, L29): a finally
// block starts from every way into it, including exceptions and `return`s,
// so its reads see the state before the statement, while the code after it
// runs only when the try block or the catch clause ended normally, and the
// tests in the finally block narrow it too.
#[test]
fn finally_joins_every_way_into_it_and_the_code_after_it_the_normal_exits() {
    assert_reads(
        r#"
        function f4(x: string | null) { try { if (x === null) throw 0; } finally { /*throwFinally*/x; } /*throwAfter*/x; }
        function f5(x: string | null) { try { if (x === null) return; } finally { /*returnFinally*/x; } /*returnAfter*/x; }
        function f7(x: string | null) { try { g(); } catch { if (x === null) return; } finally { /*catchFinally*/x; } /*catchAfter*/x; }
        function f11(x: string | null) { if (x === null) return; try { /*beforeTry*/x; } catch { /*beforeCatch*/x; } finally { /*beforeFinally*/x; } /*beforeAfter*/x; }
        function f12(x: string | null) { try { if (x === null) return; } finally { if (x === null) throw 0; } /*bothTests*/x; }
        function f13(x: string | null) { try { g(); } finally { if (x === null) throw 0; } /*finallyTest*/x; }
        function f14(x: string | null) { try { try { if (x === null) return; } finally { /*innerFinally*/x; } } finally { /*outerFinally*/x; } /*nestedAfter*/x; }
        function f16(x: string | null) { try { if (x === null) throw 0; } catch { return; } finally { /*catchReturnsFinally*/x; } /*catchReturnsAfter*/x; }
        function f21(x: string | null) { try { if (x === null) return; } catch { if (x === null) return; } finally { } /*emptyFinallyAfter*/x; }
        function f22(x: string | null) { try { if (x === null) return; } catch { } finally { /*catchEndsFinally*/x; } }
        function e1(x: string | null) { try { if (x === null) return; /*tryAfterReturn*/x; } finally { /*finally*/x; } /*afterTry*/x; }
        function e4(x: string | null) { try { try { if (x === null) return; } catch { return; } /*nestedCatch*/x; } finally { /*nestedCatchFinally*/x; } /*nestedCatchAfter*/x; }
        function e10(x: string | null) { try { if (x === null) return; } catch { if (x === null) return; } finally { /*bothReturnFinally*/x; } /*bothReturnAfter*/x; }
        function e5(x: string | null, y: string | null) { try { if (x === null) return; } finally { if (y === null) return; } /*twoX*/x; /*twoY*/y; }
        function e13(x: string | null) { try { } finally { try { if (x === null) return; } finally { /*finallyInFinally*/x; } /*afterInnerTry*/x; } /*afterOuterTry*/x; }
        function e15(x: "a" | "b") { try { switch (x) { case "a": return; } /*switchInTry*/x; } finally { /*switchFinally*/x; } /*switchAfter*/x; }
        function e18(x: string | null, y: string | null) { try { if (y === null) return; } finally { switch (x) { case null: return; } } /*switchX*/x; /*switchY*/y; }
        function e19(x: string | null) { try { if (x === null) return; } finally { } x && /*logical*/x; }
        "#,
        &[
            ("throwFinally", "x", &["null", "string"]),
            ("throwAfter", "x", &["string"]),
            ("returnFinally", "x", &["null", "string"]),
            ("returnAfter", "x", &["string"]),
            ("catchFinally", "x", &["null", "string"]),
            ("catchAfter", "x", &["null", "string"]),
            ("beforeTry", "x", &["string"]),
            ("beforeCatch", "x", &["string"]),
            ("beforeFinally", "x", &["string"]),
            ("beforeAfter", "x", &["string"]),
            ("bothTests", "x", &["string"]),
            ("finallyTest", "x", &["string"]),
            ("innerFinally", "x", &["null", "string"]),
            ("outerFinally", "x", &["null", "string"]),
            ("nestedAfter", "x", &["string"]),
            ("catchReturnsFinally", "x", &["null", "string"]),
            ("catchReturnsAfter", "x", &["string"]),
            ("emptyFinallyAfter", "x", &["string"]),
            ("catchEndsFinally", "x", &["null", "string"]),
            ("tryAfterReturn", "x", &["string"]),
            ("finally", "x", &["null", "string"]),
            ("afterTry", "x", &["string"]),
            ("nestedCatch", "x", &["string"]),
            ("nestedCatchFinally", "x", &["null", "string"]),
            ("nestedCatchAfter", "x", &["string"]),
            ("bothReturnFinally", "x", &["null", "string"]),
            ("bothReturnAfter", "x", &["string"]),
            ("twoX", "x", &["string"]),
            ("twoY", "y", &["string"]),
            ("finallyInFinally", "x", &["null", "string"]),
            ("afterInnerTry", "x", &["string"]),
            ("afterOuterTry", "x", &["string"]),
            ("switchInTry", "x", &["string: b"]),
            ("switchFinally", "x", &["string: a", "string: b"]),
            ("switchAfter", "x", &["string: b"]),
            ("switchX", "x", &["string"]),
            ("switchY", "y", &["string"]),
            ("logical", "x", &["string"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m4/try.ts` L18, and `m4-l3/extra.ts` L12, L20): `break`
// and `continue` reach their target directly, without passing through the
// finally block, whose reads still see the state before the statement.
#[test]
fn jumps_reach_their_target_without_the_finally_block() {
    assert_reads(
        r#"
        function f15(xs: number[], x: string | null) { for (const v of xs) { try { if (x === null) break; } finally { /*breakFinally*/x; } } /*breakAfter*/x; }
        function e6(x: string | null) { while (check()) { try { if (x === null) continue; /*afterContinue*/x; } finally { /*continueFinally*/x; } /*continueAfter*/x; } }
        function e14(x: string | null) { for (;;) { try { if (x === null) break; } finally { if (x !== null) continue; } } /*onlyBreak*/x; }
        "#,
        &[
            ("breakFinally", "x", &["null", "string"]),
            ("breakAfter", "x", &["null", "string"]),
            ("afterContinue", "x", &["string"]),
            ("continueFinally", "x", &["null", "string"]),
            ("continueAfter", "x", &["string"]),
            ("onlyBreak", "x", &["null"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives every read below the expected type
// (`narrowing-probes/m4/try.ts` L11 to L13, L20): a catch variable is
// `unknown`, or its annotation, and narrows like any other variable. `any`
// never narrows, so Biome keeps `any` where TypeScript gives `any` too, and a
// catch variable shadows a parameter of the same name.
#[test]
fn catch_variables_narrow() {
    assert_reads(
        r#"
        function f8() { try { g(); } catch (e) { /*declared*/e; if (e instanceof Error) /*instanceof*/e; } }
        function f9() { try { g(); } catch (e: any) { /*any*/e; } }
        function f10() { try { g(); } catch (e: unknown) { if (typeof e === "string") /*typeof*/e; } }
        function f17(x: string | null) { try { g(); } catch (x) { /*shadow*/x; } }
        "#,
        &[
            ("declared", "e", &["unknown"]),
            ("instanceof", "e", &["instanceof Error"]),
            ("any", "e", &["any"]),
            ("typeof", "e", &["string"]),
            ("shadow", "x", &["unknown"]),
        ],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) gives the read below the declared type,
// `string | null` (`narrowing-probes/m4/try.ts` L23), because the code after
// the statement is unreachable: the try block returns, so nothing reaches the
// code after the finally block. Biome records no read in unreachable code, so
// the read keeps its declared type too.
#[test]
fn the_code_after_try_is_unreachable_without_a_normal_exit() {
    assert_reads(
        r#"
        function f20(x: string | null) { try { return; } finally { if (x === null) return; } /*unreachable*/x; }
        "#,
        &[("unreachable", "x", &["null", "string"])],
    );
}

// TypeScript 7.0.2 (`tsc --strict`) narrows `x` to `string` at `afterLabel`
// and at `beforeBreak` (`narrowing-probes/m4-l3/labeled.ts` L3, L4). Biome
// keeps the declared type at every read of a root that contains a `break` to
// the label of a `try` statement, because the statement control-flow graph
// gives such a label no target.
#[test]
fn a_break_to_the_label_of_a_try_statement_stops_narrowing() {
    assert_reads(
        r#"
        function l1(x: string | null) { if (x === null) return; out: try { if (x) break out; } finally { } /*afterLabel*/x; }
        function l2(x: string | null) { out: try { if (x === null) break out; /*beforeBreak*/x; } finally { } /*afterBreak*/x; }
        "#,
        &[
            ("afterLabel", "x", &["null", "string"]),
            ("beforeBreak", "x", &["null", "string"]),
            ("afterBreak", "x", &["null", "string"]),
        ],
    );
}
