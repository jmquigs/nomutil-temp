#[macro_use]
extern crate nom;
use std::str::FromStr;
use std::fmt;

use nom::digit;
use nom::IResult;

named!(pub decimal<usize>,
   map_res!(
       map_res!(
           call!(digit),
           std::str::from_utf8),
       |s| usize::from_str_radix(s, 10)
   )
);

named!(float_sgn_suffix<i32>,
   map!(
       do_parse!(
           sign: opt!(alt!(tag!("+") | tag!("-"))) >>
           expt: decimal >>
           (sign, expt)
       ),
       |(sign, expt): (Option<&[u8]>, usize)| {
           match sign {
               Some(b"+") | None => expt as i32,
               Some(b"-") => -(expt as i32),
               _ => unreachable!(),
           }
       }
    )
);

named!(float_mag<i32>, preceded!(alt!(tag!("e") | tag!("E")), float_sgn_suffix));

named!(pub decimal_bytes<&[u8]>,
    //map!(
        recognize!(
            tuple!(
               opt!(tag!("-")),
               digit,
               opt!(complete!(preceded!(tag!("."), digit))),
               opt!(complete!(float_mag))
            )
        )
    //    Numeral::from_full_dec
    //)
);


pub fn is_ws(c: u8) -> bool {
    match c {
        b' ' | b'\t' | b'\n' | b'\r' => true,
        _ => false
    }
}

#[macro_export]
named!(pub take_ws<&[u8], &[u8]>, take_while1!(is_ws));

// todo: need a whitespace consumer that doesn't allocate a vec
#[macro_export]
named!(pub junk<&[u8], Vec<&[u8]> >, 
   many0!(take_ws));

#[macro_export]
named!(pub ws_sep,
    recognize!(many0!(take_ws))
);

#[macro_export]
macro_rules! skip_ws_all {
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        complete!(
            $i,
            preceded!(ws_sep, $submac!($($args)*))
        )
    });
}

#[macro_export]
macro_rules! skip_ws_tag {
    ($i:expr, $tag:expr) => (
        skip_ws_all!($i, tag!($tag))
    );
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
