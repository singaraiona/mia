use nom::*;
use std::str;
use std::str::FromStr;
use ast::{Verb, AST};

// Literals
fn is_bin_digit(byte: u8) -> bool { byte == b'0' || byte == b'1' }

named!(bin_digit,      take_while1!(is_bin_digit));
named!(sign,           recognize!(opt!(tag!("-"))));
named!(float_literal,  recognize!(do_parse!(sign >> digit >> tag!(".") >> digit >> ())));
named!(long_literal2,  recognize!(do_parse!(sign >> bin_digit >> ())));
named!(long_literal8,  recognize!(do_parse!(sign >> oct_digit >> ())));
named!(long_literal10, recognize!(do_parse!(sign >> digit >> ())));
named!(long_literal16, recognize!(do_parse!(sign >> hex_digit >> ())));
named!(symbol_literal, recognize!(do_parse!(alphanumeric >> ())));

// Base types
named!(boolean<bool>, alt!(tag!("#t") => { |_| true } | tag!("#f") => { |_| false }));
named!(float<f64>,    map_res!(map_res!(float_literal,  str::from_utf8), |s| { f64::from_str(s) }));
named!(long2<i64>,    map_res!(map_res!(long_literal2,  str::from_utf8), |s| { i64::from_str_radix(s, 2) }));
named!(long8<i64>,    map_res!(map_res!(long_literal8,  str::from_utf8), |s| { i64::from_str_radix(s, 8) }));
named!(long10<i64>,   map_res!(map_res!(long_literal10, str::from_utf8), |s| { i64::from_str_radix(s, 10) }));
named!(long16<i64>,   map_res!(map_res!(long_literal16, str::from_utf8), |s| { i64::from_str_radix(s, 16) }));
named!(long<i64>,
    alt!(
        preceded!(tag!("#b"), long2)        |
        preceded!(tag!("#o"), long8)        |
        preceded!(opt!(tag!("#d")), long10) |
        preceded!(tag!("#x"), long16)
    )
);
named!(string<String>, delimited!(tag!("\""), string_content, tag!("\"")));
named!(symbol<String>, map!(map_res!(alphanumeric, str::from_utf8), |s| s.to_string()));

named!(
    string_content<String>,
    map!(
        escaped_transform!(
            take_until_either!("\"\\"),
            '\\',
            alt!(
                tag!("\\") => { |_| &b"\\"[..] } |
                tag!("\"") => { |_| &b"\""[..] } |
                tag!("n")  => { |_| &b"\n"[..] } |
                tag!("r")  => { |_| &b"\r"[..] } |
                tag!("t")  => { |_| &b"\t"[..] }
            )
        ),
        |i: Vec<_>| String::from_utf8_lossy(&i).into_owned()
    )
);

named!(
    verb<Verb>,
    alt_complete!(
        tag!("+") => { |_| Verb::Plus   } |
        tag!("-") => { |_| Verb::Minus  } |
        tag!("*") => { |_| Verb::Times  } |
        tag!("/") => { |_| Verb::Divide } |
        tag!("'") => { |_| Verb::Quote  }
    )
);

// AST
named!(
    expr<AST>,
    alt_complete!(
        boolean => { |x| AST::Bool(x)             } |
        long    => { |x| AST::Long(x)             } |
        float   => { |x| AST::Float(x)            } |
        string  => { |x| AST::String(Box::new(x)) } |
        symbol  => { |x| AST::Symbol(Box::new(x)) } |
        verb    => { |x| AST::Verb(x)             } |
        list    => { |x| x }
    )
);

named!(exprs<AST>, map!(many0!(ws!(expr)), |v| AST::List(Box::new(v))));
named!(list<AST>,  do_parse!(tag!("(") >> l: exprs >> tag!(")") >> (l)));
//
named!(pub parse<AST>, terminated!(exprs, eof!()));

