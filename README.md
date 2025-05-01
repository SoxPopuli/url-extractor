# URL Extractor

Returns a list of URLs from stdin

# URL Structure and Components
A URL (Uniform Resource Locator) is composed of several distinct parts, each
with specific rules about what characters are allowed. Here's a breakdown of
the components:

## 1. Scheme
- Example: `https://`, `http://`, `ftp://`
- Specifies the protocol used
- Must start with a letter and can contain letters, digits, plus (+), period (.), or hyphen (-)
- Always followed by a colon and two forward slashes `://`

## 2. Authority (includes user info, host, and port)
- User info (optional): `username:password@` 
  - Can contain letters, digits, and some special characters
  - Separated from host by `@`
  - Note: Including passwords in URLs is discouraged for security reasons
- Host: `www.example.com`
  - Can be a domain name or IP address
  - Domain names can contain letters, digits, hyphens (but can't start or end with hyphen)
  - Labels separated by periods
- Port (optional): `:8080`
  - Preceded by a colon
  - Numeric only

## 3. Path
- Example: `/products/category/item`
- Specifies the resource location on the server
- Separated from authority by a forward slash
- Can contain letters, digits, and several special characters
- Path segments are separated by forward slashes
- Special characters must be percent-encoded

## 4. Query String (optional)
- Example: `?name=value&search=term`
- Begins with a question mark
- Contains parameters as name-value pairs
- Pairs are separated by ampersands (&)
- Special characters must be percent-encoded

## 5. Fragment (optional)
- Example: `#section2`
- Preceded by a hash symbol
- Points to a specific section within the resource
- Can contain letters, digits, and several special characters
- Special characters must be percent-encoded

## Example of a Complete URL
```
https://username:password@www.example.com:8080/path/to/resource?query=value&name=term#section
```
