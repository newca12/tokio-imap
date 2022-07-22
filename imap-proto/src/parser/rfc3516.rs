//!
//! https://tools.ietf.org/html/rfc3516
//!
//! IMAP4 Binary Content Extension
//!

use nom::{
    bytes::streaming::{tag, tag_no_case},
    character::streaming::char,
    combinator::{map, opt},
    sequence::{delimited, tuple},
    IResult,
};
use std::borrow::Cow;

use crate::{parser::core::*, types::*};

use super::rfc3501::body::section;

pub fn msg_att_binary_section(i: &[u8]) -> IResult<&[u8], AttributeValue> {
    map(
        tuple((
            tag_no_case("BINARY"),
            section,
            opt(delimited(char('<'), number, char('>'))),
            tag(" ~"),
            nstring,
        )),
        |(_, section, index, _, data)| AttributeValue::BodySection {
            section,
            index,
            data: data.map(Cow::Borrowed),
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_response;

    #[test]
    #[ignore]
    //not implemented yet
    fn test_uid_fetch_binary_size() {
        const RESPONSE: &[u8] = b"* 24 FETCH (UID 130500 BINARY.SIZE[2] 1010\r\n";

        match parse_response(RESPONSE) {
            Ok((_, Response::Fetch(_, _))) => {}
            rsp => panic!("unexpected response {:?}", rsp),
        }
    }

    #[test]
    #[ignore]
    //not implemented yet
    //. uid fetch 130504 BODY.PEEK[2]<0.5>
    fn test_uid_fetch_body_subpart() {
        const RESPONSE: &[u8] = b"* 28 FETCH (UID 130504 BODY[2]<0> {5}";

        match parse_response(RESPONSE) {
            Ok((_, Response::Fetch(_, _))) => {}
            rsp => panic!("unexpected response {:?}", rsp),
        }
    }

    #[test]
    #[ignore]
    //not implemented yet
    //. uid fetch 130504 BINARY.PEEK[2]<0.5>
    fn test_uid_fetch_binary_subpart() {
        const RESPONSE: &[u8] = b"* 28 FETCH (UID 130504 BINARY[2]<0> {5}";

        match parse_response(RESPONSE) {
            Ok((_, Response::Fetch(_, _))) => {}
            rsp => panic!("unexpected response {:?}", rsp),
        }
    }

    #[test]
    fn test_uid_fetch_binary() {
        const RESPONSE: &[u8] =
            b"* 43 FETCH (UID 130499 BINARY[1] ~{21}\r\nChat-Version: 1.0\r\n\r\n)\r\n";

        match parse_response(RESPONSE) {
            Ok((_, Response::Fetch(_, _))) => {}
            rsp => panic!("unexpected response {:?}", rsp),
        }
    }
}
