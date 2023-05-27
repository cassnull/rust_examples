use url::{Host, Origin, ParseError, Url};

fn main() -> Result<(), ParseError> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    assert_eq!(url.scheme(), "ftp");
    assert_eq!(url.host(), Some(Host::Domain("rust-lang.org")));
    assert_eq!(url.port_or_known_default(), Some(21));
    assert_eq!(
        Origin::Tuple(
            "ftp".to_string(),
            Host::Domain("rust-lang.org".to_owned()),
            21
        ),
        url.origin()
    );
    println!("The origin is as expected!");

    Ok(())
}
