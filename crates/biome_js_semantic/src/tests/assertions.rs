use crate::{semantic_events, SemanticEvent};
use biome_console::{markup, ConsoleExt, EnvConsole};
use biome_diagnostics::location::AsSpan;
use biome_diagnostics::{
    Advices, Diagnostic, DiagnosticExt, Location, LogCategory, PrintDiagnostic, Visit,
};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{AnyJsRoot, JsFileSource, JsSyntaxToken, TextRange, TextSize, WalkEvent};
use biome_rowan::{AstNode, NodeOrToken};
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;

/// This method helps testing scope resolution. It does this
/// iterating [SemanticEventIterator] and storing where each scope start and end. Later it iterates
/// the tree looking at tokens with trailing comments following a specifically patterns
/// specifying if a scope has started or ended.
///
/// ### Available Patterns
///
/// #### Declaration Assertion
///
/// Test if the attached token is a declaration.
/// Pattern: ```/*# <LABEL> */
///
/// Every declaration assertion will be tested if it matches a [SemanticEvent::DeclarationFound].
///
/// Example:
/// ```js
/// let a/*#A*/ = 1;
/// ```
///
/// #### Read Assertion
///
/// Test if the attached token is "reading" the value of a symbol.
/// Pattern: ```/*READ <LABEL> */
///
/// /// Example:
/// ```js
/// let a/*#A*/ = 1;
/// let b = a/*READ A*/ + 1;
/// ```
/// #### Write Assertion
///
/// Test if the attached token is "writing" a value to a symbol.
/// Pattern: ```/*WRITE <LABEL> */
///
/// /// Example:
/// ```js
/// let a/*#A*/;
/// a/*WRITE A */ = 1;
/// ```
///
/// #### At Scope Assertion
///
/// Test if the attached token is a declaration that lives inside the specified scope.
/// Pattern: ```/*@ <LABEL> */```
///
/// Every at scope assertion will be tested if it matches the  ```scope_started_at``` field of [SemanticEvent::DeclarationFound].
///
/// Example:
/// ```js
/// function f() {/*START A*/ let a/*#a*//*@A*/ = 1; }
/// ```
///
/// #### Scope Start Assertion
///
/// Test if the attached token starts a new scope.
/// Pattern: ```/*START <LABEL>*/```
///
/// Every scope start assertion will be tested if it matches a [SemanticEvent::ScopeStarted].
///
/// Example:
/// ```js
/// function f() {/*START SCOPE1*/ }
/// ```
///
/// #### Scope End Assertion
///
/// Test if the attached token ends a scope.
/// Pattern: ```/*END <LABEL>*/```
///
/// Every scope end assertion will be tested if it matches a [SemanticEvent::ScopeEnded].
/// This assertion also tests if the event and the assertion start scope matches.
///
/// Example:
/// ```js
/// function f() {/*START SCOPE1*/ }/*END SCOPE1*/
/// ```
///
/// #### Unique Assertion
///
/// Test if only one event is attached to the token.
/// Pattern: ```/*UNIQUE*/```
///
/// Example:
/// ```js
/// "for(;;) ;/*UNIQUE*/;"
/// ```
///
/// #### No events Assertion
///
/// Test with there are no events attached to the token.
///
/// Example:
/// ```js
/// if(true) ;/*NOEVENT*/;
/// ```
pub fn assert(code: &str, test_name: &str) {
    let r = biome_js_parser::parse(code, JsFileSource::tsx(), JsParserOptions::default());

    if r.has_errors() {
        let mut console = EnvConsole::default();
        for diag in r.into_diagnostics() {
            let error = diag
                .with_file_path("example.js")
                .with_file_source_code(code);
            console.log(markup! {
                {PrintDiagnostic::verbose(&error)}
            });
        }
        panic!("Compilation error");
    }

    // Extract semantic events and index by range

    let mut events_by_pos: FxHashMap<TextSize, Vec<SemanticEvent>> = FxHashMap::default();
    let mut declaration_range_by_start: FxHashMap<TextSize, TextRange> = FxHashMap::default();
    let mut scope_ranges: Vec<TextRange> = vec![];
    for event in semantic_events(r.syntax()) {
        let pos = match event {
            SemanticEvent::DeclarationFound { range, .. } => {
                declaration_range_by_start.insert(range.start(), range);
                range.start()
            }
            SemanticEvent::ScopeStarted { range, .. } => {
                scope_ranges.push(range);
                range.start()
            }
            SemanticEvent::Read { range, .. }
            | SemanticEvent::HoistedRead { range, .. }
            | SemanticEvent::Write { range, .. }
            | SemanticEvent::HoistedWrite { range, .. }
            | SemanticEvent::UnresolvedReference { range, .. } => range.start(),
            SemanticEvent::ScopeEnded { range, .. } => range.end(),
            SemanticEvent::Export { .. } => continue,
        };
        let v = events_by_pos.entry(pos).or_default();
        v.push(event);
    }

    let assertions = SemanticAssertions::from_root(r.tree(), code, test_name);

    // check

    assertions.check(
        code,
        test_name,
        events_by_pos,
        declaration_range_by_start,
        scope_ranges,
    );
}

#[derive(Debug, Diagnostic)]
#[diagnostic(category = "semanticTests")]
struct TestSemanticDiagnostic {
    #[message]
    #[description]
    message: String,

    #[location(span)]
    span: Option<TextRange>,

    #[advice]
    advice: TestAdvice,
}

impl TestSemanticDiagnostic {
    fn new(message: impl Into<String>, span: impl AsSpan) -> Self {
        Self {
            message: message.into(),
            span: span.as_span(),
            advice: TestAdvice::default(),
        }
    }

    fn push_advice(&mut self, range: impl AsSpan, message: impl Into<String>) {
        self.advice.advices.push((range.as_span(), message.into()));
    }
}

#[derive(Debug, Default)]
struct TestAdvice {
    advices: Vec<(Option<TextRange>, String)>,
}

impl Advices for TestAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for (span, message) in &self.advices {
            let location = Location::builder().span(&span).build();
            visitor.record_log(LogCategory::Info, &message)?;
            visitor.record_frame(location)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct DeclarationAssertion {
    range: TextRange,
    declaration_name: String,
}

#[derive(Clone, Debug)]
struct ReadAssertion {
    range: TextRange,
    declaration_assertion_name: String,
}

#[derive(Clone, Debug)]
struct WriteAssertion {
    range: TextRange,
    declaration_assertion_name: String,
}

#[derive(Clone, Debug)]
struct AtScopeAssertion {
    range: TextRange,
    scope_name: String,
}

#[derive(Clone, Debug)]
struct ScopeStartAssertion {
    range: TextRange,
    scope_name: String,
}

#[derive(Clone, Debug)]
struct ScopeEndAssertion {
    range: TextRange,
    scope_name: String,
}

#[derive(Clone, Debug)]
struct NoEventAssertion {
    range: TextRange,
}

#[derive(Clone, Debug)]
struct UniqueAssertion {
    range: TextRange,
}

#[derive(Clone, Debug)]
struct UnresolvedReferenceAssertion {
    range: TextRange,
}

#[derive(Clone, Debug)]
enum SemanticAssertion {
    Declaration(DeclarationAssertion),
    Read(ReadAssertion),
    Write(WriteAssertion),
    ScopeStart(ScopeStartAssertion),
    ScopeEnd(ScopeEndAssertion),
    AtScope(AtScopeAssertion),
    NoEvent(NoEventAssertion),
    Unique(UniqueAssertion),
    UnresolvedReference(UnresolvedReferenceAssertion),
}

impl SemanticAssertion {
    fn try_from(token: &JsSyntaxToken, assertion_text: &str) -> Option<Self> {
        if assertion_text.starts_with("/*#") {
            let name = assertion_text
                .trim()
                .trim_start_matches("/*#")
                .trim_end_matches("*/")
                .trim()
                .to_string();

            Some(SemanticAssertion::Declaration(DeclarationAssertion {
                range: token.parent().unwrap().text_trimmed_range(),
                declaration_name: name,
            }))
        } else if assertion_text.starts_with("/*READ ") {
            let symbol_name = assertion_text
                .trim()
                .trim_start_matches("/*READ ")
                .trim_end_matches("*/")
                .trim()
                .to_string();

            Some(SemanticAssertion::Read(ReadAssertion {
                range: token.parent().unwrap().text_trimmed_range(),
                declaration_assertion_name: symbol_name,
            }))
        } else if assertion_text.starts_with("/*WRITE ") {
            let symbol_name = assertion_text
                .trim()
                .trim_start_matches("/*WRITE ")
                .trim_end_matches("*/")
                .trim()
                .to_string();

            Some(SemanticAssertion::Write(WriteAssertion {
                range: token.parent().unwrap().text_trimmed_range(),
                declaration_assertion_name: symbol_name,
            }))
        } else if assertion_text.contains("/*START") {
            let scope_name = assertion_text
                .trim()
                .trim_start_matches("/*START")
                .trim_end_matches("*/")
                .trim()
                .to_string();
            Some(SemanticAssertion::ScopeStart(ScopeStartAssertion {
                range: token.parent().unwrap().text_trimmed_range(),
                scope_name,
            }))
        } else if assertion_text.contains("/*END") {
            let scope_name = assertion_text
                .trim()
                .trim_start_matches("/*END")
                .trim_end_matches("*/")
                .trim()
                .to_string();
            Some(SemanticAssertion::ScopeEnd(ScopeEndAssertion {
                range: token.parent().unwrap().text_trimmed_range(),
                scope_name,
            }))
        } else if assertion_text.starts_with("/*@") {
            let scope_name = assertion_text
                .trim()
                .trim_start_matches("/*@")
                .trim_end_matches("*/")
                .trim()
                .to_string();
            Some(SemanticAssertion::AtScope(AtScopeAssertion {
                range: token.parent().unwrap().text_trimmed_range(),
                scope_name,
            }))
        } else if assertion_text.contains("/*NOEVENT") {
            Some(SemanticAssertion::NoEvent(NoEventAssertion {
                range: token.parent().unwrap().text_trimmed_range(),
            }))
        } else if assertion_text.contains("/*UNIQUE") {
            Some(SemanticAssertion::Unique(UniqueAssertion {
                range: token.parent().unwrap().text_trimmed_range(),
            }))
        } else if assertion_text.contains("/*?") {
            Some(SemanticAssertion::UnresolvedReference(
                UnresolvedReferenceAssertion {
                    range: token.parent().unwrap().text_trimmed_range(),
                },
            ))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct SemanticAssertions {
    declarations_assertions: BTreeMap<String, DeclarationAssertion>,
    read_assertions: Vec<ReadAssertion>,
    write_assertions: Vec<WriteAssertion>,
    at_scope_assertions: Vec<AtScopeAssertion>,
    scope_start_assertions: BTreeMap<String, ScopeStartAssertion>,
    scope_end_assertions: Vec<ScopeEndAssertion>,
    no_events: Vec<NoEventAssertion>,
    uniques: Vec<UniqueAssertion>,
    unresolved_references: Vec<UnresolvedReferenceAssertion>,
}

impl SemanticAssertions {
    fn from_root(root: AnyJsRoot, code: &str, test_name: &str) -> Self {
        let mut declarations_assertions: BTreeMap<String, DeclarationAssertion> = BTreeMap::new();
        let mut read_assertions = vec![];
        let mut write_assertions = vec![];
        let mut at_scope_assertions = vec![];
        let mut scope_start_assertions: BTreeMap<String, ScopeStartAssertion> = BTreeMap::new();
        let mut scope_end_assertions = vec![];
        let mut no_events = vec![];
        let mut uniques = vec![];
        let mut unresolved_references = vec![];

        for node in root
            .syntax()
            .preorder_with_tokens(biome_rowan::Direction::Next)
        {
            if let WalkEvent::Enter(NodeOrToken::Token(token)) = node {
                let pieces = token
                    .leading_trivia()
                    .pieces()
                    .chain(token.trailing_trivia().pieces());
                for piece in pieces {
                    let text = piece.text();

                    let assertion = SemanticAssertion::try_from(&token, text);
                    match assertion {
                        Some(SemanticAssertion::Declaration(assertion)) => {
                            // Declaration assertions names cannot clash
                            let old = declarations_assertions
                                .insert(assertion.declaration_name.clone(), assertion)
                                .map(|x| x.range);
                            if let Some(old) = old {
                                error_assertion_name_clash(&token, code, test_name, old);
                            }
                        }
                        Some(SemanticAssertion::Read(assertion)) => {
                            read_assertions.push(assertion);
                        }
                        Some(SemanticAssertion::Write(assertion)) => {
                            write_assertions.push(assertion);
                        }
                        Some(SemanticAssertion::ScopeStart(assertion)) => {
                            // Scope start assertions names cannot clash
                            let old = scope_start_assertions
                                .insert(assertion.scope_name.clone(), assertion)
                                .map(|x| x.range);
                            if let Some(old) = old {
                                error_assertion_name_clash(&token, code, test_name, old);
                            }
                        }
                        Some(SemanticAssertion::ScopeEnd(assertion)) => {
                            scope_end_assertions.push(assertion);
                        }
                        Some(SemanticAssertion::AtScope(assertion)) => {
                            at_scope_assertions.push(assertion);
                        }
                        Some(SemanticAssertion::NoEvent(assertion)) => {
                            no_events.push(assertion);
                        }
                        Some(SemanticAssertion::Unique(assertion)) => {
                            uniques.push(assertion);
                        }
                        Some(SemanticAssertion::UnresolvedReference(assertion)) => {
                            unresolved_references.push(assertion);
                        }
                        None => {}
                    };
                }
            }
        }

        Self {
            declarations_assertions,
            read_assertions,
            write_assertions,
            at_scope_assertions,
            scope_start_assertions,
            scope_end_assertions,
            no_events,
            uniques,
            unresolved_references,
        }
    }

    fn check(
        &self,
        code: &str,
        test_name: &str,
        events_by_pos: FxHashMap<TextSize, Vec<SemanticEvent>>,
        declaration_range_by_start: FxHashMap<TextSize, TextRange>,
        scope_ranges: Vec<TextRange>,
    ) {
        // Check every declaration assertion is ok

        for (_, assertion) in self.declarations_assertions.iter() {
            if let Some(events) = events_by_pos.get(&assertion.range.start()) {
                match &events[0] {
                    SemanticEvent::DeclarationFound { .. } => {
                        // OK because we are attached to a declaration
                    }
                    _ => {
                        println!("Assertion: {assertion:?}");
                        println!("Events: {events_by_pos:#?}");
                        error_assertion_not_attached_to_a_declaration(
                            code,
                            assertion.range,
                            test_name,
                        )
                    }
                }
            } else {
                println!("Assertion: {assertion:?}");
                println!("Events: {events_by_pos:#?}");
                error_assertion_not_attached_to_a_declaration(code, assertion.range, test_name);
            }
        }

        // Check every read assertion is ok
        let is_read_assertion = |e: &SemanticEvent| matches!(e, SemanticEvent::Read { .. });

        for assertion in self.read_assertions.iter() {
            let decl = match self
                .declarations_assertions
                .get(&assertion.declaration_assertion_name)
            {
                Some(decl) => decl,
                None => {
                    panic!(
                        "No declaration found with name: {}",
                        assertion.declaration_assertion_name
                    );
                }
            };

            let events = match events_by_pos.get(&assertion.range.start()) {
                Some(events) => events,
                None => {
                    show_all_events(test_name, code, events_by_pos, is_read_assertion);
                    show_unmatched_assertion(test_name, code, assertion, assertion.range);
                    panic!("No read event found at this range");
                }
            };

            let mut unused_match = None;
            let at_least_one_match = events.iter().any(|e| {
                let declaration_at_range = match &e {
                    SemanticEvent::Read { declaration_at, .. }
                    | SemanticEvent::HoistedRead { declaration_at, .. } => {
                        declaration_range_by_start.get(declaration_at)
                    }
                    _ => None,
                };

                if let Some(declaration_at_range) = declaration_at_range {
                    unused_match = Some(format!(
                        "{} != {}",
                        &code[*declaration_at_range], &code[decl.range]
                    ));
                    code[*declaration_at_range] == code[decl.range]
                } else {
                    false
                }
            });

            if !at_least_one_match {
                println!("Assertion: {assertion:?}");
                println!("Events: {events_by_pos:#?}");
                if let Some(unused_match) = unused_match {
                    panic!(
                        "A read event was found, but was discarded because [{unused_match}] when checking {assertion:?}"
                    );
                } else {
                    panic!(
                        "No matching read event found at this range when checking {assertion:?}"
                    );
                }
            }
        }

        // Check every write assertion is ok

        for assertion in self.write_assertions.iter() {
            let decl = match self
                .declarations_assertions
                .get(&assertion.declaration_assertion_name)
            {
                Some(decl) => decl,
                None => {
                    panic!(
                        "No declaration found with name: {}",
                        assertion.declaration_assertion_name
                    );
                }
            };

            let events = match events_by_pos.get(&assertion.range.start()) {
                Some(events) => events,
                None => {
                    println!("Assertion: {assertion:?}");
                    println!("Events: {events_by_pos:#?}");
                    panic!("No write event found at this range");
                }
            };

            let at_least_one_match = events.iter().any(|e| {
                let declaration_at_range = match &e {
                    SemanticEvent::Write { declaration_at, .. }
                    | SemanticEvent::HoistedWrite { declaration_at, .. } => {
                        declaration_range_by_start.get(declaration_at)
                    }
                    _ => None,
                };

                if let Some(declaration_at_range) = declaration_at_range {
                    code[*declaration_at_range] == code[decl.range]
                } else {
                    false
                }
            });

            if !at_least_one_match {
                println!("Assertion: {assertion:?}");
                println!("Events: {events_by_pos:#?}");
                panic!("No matching write event found at this range");
            }
        }

        let is_scope_event = |e: &SemanticEvent| {
            matches!(
                e,
                SemanticEvent::ScopeStarted { .. } | SemanticEvent::ScopeEnded { .. }
            )
        };

        // Check every at scope assertion is ok
        for at_scope_assertion in self.at_scope_assertions.iter() {
            if let Some(events) = events_by_pos.get(&at_scope_assertion.range.start()) {
                // Needs to be a unique event for now
                match &events[0] {
                    SemanticEvent::DeclarationFound {
                        scope_id,
                        hoisted_scope_id,
                        ..
                    } => match self
                        .scope_start_assertions
                        .get(&at_scope_assertion.scope_name)
                    {
                        Some(scope_start_assertion) => {
                            let scope_started_at = &scope_ranges
                                [hoisted_scope_id.unwrap_or(*scope_id).index()]
                            .start();
                            if scope_start_assertion.range.start() != *scope_started_at {
                                show_all_events(test_name, code, events_by_pos, is_scope_event);
                                show_unmatched_assertion(
                                    test_name,
                                    code,
                                    at_scope_assertion,
                                    at_scope_assertion.range,
                                );
                                panic!("Assertion pointing to a wrong scope");
                            }
                            assert_eq!(scope_start_assertion.range.start(), *scope_started_at);
                        }
                        None => {
                            show_all_events(test_name, code, events_by_pos, is_scope_event);
                            show_unmatched_assertion(
                                test_name,
                                code,
                                at_scope_assertion,
                                at_scope_assertion.range,
                            );
                            panic!("Assertion pointing to a wrong scope");
                        }
                    },
                    _ => {
                        error_assertion_not_attached_to_a_declaration(
                            code,
                            at_scope_assertion.range,
                            test_name,
                        );
                    }
                }
            }
        }

        // Check every scope start assertion is ok
        for scope_assertion in self.scope_start_assertions.values() {
            if let Some(events) = events_by_pos.get(&scope_assertion.range.start()) {
                let is_at_least_one_scope_start = events
                    .iter()
                    .any(|e| matches!(e, SemanticEvent::ScopeStarted { .. }));

                if !is_at_least_one_scope_start {
                    panic!("error_scope_assertion_not_attached_to_a_scope_event");
                }
            } else {
                show_all_events(test_name, code, events_by_pos, is_scope_event);
                show_unmatched_assertion(test_name, code, scope_assertion, scope_assertion.range);
                panic!("No scope event found!");
            }
        }

        // Check every scope end assertion is ok
        for scope_end_assertion in self.scope_end_assertions.iter() {
            // Check we have a scope start with the same label.
            let scope_start_assertions_range = match self
                .scope_start_assertions
                .get(&scope_end_assertion.scope_name)
            {
                Some(scope_start_assertions) => scope_start_assertions.range,
                None => {
                    error_scope_end_assertion_points_to_non_existing_scope_start_assertion(
                        code,
                        scope_end_assertion.range,
                        test_name,
                    );
                    continue;
                }
            };

            if let Some(events) = events_by_pos.get(&scope_end_assertion.range.end()) {
                // At least one of the events should be a scope start starting
                // where we expect
                let e = events.iter().find(|event| match event {
                    SemanticEvent::ScopeEnded { range, .. } => {
                        range.start() == scope_start_assertions_range.start()
                    }
                    _ => false,
                });

                if e.is_none() {
                    error_scope_end_assertion_points_to_the_wrong_scope_start(
                        code,
                        scope_end_assertion.range,
                        events,
                        test_name,
                    );
                }
            } else {
                dbg!(events_by_pos);
                panic!("No scope event found. Assertion: {scope_end_assertion:?}");
            }
        }

        // Check every no event assertion

        for assertion in self.no_events.iter() {
            if events_by_pos.contains_key(&assertion.range.start()) {
                panic!("unexpected event at this position")
            }

            if events_by_pos.contains_key(&assertion.range.end()) {
                panic!("unexpected event at this position")
            }
        }

        // Check every unique assertion

        for unique in self.uniques.iter() {
            if let Some(v) = events_by_pos.get(&unique.range.start()) {
                if v.len() > 1 {
                    panic!("unexpected more than one event");
                } else if v.is_empty() {
                    panic!("unexpected no events");
                }
            }

            if let Some(v) = events_by_pos.get(&unique.range.end()) {
                if v.len() > 1 {
                    panic!("unexpected more than one event");
                } else if v.is_empty() {
                    panic!("unexpected no events");
                }
            }
        }

        // Check every unresolved_reference assertion
        let is_unresolved_reference =
            |e: &SemanticEvent| matches!(e, SemanticEvent::UnresolvedReference { .. });

        for unresolved_reference in self.unresolved_references.iter() {
            match events_by_pos.get(&unresolved_reference.range.start()) {
                Some(v) => {
                    let ok = v
                        .iter()
                        .any(|e| matches!(e, SemanticEvent::UnresolvedReference { .. }));
                    if !ok {
                        show_all_events(test_name, code, events_by_pos, is_unresolved_reference);
                        show_unmatched_assertion(
                            test_name,
                            code,
                            unresolved_reference,
                            unresolved_reference.range,
                        );
                        panic!("No UnresolvedReference event found");
                    }
                }
                None => {
                    show_all_events(test_name, code, events_by_pos, is_unresolved_reference);
                    show_unmatched_assertion(
                        test_name,
                        code,
                        unresolved_reference,
                        unresolved_reference.range,
                    );
                    panic!("No UnresolvedReference event found");
                }
            }
        }
    }
}

fn show_unmatched_assertion(
    test_name: &str,
    code: &str,
    assertion: &impl std::fmt::Debug,
    assertion_range: TextRange,
) {
    let assertion_code = &code[assertion_range];

    // eat all trivia at the start and at the end
    let mut start: usize = assertion_range.start().into();
    start += assertion_code
        .bytes()
        .take_while(u8::is_ascii_whitespace)
        .count();
    let mut end: usize = assertion_range.end().into();
    end -= assertion_code
        .bytes()
        .rev()
        .take_while(u8::is_ascii_whitespace)
        .count();

    let diagnostic = TestSemanticDiagnostic::new(
        format!("This assertion was not matched: {assertion:?}"),
        start..end,
    );
    let error = diagnostic
        .with_file_path(test_name.to_string())
        .with_file_source_code(code);

    let mut console = EnvConsole::default();
    console.log(markup! {
        {PrintDiagnostic::verbose(&error)}
    });
}

fn show_all_events<F>(
    test_name: &str,
    code: &str,
    events_by_pos: FxHashMap<TextSize, Vec<SemanticEvent>>,
    f: F,
) where
    F: Fn(&SemanticEvent) -> bool,
{
    let mut console = EnvConsole::default();
    let mut all_events = vec![];
    for (_, events) in events_by_pos {
        for e in events {
            if f(&e) {
                all_events.push(e);
            }
        }
    }

    all_events.sort_by_key(|l| l.range().start());

    for e in all_events {
        let diagnostic = match e {
            SemanticEvent::ScopeStarted { range, .. } => {
                let code = &code[range];
                let mut start: usize = range.start().into();
                start += code.bytes().take_while(|b| b.is_ascii_whitespace()).count();
                TestSemanticDiagnostic::new(format!("{e:?}"), start..start + 1)
            }
            SemanticEvent::ScopeEnded { range, .. } => {
                let code = &code[range];
                let mut start: usize = range.end().into();
                start -= code
                    .bytes()
                    .rev()
                    .take_while(|b| b.is_ascii_whitespace())
                    .count();
                TestSemanticDiagnostic::new(format!("{e:?}"), start - 1..start)
            }
            _ => TestSemanticDiagnostic::new(format!("{e:?}"), e.range()),
        };

        let error = diagnostic
            .with_file_path(test_name.to_string())
            .with_file_source_code(code);

        console.log(markup! {
            {PrintDiagnostic::verbose(&error)}
        });
    }
}

fn error_assertion_not_attached_to_a_declaration(
    code: &str,
    assertion_range: TextRange,
    test_name: &str,
) {
    let diagnostic = TestSemanticDiagnostic::new(
        "This assertion must be attached to a SemanticEvent::DeclarationFound.",
        assertion_range,
    );
    let error = diagnostic
        .with_file_path(test_name.to_string())
        .with_file_source_code(code);

    let mut console = EnvConsole::default();
    console.log(markup! {
        {PrintDiagnostic::verbose(&error)}
    });
    panic!("This assertion must be attached to a SemanticEvent::DeclarationFound.");
}

fn error_assertion_name_clash(
    token: &JsSyntaxToken,
    code: &str,
    test_name: &str,
    old_range: TextRange,
) {
    // If there is already an assertion with the same name. Suggest a rename

    let mut diagnostic =
        TestSemanticDiagnostic::new("Assertion label conflict.", token.text_trimmed_range());
    diagnostic.push_advice(
        token.text_trimmed_range(),
        "There is already a assertion with the same name. Consider renaming this one.",
    );
    diagnostic.push_advice(old_range, "Previous assertion");
    let error = diagnostic
        .with_file_path(test_name.to_string())
        .with_file_source_code(code);

    let mut console = EnvConsole::default();
    console.log(markup! {
        {PrintDiagnostic::verbose(&error)}
    });

    panic!("Assertion label conflict");
}

fn error_scope_end_assertion_points_to_non_existing_scope_start_assertion(
    code: &str,
    range: TextRange,
    file_name: &str,
) {
    let mut diagnostic = TestSemanticDiagnostic::new("Scope start assertion not found.", range);
    diagnostic.push_advice(
        range,
        "This scope end assertion points to a non-existing scope start assertion.",
    );

    let error = diagnostic
        .with_file_path(file_name.to_string())
        .with_file_source_code(code);

    let mut console = EnvConsole::default();
    console.log(markup! {
        {PrintDiagnostic::verbose(&error)}
    });
    panic!("Scope start assertion not found.");
}

fn error_scope_end_assertion_points_to_the_wrong_scope_start(
    code: &str,
    range: TextRange,
    events: &[SemanticEvent],
    file_name: &str,
) {
    let mut diagnostic =
        TestSemanticDiagnostic::new("The scope end found here do not match the assertion", range);

    for e in events {
        diagnostic.push_advice(e.range(), format!("This event was found: {e:?}"));
    }

    let error = diagnostic
        .with_file_path(file_name.to_string())
        .with_file_source_code(code);

    let mut console = EnvConsole::default();
    console.log(markup! {
        {PrintDiagnostic::verbose(&error)}
    });
    panic!("Wrong scope start");
}
