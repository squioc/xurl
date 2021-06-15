# Xurl

A little tool to manipulate urls.

## Building for source

To build the binary just:

```
cargo build
```

## Installation

To install Xurl, type:

```
cargo install xurl
```

## Commands

Xurl offer severals commands:

### Dissect

The `dissect` command splits and prints the different parts of the url.
These parts are:
- The scheme
- For urls with authentication information, the username and the password
- The host
- The port
- The path to the document
- The query string
- The fragment


```
$ xurl https://username:password@github.com/squioc/xurl?token=1234#Dissect
scheme: https
username: username
password: password
host: github.com
port: 443
path: /squioc/xurl
query: token=1234
fragment: Dissect
```

### Join

The `join` command joins an url and a new path together.

```
$ xurl join https://github.com/squioc/xurl /rust-lang/rust
https://github.com/rust-lang/rust
```

### Encode

The `encode` command applies the [percent-encoding](https://en.wikipedia.org/wiki/Percent-encoding) on the url

```
$ xurl encode https://github.com/squioc/xurl
https%3A%2F%2Fgithub.com%2Fsquioc%2Fxurl
```

### Decode

The `decode` command reverses the [percent-encoding](https://en.wikipedia.org/wiki/Percent-encoding) applyied on an url

```
$ xurl encode https%3A%2F%2Fgithub.com%2Fsquioc%2Fxurl
https://github.com/squioc/xurl
```

### Idna Encode

The `idna-encode` command applies the [punycode](https://en.wikipedia.org/wiki/Punycode) translation on an internationalized domain name.

```
$ xurl idna-encode https://lafierté-bernard.ça/
https://xn--lafiert-bernard-hnb.xn--a-5fa/
```

### Idna Decode

The `idna-decode` command reverses the [punycode](https://en.wikipedia.org/wiki/Punycode) translation applied on an internationalized domain name.

```
$ xurl idna-encode https://xn--lafiert-bernard-hnb.xn--a-5fa/
https://lafierté-bernard.ça/
```
