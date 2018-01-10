use nom::*;
use std::str;
use std::str::FromStr;
use ast::AST;

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
named!(float<f64>,  map_res!(map_res!(float_literal,  str::from_utf8), |s| { f64::from_str(s) }));
named!(long2<i64>,  map_res!(map_res!(long_literal2,  str::from_utf8), |s| { i64::from_str_radix(s, 2) }));
named!(long8<i64>,  map_res!(map_res!(long_literal10, str::from_utf8), |s| { i64::from_str_radix(s, 8) }));
named!(long10<i64>, map_res!(map_res!(long_literal10, str::from_utf8), |s| { i64::from_str_radix(s, 10) }));
named!(long16<i64>, map_res!(map_res!(long_literal10, str::from_utf8), |s| { i64::from_str_radix(s, 16) }));

named!(
    long<i64>,
    alt!(
        preceded!(tag!("#b"), long2) |
        preceded!(tag!("#o"), long8) |
        preceded!(opt!(tag!("#d")), long10) |
        preceded!(tag!("#x"), long16)
    )
);

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

named!(string<String>, delimited!(tag!("\""), string_content, tag!("\"")));
named!(boolean<bool>, alt!( tag!("#t") => { |_| true } | tag!("#f") => { |_| false }));

// AST
named!(
    expr<AST>,
    alt_complete!(
        float  => { |x| AST::Float(x) } |
        long   => { |x| AST::Long(x)  } |
        string => { |x| AST::String(Box::new(x))}
    )
);

named!(exprs<AST>, terminated!( map!(many0!(ws!(expr)), |v| AST::List(Box::new(v))), eof!()));

pub fn parse(i: &[u8]) -> IResult<&[u8], AST> { exprs(i) }
