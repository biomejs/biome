use crate::parser::MarkdownParser;
use biome_markdown_syntax::{MarkdownSyntaxKind, MarkdownSyntaxKind::*, T};
use biome_parser::{
    prelude::ParsedSyntax::{self, *},
    token_set, Parser,
};

pub(crate) fn at_thematic_break_block(p: &mut MarkdownParser) -> bool {
    p.at_ts(token_set!(T![*], T![-], T![_]))
}

pub(crate) fn parse_thematic_break_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_thematic_break_block(p) {
        return Absent;
    }
    let start_token = match p.cur() {
        T![*] => T![*],
        T![-] => T![-],
        T![_] => T![_],
        _ => unreachable!(),
    };

    parse_thematic_break_list(p, start_token)
}

pub(crate) fn parse_thematic_break_list(
    p: &mut MarkdownParser,
    start_token: MarkdownSyntaxKind,
) -> ParsedSyntax {
    match start_token {
        T![*] => parse_star_list(p),
        T![-] => parse_minus_list(p),
        T![_] => parse_underscore_list(p),
        _ => Absent,
    }
}

macro_rules! parse_list {
    ($name:ident, $token: expr,$item: expr,$list: expr,$thematic_type: expr) => {
        pub(crate) fn $name(p: &mut MarkdownParser) -> ParsedSyntax {
            let thematic_type_mark = p.start();
            let start_new_line = p.source().before_new_line();
            let mut count = 0;

            let list_mark = p.start();
            // same line
            while p.at($token) && p.source().before_new_line() == start_new_line {
                count += 1;
                let star = p.start();
                p.eat($token);
                star.complete(p, $item);
            }
            if count < 3 {
                list_mark.abandon(p);
                thematic_type_mark.abandon(p);
                return Absent;
            }
            list_mark.complete(p, $list);
            Present(thematic_type_mark.complete(p, $thematic_type))
        }
    };
}

parse_list!(
    parse_star_list,
    T![*],
    MD_STAR,
    MD_STAR_LIST,
    MD_STAR_THEMATIC_BREAK_BLOCK
);
parse_list!(
    parse_minus_list,
    T![-],
    MD_MINUS,
    MD_MINUS_LIST,
    MD_MINUS_THEMATIC_BREAK_BLOCK
);
parse_list!(
    parse_underscore_list,
    T![_],
    MD_UNDERSCORE,
    MD_UNDERSCORE_LIST,
    MD_UNDERSCORE_THEMATIC_BREAK_BLOCK
);
