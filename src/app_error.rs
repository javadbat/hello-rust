use warp::reject::Reject;
use std::fmt::Display;
#[derive(Debug)]
pub enum PaginationError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    RangeError,
    OutOfBound,
}
impl Display for PaginationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PaginationError::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            PaginationError::MissingParameters => write!(f, "Missing parameter"),
            PaginationError::RangeError => write!(f,"start parameter is greater than end parameter"),
            PaginationError::OutOfBound => write!(f,"specified range is out of bound"),
        }
    }
}
impl Reject for PaginationError{

}
