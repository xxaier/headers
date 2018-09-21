use std::time::SystemTime;

use util::{EntityTag, HttpDate};
use ::HeaderValue;

/// `If-Range` header, defined in [RFC7233](http://tools.ietf.org/html/rfc7233#section-3.2)
///
/// If a client has a partial copy of a representation and wishes to have
/// an up-to-date copy of the entire representation, it could use the
/// Range header field with a conditional GET (using either or both of
/// If-Unmodified-Since and If-Match.)  However, if the precondition
/// fails because the representation has been modified, the client would
/// then have to make a second request to obtain the entire current
/// representation.
///
/// The `If-Range` header field allows a client to \"short-circuit\" the
/// second request.  Informally, its meaning is as follows: if the
/// representation is unchanged, send me the part(s) that I am requesting
/// in Range; otherwise, send me the entire representation.
///
/// # ABNF
///
/// ```text
/// If-Range = entity-tag / HTTP-date
/// ```
///
/// # Example values
///
/// * `Sat, 29 Oct 1994 19:43:31 GMT`
/// * `\"xyzzy\"`
///
/// # Examples
///
/// ```
/// # extern crate headers_ext as headers;
/// use headers::IfRange;
/// use std::time::{SystemTime, Duration};
///
/// let fetched = SystemTime::now() - Duration::from_secs(60 * 60 * 24);
/// let if_range = IfRange::date(fetched);
/// ```
#[derive(Clone, Debug, PartialEq, Header)]
pub struct IfRange(IfRange_);

impl IfRange {
    /// Create an `IfRange` header with an entity tag.
    pub fn etag(tag: super::ETag) -> IfRange {
        IfRange(IfRange_::EntityTag(tag.0))
    }

    /// Create an `IfRange` header with a date value.
    pub fn date(time: SystemTime) -> IfRange {
        IfRange(IfRange_::Date(time.into()))
    }
}

#[derive(Clone, Debug, PartialEq)]
enum IfRange_ {
    /// The entity-tag the client has of the resource
    EntityTag(EntityTag),
    /// The date when the client retrieved the resource
    Date(HttpDate),
}

impl ::headers_core::decode::TryFromValues for IfRange_ {
    fn try_from_values(values: &mut ::Values) -> Option<Self> {
        let val = values.next()?;

        if let Some(tag) = EntityTag::from_val(val) {
            return Some(IfRange_::EntityTag(tag));
        }

        let date = HttpDate::from_val(val)?;
        Some(IfRange_::Date(date))
    }
}

impl<'a> From<&'a IfRange_> for HeaderValue {
    fn from(if_range: &'a IfRange_) -> HeaderValue {
        match *if_range {
            IfRange_::EntityTag(ref tag) => tag.into(),
            IfRange_::Date(ref date) => date.into(),
        }
    }
}



/*
#[cfg(test)]
mod tests {
    use std::str;
    use *;
    use super::IfRange as HeaderField;
    test_header!(test1, vec![b"Sat, 29 Oct 1994 19:43:31 GMT"]);
    test_header!(test2, vec![b"\"xyzzy\""]);
    test_header!(test3, vec![b"this-is-invalid"], None::<IfRange>);
}
*/
