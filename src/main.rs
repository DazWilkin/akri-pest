use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "zeroconf.pest"]
pub struct ZeroConfParser;

fn main() {
    let pairs = ZeroConfParser::parse(
        Rule::filter,
        "name=\"hades-canyon\" domain=\"local\" kind=\"_rust._tcp\"",
    )
    .unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for i1p in pair.into_inner() {
            match i1p.as_rule() {
                Rule::term => {
                    println!("Term:  {}", i1p.as_str());
                    for i2p in i1p.into_inner() {
                        match i2p.as_rule() {
                            Rule::full_kind => {
                                println!("Kind: {}", i2p.as_str());
                                for i3p in i2p.into_inner() {
                                    match i3p.as_rule() {
                                        Rule::kind => {
                                            println!("Kind: {}", i3p.as_str());
                                            for i4p in i3p.into_inner() {
                                                match i4p.as_rule() {
                                                    Rule::full_stype => {
                                                        println!("Type: {}", i4p.as_str())
                                                    }
                                                    Rule::full_protocol => {
                                                        println!("Protocol: {}", i4p.as_str())
                                                    }
                                                    _ => unreachable!(),
                                                }
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                            }
                            Rule::full_name => println!("Name: {}", i2p.as_str()),
                            Rule::full_domain => println!("Domain: {}", i2p.as_str()),
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::ZeroConfParser;
    use super::*;
    use lazy_static::*;
    use pest::{consumes_to, parses_to};

    const NAME: &str = "hades-canyon";
    const TCP: &str = "tcp";
    const RUST: &str = "rust";
    const DOMAIN: &str = "local";

    lazy_static! {
        static ref FULL_NAME: String = format!("name=\"{}\"", NAME);
        static ref KIND: String = format!("_{}._{}", RUST, TCP);
        static ref FULL_KIND: String = format!("kind=\"{}\"", *KIND);
        static ref FULL_DOMAIN: String = format!("domain=\"{}\"", DOMAIN);
    }

    #[test]
    fn test_name() {
        parses_to! {
            parser:ZeroConfParser,
            input:NAME,
            rule:Rule::name,
            tokens:[
                name(0,12)
            ]
        }
    }
    #[test]
    fn test_full_name() {
        parses_to! {
            parser:ZeroConfParser,
            input:&FULL_NAME,
            rule:Rule::full_name,
            tokens:[
                full_name(0,19,[
                    name(6,18)
                ])
            ]
        }
    }
    #[test]
    fn test_tcp() {
        parses_to! {
            parser:ZeroConfParser,
            input:TCP,
            rule: Rule::tcp,
            tokens:[
                tcp(0,3)
            ]
        };
    }
    #[test]
    fn test_protocol() {
        parses_to! {
            parser:ZeroConfParser,
            input:TCP,
            rule: Rule::protocol,
            tokens:[
                protocol(0,3,[
                    tcp(0,3)
                    ])
            ]
        };
    }
    #[test]
    fn test_stype() {
        parses_to! {
            parser:ZeroConfParser,
            input:RUST,
            rule:Rule::stype,
            tokens:[
                stype(0,4)
            ]
        }
    }
    #[test]
    fn test_kind() {
        parses_to! {
            parser:ZeroConfParser,
            input:&KIND,
            rule:Rule::kind,
            tokens:[
                kind(0,10,[
                    full_stype(0,5,[
                        stype(1,5)
                    ]),
                    full_protocol(6,10,[
                        protocol(7,10,[
                            tcp(7,10)
                        ])
                    ])
                ])
            ]
        }
    }
    #[test]
    fn test_full_kind() {
        parses_to! {
            parser:ZeroConfParser,
            input:&FULL_KIND,
            rule:Rule::full_kind,
            tokens:[
                full_kind(0,17,[
                    kind(6,16,[
                        full_stype(6,11,[
                            stype(7,11)
                        ]),
                        full_protocol(12,16,[
                            protocol(13,16,[
                                tcp(13,16)
                            ])
                        ])
                    ])
                ])
            ]
        }
    }
    #[test]
    fn test_domain() {
        parses_to! {
            parser:ZeroConfParser,
            input:DOMAIN,
            rule: Rule::domain,
            tokens:[
                domain(0,5)
            ]
        };
    }
    #[test]
    fn test_full_domain() {
        parses_to! {
            parser:ZeroConfParser,
            input:&FULL_DOMAIN,
            rule: Rule::full_domain,
            tokens:[
                full_domain(0,14,[
                    domain(8,13)
                ])
            ]
        };
    }
    #[test]
    fn test_term_kind() {
        parses_to! {
            parser:ZeroConfParser,
            input:&FULL_KIND,
            rule:Rule::term,
            tokens:[
                term(0,17,[
                    full_kind(0,17,[
                        kind(6,16,[
                            full_stype(6,11,[
                                stype(7,11)
                            ]),
                            full_protocol(12,16,[
                                protocol(13,16,[
                                    tcp(13,16)
                                ])
                            ])
                        ])
                    ])
                ])
            ]
        }
    }
    #[test]
    fn test_term_domain() {
        parses_to! {
            parser:ZeroConfParser,
            input:&FULL_DOMAIN,
            rule:Rule::term,
            tokens:[
                term(0,14,[
                    full_domain(0,14,[
                        domain(8,13)
                    ])
                ])
            ]
        }
    }
    #[test]
    fn test_filter_kind_domain() {
        let f = format!("{} {}", *FULL_KIND, *FULL_DOMAIN);
        println!("{:?} [{}]", f, f.len());
        parses_to! {
            parser:ZeroConfParser,
            input:&f,
            rule:Rule::filter,
            // 00000000001111111111222222222233
            // 01234567890123456789012345678901
            // kind="_rust._tcp" domain="local"
            tokens:[
                filter(0,32,[
                    term(0,17,[
                        full_kind(0,17,[
                            kind(6,16,[
                                full_stype(6,11,[
                                    stype(7,11)
                                ]),
                                full_protocol(12,16,[
                                    protocol(13,16,[
                                        tcp(13,16)
                                    ])
                                ])
                            ])
                        ])
                    ]
                    ),
                    term(18,32,[
                        full_domain(18,32,[
                            domain(26,31)
                        ])
                    ])
                ])
            ]
        };
    }
    #[test]
    fn test_filter_domain_kind() {
        let f = format!("{} {}", *FULL_DOMAIN, *FULL_KIND);
        println!("{:?} [{}]", f, f.len());
        parses_to! {
            parser:ZeroConfParser,
            input:&f,
            rule:Rule::filter,
            // 00000000001111111111222222222233
            // 01234567890123456789012345678901
            // domain="local" kind="_rust._tcp"
            tokens:[
                filter(0,32,[
                    term(0,14,[
                        full_domain(0,14,[
                            domain(8,13)
                        ])
                    ]),
                    term(15,32,[
                        full_kind(15,32,[
                            kind(21,31,[
                                full_stype(21,26,[
                                    stype(22,26)
                                ]),
                                full_protocol(27,31,[
                                    protocol(28,31,[
                                        tcp(28,31)
                                    ])
                                ])
                            ])
                        ])
                    ])
                ])
            ]
        };
    }
    #[test]
    fn test_filter_domain_kind_name() {
        let f = format!("{} {} {}", *FULL_DOMAIN, *FULL_KIND, *FULL_NAME);
        println!("{:?} [{}]", f, f.len());
        parses_to! {
            parser:ZeroConfParser,
            input:&f,
            rule:Rule::filter,
            // 00000000001111111111222222222233333333334444444444555
            // 01234567890123456789012345678901234567890123456789012
            // domain="local" kind="_rust._tcp" name="hades-canyon"X
            tokens:[
                filter(0,52,[
                    term(0,14,[
                        full_domain(0,14,[
                            domain(8,13)
                        ])
                    ]),
                    term(15,32,[
                        full_kind(15,32,[
                            kind(21,31,[
                                full_stype(21,26,[
                                    stype(22,26)
                                ]),
                                full_protocol(27,31,[
                                    protocol(28,31,[
                                        tcp(28,31)
                                    ])
                                ])
                            ])
                        ])
                    ]),
                    term(33,52,[
                        full_name(33,52,[
                            name(39,51)
                        ])
                    ])
                ])
            ]
        };
    }
}
