use nom::{bytes::complete, character, IResult};

pub fn colon(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
    complete::take_while1(|byte| byte == b':')(bytes)
}

pub fn crlf(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
    complete::tag(b"\r\n")(bytes)
}

pub fn space(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
    complete::take_while1(character::is_space)(bytes)
}
