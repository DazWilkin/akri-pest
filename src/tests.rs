use super:: ZeroConfParser;
use super::*;
use lazy_static::*;
use pest:: { consumes_to, parses_to };

const NAME: & str = "hades-canyon";
const TCP: & str = "tcp";
const RUST: & str = "rust";
const DOMAIN: & str = "local";
const PORT: & str = "8080";

lazy_static! {
    static ref FULL_NAME: String = format!("name=\"{}\"", NAME);
    static ref KIND: String = format!("_{}._{}", RUST, TCP);
    static ref FULL_KIND: String = format!("kind=\"{}\"", * KIND);
    static ref FULL_DOMAIN: String = format!("domain=\"{}\"", DOMAIN);
    static ref FULL_PORT: String = format!("port={}", PORT);
}

#[test]
fn test_name() {
    parses_to! {
        parser: ZeroConfParser,
            input: NAME,
                rule: Rule:: name,
                    tokens: [
                        name(0, 12)
                    ]
    }
}
#[test]
fn test_tcp() {
    parses_to! {
        parser: ZeroConfParser,
            input: TCP,
                rule: Rule:: tcp,
                    tokens: [
                        tcp(0, 3)
                    ]
    };
}
#[test]
fn test_protocol() {
    parses_to! {
        parser: ZeroConfParser,
            input: TCP,
                rule: Rule:: protocol,
                    tokens: [
                        protocol(0, 3, [
                            tcp(0, 3)
                        ])
                    ]
    };
}
#[test]
fn test_stype() {
    parses_to! {
        parser: ZeroConfParser,
            input: RUST,
                rule: Rule:: stype,
                    tokens: [
                        stype(0, 4)
                    ]
    }
}
#[test]
fn test_kind() {
    parses_to! {
        parser: ZeroConfParser,
            input:& KIND,
                rule: Rule:: kind,
                    tokens: [
                        kind(0, 10, [
                            stype(1, 5),
                            protocol(7, 10, [
                                tcp(7, 10)
                            ])
                        ])
                    ]
    }
}
#[test]
fn test_domain() {
    parses_to! {
        parser: ZeroConfParser,
            input: DOMAIN,
                rule: Rule:: domain,
                    tokens: [
                        domain(0, 5)
                    ]
    };
}
#[test]
fn test_port() {
    parses_to! {
        parser: ZeroConfParser,
            input: PORT,
                rule: Rule:: port,
                    tokens: [
                        port(0, 4)
                    ]
    }
}
#[test]
fn test_term_kind() {
    parses_to! {
        parser: ZeroConfParser,
            input:& FULL_KIND,
                rule: Rule:: term,
                    tokens: [
                        kind(6, 16, [
                            stype(7, 11),
                            protocol(13, 16, [
                                tcp(13, 16)
                            ])
                        ])
                    ]
    }
}
#[test]
fn test_term_domain() {
    parses_to! {
        parser: ZeroConfParser,
            input:& FULL_DOMAIN,
                rule: Rule:: term,
                    tokens: [
                        domain(8, 13)
                    ]
    }
}
#[test]
fn test_filter_kind_domain() {
    let filter = format!("{} {}", * FULL_KIND, * FULL_DOMAIN);
    println!("{:?} [{}]", filter, filter.len());
    parses_to! {
        parser: ZeroConfParser,
            input:& filter,
                rule: Rule:: filter,
                    // 00000000001111111111222222222233
                    // 01234567890123456789012345678901
                    // kind="_rust._tcp" domain="local"
                    tokens: [
                        filter(0, 32, [
                            kind(6, 16, [
                                stype(7, 11),
                                protocol(13, 16, [
                                    tcp(13, 16)
                                ])
                            ]),
                            domain(26, 31)
                        ])
                    ]
    };
}
#[test]
fn test_filter_domain_kind() {
    let filter = format!("{} {}", * FULL_DOMAIN, * FULL_KIND);
    println!("{:?} [{}]", filter, filter.len());
    parses_to! {
        parser: ZeroConfParser,
            input:& filter,
                rule: Rule:: filter,
                    // 00000000001111111111222222222233
                    // 01234567890123456789012345678901
                    // domain="local" kind="_rust._tcp"
                    tokens: [
                        filter(0, 32, [
                            domain(8, 13),
                            kind(21, 31, [
                                stype(22, 26),
                                protocol(28, 31, [
                                    tcp(28, 31)
                                ])
                            ])
                        ])
                    ]
    };
}
#[test]
fn test_filter_domain_kind_name() {
    let filter = format!("{} {} {}", * FULL_DOMAIN, * FULL_KIND, * FULL_NAME);
    println!("{:?} [{}]", filter, filter.len());
    parses_to! {
        parser: ZeroConfParser,
            input:& filter,
                rule: Rule:: filter,
                    // 00000000001111111111222222222233333333334444444444555
                    // 01234567890123456789012345678901234567890123456789012
                    // domain="local" kind="_rust._tcp" name="hades-canyon"X
                    tokens: [
                        filter(0, 52, [
                            domain(8, 13),
                            kind(21, 31, [
                                stype(22, 26),
                                protocol(28, 31, [
                                    tcp(28, 31)
                                ])
                            ]),
                            name(39, 51)
                        ])
                    ]
    };
}
