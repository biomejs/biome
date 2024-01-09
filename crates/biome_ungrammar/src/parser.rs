//! Simple hand-written ungrammar parser.
use std::collections::HashMap;

use crate::{
    error::{bail, format_err, Result},
    lexer::{self, CombinatorKind, TokenKind},
    Grammar, Node, NodeData, Rule, Token, TokenData,
};

macro_rules! bail {
    ($loc:expr, $($tt:tt)*) => {{
        let err = $crate::error::format_err!($($tt)*)
            .with_location($loc);
        return Err(err);
    }};
}

pub(crate) fn parse(tokens: Vec<lexer::Token>) -> Result<Grammar> {
    let mut p = Parser::new(tokens);
    while !p.is_eof() {
        node(&mut p)?;
    }
    p.finish()
}

#[derive(Default)]
struct Parser {
    grammar: Grammar,
    tokens: Vec<lexer::Token>,
    node_table: HashMap<String, Node>,
    token_table: HashMap<String, Token>,
}

const DUMMY_RULE: Rule = Rule::Node(Node(!0));

impl Parser {
    fn new(mut tokens: Vec<lexer::Token>) -> Parser {
        tokens.reverse();
        Parser {
            tokens,
            ..Parser::default()
        }
    }

    fn peek(&self) -> Option<&lexer::Token> {
        self.peek_n(0)
    }
    fn peek_n(&self, n: usize) -> Option<&lexer::Token> {
        self.tokens.iter().nth_back(n)
    }
    fn bump(&mut self) -> Result<lexer::Token> {
        self.tokens
            .pop()
            .ok_or_else(|| format_err!("unexpected EOF"))
    }
    fn expect(&mut self, kind: TokenKind, what: &str) -> Result<()> {
        let token = self.bump()?;
        if token.kind != kind {
            bail!(token.loc, "unexpected token, expected `{}`", what);
        }
        Ok(())
    }
    fn is_eof(&self) -> bool {
        self.tokens.is_empty()
    }
    fn finish(self) -> Result<Grammar> {
        for node_data in &self.grammar.nodes {
            if matches!(node_data.rule, DUMMY_RULE) {
                crate::error::bail!("Undefined node: {}", node_data.name)
            }
        }
        Ok(self.grammar)
    }
    fn intern_node(&mut self, name: String) -> Node {
        let len = self.node_table.len();
        let grammar = &mut self.grammar;
        *self.node_table.entry(name.clone()).or_insert_with(|| {
            grammar.nodes.push(NodeData {
                name,
                rule: DUMMY_RULE,
            });
            Node(len)
        })
    }
    fn intern_token(&mut self, name: String) -> Token {
        let len = self.token_table.len();
        let grammar = &mut self.grammar;
        *self.token_table.entry(name.clone()).or_insert_with(|| {
            grammar.tokens.push(TokenData { name });
            Token(len)
        })
    }
}

/// Parse a full Node. The entire production of:
/// name '=' Rule.
fn node(p: &mut Parser) -> Result<()> {
    let token = p.bump()?;
    let node = match token.kind {
        TokenKind::Node(it) => p.intern_node(it),
        _ => bail!(token.loc, "expected ident"),
    };
    p.expect(TokenKind::Eq, "=")?;
    if !matches!(p.grammar[node].rule, DUMMY_RULE) {
        bail!(token.loc, "duplicate rule: `{}`", p.grammar[node].name)
    }

    let rule = rule(p)?;
    p.grammar.nodes[node.0].rule = rule;
    Ok(())
}

/// Parse any Rule, the right-hand side of a production. This handles
/// all of the combinators other than juxtaposition:
/// 'auto' | Expr | Value
/// length || color || direction
/// veritcal && horizontal
fn rule(p: &mut Parser) -> Result<Rule> {
    if let Some(lexer::Token {
        kind: TokenKind::Pipe | TokenKind::DoubleAmpersand | TokenKind::DoublePipe,
        loc,
    }) = p.peek()
    {
        bail!(
            *loc,
            "The first element in a sequence of productions or alternatives \
            must not be a combinator (`|`, `||`, or `&&`)"
        );
    }

    let lhs = seq_rule(p)?;
    let mut rules = vec![lhs];
    let mut combinator_kind: Option<CombinatorKind> = None;
    while let Some(token) = p.peek() {
        let token_combinator = CombinatorKind::new(&token.kind);

        if matches!(token_combinator, CombinatorKind::NonCombinator) {
            break;
        }

        match combinator_kind {
            Some(kind) if kind != token_combinator => {
                bail!(token.loc, "Cannot mix combinators at the same level in a Rule. Use parentheses to specify precedence");
            }
            None => combinator_kind = Some(token_combinator),
            _ => (),
        }

        p.bump()?;
        let rule = seq_rule(p)?;
        rules.push(rule)
    }
    let res = if rules.len() == 1 {
        rules.pop().unwrap()
    } else {
        match combinator_kind {
            Some(CombinatorKind::DoubleAmpersand) => Rule::UnorderedAll(rules),
            Some(CombinatorKind::DoublePipe) => Rule::UnorderedSome(rules),
            Some(CombinatorKind::Pipe) => Rule::Alt(rules),
            None | Some(CombinatorKind::NonCombinator) => {
                unreachable!("Matched more than one rule but didn't determine a combinator")
            }
        }
    };
    Ok(res)
}

/// Parse a multi-element sequence as a single Rule:
/// 'while' '(' Expr ')'
fn seq_rule(p: &mut Parser) -> Result<Rule> {
    let lhs = atom_rule(p)?;

    let mut seq = vec![lhs];
    while let Some(rule) = opt_atom_rule(p)? {
        seq.push(rule)
    }
    let res = if seq.len() == 1 {
        seq.pop().unwrap()
    } else {
        Rule::Seq(seq)
    };
    Ok(res)
}

/// Parse any single-element Rule, returning an Error if no rule is parsed.
/// Rule
/// Rule*
/// Rule?
/// ( Rule )
fn atom_rule(p: &mut Parser) -> Result<Rule> {
    match opt_atom_rule(p)? {
        Some(it) => Ok(it),
        None => {
            let token = p.bump()?;
            bail!(token.loc, "unexpected token")
        }
    }
}

/// Parse any single-element Rule. Returns None if no rule is parsed.
/// Rule
/// Rule*
/// Rule?
/// ( Rule )
fn opt_atom_rule(p: &mut Parser) -> Result<Option<Rule>> {
    let token = match p.peek() {
        Some(it) => it,
        None => return Ok(None),
    };
    let mut res = match &token.kind {
        TokenKind::Node(name) => {
            if let Some(lookahead) = p.peek_n(1) {
                match lookahead.kind {
                    TokenKind::Eq => return Ok(None),
                    TokenKind::Colon => {
                        let label = name.clone();
                        p.bump()?;
                        p.bump()?;
                        let rule = atom_rule(p)?;
                        let res = Rule::Labeled {
                            label,
                            rule: Box::new(rule),
                        };
                        return Ok(Some(res));
                    }
                    _ => (),
                }
            }
            match p.peek_n(1) {
                Some(token) if token.kind == TokenKind::Eq => return Ok(None),
                _ => (),
            }
            let name = name.clone();
            p.bump()?;
            let node = p.intern_node(name);
            Rule::Node(node)
        }
        TokenKind::Token(name) => {
            let name = name.clone();
            p.bump()?;
            let token = p.intern_token(name);
            Rule::Token(token)
        }
        TokenKind::LParen => {
            p.bump()?;
            let rule = rule(p)?;
            p.expect(TokenKind::RParen, ")")?;
            rule
        }
        _ => return Ok(None),
    };

    if let Some(token) = p.peek() {
        match &token.kind {
            TokenKind::QMark => {
                p.bump()?;
                res = Rule::Opt(Box::new(res));
            }
            TokenKind::Star => {
                p.bump()?;
                res = Rule::Rep(Box::new(res));
            }
            _ => (),
        }
    }
    Ok(Some(res))
}
