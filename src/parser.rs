use nom::*;
use std::str;
use std::str::FromStr;
use mia::{Function, Special, AST, build_symbol, quoted};
use function;
use special;

// Literals
fn is_bin_digit(byte: u8) -> bool { byte == b'0' || byte == b'1' }

named!(bin_digit,      take_while1!(is_bin_digit));
named!(sign,           recognize!(opt!(tag!("-"))));
named!(float_literal,  recognize!(do_parse!(sign >> digit >> tag!(".") >> digit >> ())));
named!(long_literal2,  recognize!(do_parse!(sign >> bin_digit >> ())));
named!(long_literal8,  recognize!(do_parse!(sign >> oct_digit >> ())));
named!(long_literal10, recognize!(do_parse!(sign >> digit >> ())));
named!(long_literal16, recognize!(do_parse!(sign >> hex_digit >> ())));

// Base types
named!(float<f64>,    map_res!(map_res!(float_literal,  str::from_utf8), |s| { f64::from_str(s) }));
named!(long2<i64>,    map_res!(map_res!(long_literal2,  str::from_utf8), |s| { i64::from_str_radix(s, 2) }));
named!(long8<i64>,    map_res!(map_res!(long_literal8,  str::from_utf8), |s| { i64::from_str_radix(s, 8) }));
named!(long10<i64>,   map_res!(map_res!(long_literal10, str::from_utf8), |s| { i64::from_str_radix(s, 10) }));
named!(long16<i64>,   map_res!(map_res!(long_literal16, str::from_utf8), |s| { i64::from_str_radix(s, 16) }));
named!(long<i64>,
    alt!(
        preceded!(tag!("#b"),       long2 ) |
        preceded!(tag!("#o"),       long8 ) |
        preceded!(opt!(tag!("#d")), long10) |
        preceded!(tag!("#x"),       long16)
    )
);
named!(string<String>, delimited!(tag!("\""), string_content, tag!("\"")));
named!(symbol<&str>,   map_res!(alphanumeric, str::from_utf8));
named!(verb<&str>,     map_res!(
    alt!(
        tag!("+") |
        tag!("-") |
        tag!("*") |
        tag!("/") |
        tag!("=") |
        tag!("'")), str::from_utf8));
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

named!(vlong<Vec<i64>>,  delimited!(tag!("#l("),  many0!(ws!(long)),  tag!(")")));
named!(vfloat<Vec<f64>>, delimited!(tag!("#f("),  many0!(ws!(alt!(float | map!(long, |l| l as f64)))), tag!(")")));

// AST
named!(
    expr<AST>,
    alt_complete!(
        quote    => { |x| quoted(x)       } |
        float    => { |x| float!(x)       } |
        long     => { |x| long!(x)        } |
        string   => { |x| STRING!(x)      } |
        symbol   => { |x| build_symbol(x) } |
        verb     => { |x| build_symbol(x) } |
        vlong    => { |x| LONG!(x)        } |
        vfloat   => { |x| FLOAT!(x)       } |
        list     => { |x| LIST!(x)        }
    )
);

named!(exprs<Vec<AST>>, many0!(ws!(expr)));
named!(list<Vec<AST>>,  do_parse!(tag!("(") >> l: exprs >> tag!(")") >> (l)));
named!(quote<AST>,      do_parse!(tag!("'") >> l: expr >> (l)));
//
named!(pub parse<Vec<AST>>, terminated!(exprs, eof!()));

