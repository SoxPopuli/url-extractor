use std::io::stdin;

use winnow::{
    Result as PResult,
    ascii::alpha1,
    combinator::{alt, opt, seq, terminated},
    prelude::*,
    token::{literal, take_till, take_until},
};

fn parse_scheme<'a>(x: &mut &'a str) -> PResult<&'a str> {
    fn default_scheme<'a>(x: &mut &'a str) -> PResult<&'a str> {
        let scheme = alpha1.parse_next(x)?;
        let _ = literal("://").parse_next(x)?;
        Ok(scheme)
    }

    alt((default_scheme, terminated("mailto", ":"))).parse_next(x)
}

fn parse_authority<'a>(x: &mut &'a str) -> PResult<&'a str> {

    todo!()
}

#[derive(Debug)]
struct Url {
    scheme: Option<String>,
}

fn parse_url(url: &mut &str) -> PResult<Url> {
    seq! { Url {
        scheme: opt(parse_scheme.map(str::to_string)),
    }}
    .parse_next(url)
}

fn main() {
    let stdin = stdin().lock();

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::parse_url;

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
