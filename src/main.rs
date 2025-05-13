#![allow(dead_code)]

use std::io::stdin;

use winnow::{
    Result as PResult,
    ascii::alpha1,
    combinator::{opt, seq},
    prelude::*,
    token::{literal, one_of, take_while},
};

fn parse_scheme(x: &mut &str) -> PResult<String> {
    let first_char = one_of(('a'..='z', 'A'..='Z')).parse_next(x)?;

    let rest = take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', '+', '-', '.')).parse_next(x)?;

    let mut s = String::with_capacity(rest.len() + 1);
    s.push(first_char);
    s.push_str(rest);

    Ok(s)
}

mod authority {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    use winnow::{
        ascii::{dec_uint, hex_uint}, combinator::{alt, delimited, not, opt, repeat, terminated}, error::{StrContext::Expected, StrContextValue::Description}, prelude::*, token::{any, take_until}, ModalResult as PResult
    };

    pub fn parse_ipv4(x: &mut &str) -> PResult<Ipv4Addr> {
        let ip = (
            terminated(dec_uint::<_, u8, _>, '.'),
            terminated(dec_uint::<_, u8, _>, '.'),
            terminated(dec_uint::<_, u8, _>, '.'),
            dec_uint::<_, u8, _>,
        )
            .context(Expected(Description("ipv4")))
            .parse_next(x)?;

        Ok(Ipv4Addr::new(ip.0, ip.1, ip.2, ip.3))
    }

    pub fn parse_ipv6(x: &mut &str) -> PResult<Ipv6Addr> {
        let ip = (
            terminated(hex_uint::<_, u16, _>, ':'),
            terminated(hex_uint::<_, u16, _>, ':'),
            terminated(hex_uint::<_, u16, _>, ':'),
            terminated(hex_uint::<_, u16, _>, ':'),
            terminated(hex_uint::<_, u16, _>, ':'),
            terminated(hex_uint::<_, u16, _>, ':'),
            terminated(hex_uint::<_, u16, _>, ':'),
            hex_uint::<_, u16, _>,
        )
            .context(Expected(Description("ipv6")))
            .parse_next(x)?;

        Ok(Ipv6Addr::new(
            ip.0, ip.1, ip.2, ip.3, ip.4, ip.5, ip.6, ip.7,
        ))
    }

    pub fn parse_authority<'a>(x: &mut &'a str) -> PResult<&'a str> {
        fn parse_userinfo<'a>(x: &mut &'a str) -> PResult<(&'a str, Option<&'a str>)> {
            let content = take_until(1.., '@').parse_next(x)?;

            Ok(content
                .split_once(':')
                .map(|(username, password)| (username, Some(password)))
                .unwrap_or((content, None)))
        }

        fn parse_port(x: &mut &str) -> PResult<IpAddr> {
            alt((
                parse_ipv4.map(IpAddr::V4),
                delimited('[', parse_ipv6, ']').map(IpAddr::V6),
                "localhost".map(|_| IpAddr::V4(Ipv4Addr::LOCALHOST)),
                delimited('(', "::1", ')').map(|_| IpAddr::V6(Ipv6Addr::LOCALHOST)),
            ))
            .parse_next(x)
        }

        fn parse_host(x: &mut &str) -> PResult<String> {
            alt((
                    repeat(1.., 
                        any
                    ),
                    parse_port.map(|x| x.to_string())
            )).parse_next(x)
        }

        let userinfo = opt(parse_userinfo).parse_next(x)?;
        let _ = opt('@').parse_next(x)?;

        let host = take_until(1.., ':').parse_next(x)?;
        let _ = opt(':').parse_next(x)?;


        todo!()
    }
}

#[derive(Debug)]
struct Url {
    scheme: Option<String>,
}

fn parse_url(url: &mut &str) -> PResult<Url> {
    seq! { Url {
        scheme: opt(alpha1.map(str::to_string)),
        _: literal(":"),
        _: opt(literal("//")),
    }}
    .parse_next(url)
}

fn main() {
    let stdin = stdin().lock();

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};
    use winnow::error::{ContextError, ParseError};

    use crate::*;

    #[test]
    fn ipv4() {
        fn parse(x: &str) -> Ipv4Addr {
            authority::parse_ipv4.parse(x).unwrap()
        }

        assert_eq!(parse("192.168.0.1"), Ipv4Addr::new(192, 168, 0, 1));
        assert_eq!(parse("255.255.255.255"), Ipv4Addr::new(255, 255, 255, 255));
        assert_eq!(parse("0.0.0.0"), Ipv4Addr::new(0, 0, 0, 0));
        assert!(authority::parse_ipv4.parse("meow").is_err());
    }

    #[test]
    fn ipv6() {
        fn parse(x: &str) -> Result<Ipv6Addr, ParseError<&str, ContextError>> {
            authority::parse_ipv6.parse(x)
        }

        assert_eq!(
            parse("2001:0db8:85a3:0000:0000:8a2e:0370:7334"),
            Ok(Ipv6Addr::new(
                0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334
            ))
        );
        assert!(parse("error").is_err())
    }

    #[test]
    fn parsing_test() {
        #[rustfmt::skip]
        let urls = [
            "http://example.com",                                              // 1. Basic HTTP URL
            "https://www.example.org",                                         // 2. HTTPS URL
            "https://example.com/products",                                    // 3. URL with a path
            "https://example.com/categories/electronics/smartphones",          // 4. URL with a multi-level path
            "https://example.com/search?q=laptops",                            // 5. URL with a query string
            "https://example.com/search?q=laptops&price=500-1000&brand=apple", // 6. URL with multiple query parameters
            "https://example.com/article#conclusion",                          // 7. URL with a fragment
            "https://example.com:8443/secure",                                 // 8. URL with a port number
            "https://username:password@example.com",                           // 9. URL with user information
            "https://blog.example.com/posts",                                  // 10. URL with subdomain
            "http://192.168.1.1",                                              // 11. IP address as host
            "http://[2001:0db8:85a3:0000:0000:8a2e:0370:7334]",                // 12. IPv6 address as host
            "https://example.com/document.pdf",                                // 13. URL with a file extension
            "mailto:user@example.com",                                         // 16. Email URL
            "https://example.com/search?q=New%20York",                         // 17. URL with percent-encoded characters
            "https://example.com/directory/",                                  // 18. URL with a trailing slash
            "https://example.com:8080/api/v2/users",                           // 23. URL with port and path
            "https://example.com/products/women's-shoes",                      // 24. URL with special characters in path
            "https://example.com/page?id=123#section2",                        // 25. URL with fragment and query string
            "https://例子.测试",                                               // 30. URL with internationalized domain name (IDN)
            "example.com",                                                     // 31. Missing scheme (browsers default to HTTP or HTTPS)
            "google.com",                                                      // 32. Missing www subdomain (many sites work with or without it)
            "github.com/marketplace",                                          // 33. Missing trailing slash for directory (browsers typically add it)
        ];

        for mut url in urls {
            let original = url.to_string();
            let parsed = parse_url(&mut url).expect("url parse error");
            println!("{original:<60} = {parsed:?}");
        }
        panic!();
    }
}
