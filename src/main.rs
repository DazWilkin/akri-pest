use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "zeroconf.pest"]
pub struct ZeroConfParser;

fn main() {
    good();
    // bad();
}
fn good() {
    let tcp = "tcp";
    let rust = "rust";
    let kind = format!("_{}._{}", rust, tcp);
    let full_kind = format!("kind=\"{}\"", &kind);

    let domain = "local";
    let full_domain = format!("domain=\"{}\"", &domain);

    let filter = format!("{} {}", full_domain, full_kind);
    let successful_parse = ZeroConfParser::parse(Rule::filter, &filter);
    println!("{:?}", successful_parse);

    let filter = format!("{} {}", full_kind, full_domain);
    let successful_parse = ZeroConfParser::parse(Rule::filter, &filter);
    println!("{:?}", successful_parse);

    // But permits duplicates
    let filter = format!("{} {}", full_kind, full_kind);
    let successful_parse = ZeroConfParser::parse(Rule::filter, &filter);
    println!("{:?}", successful_parse);
}

#[cfg(test)]
mod tests {
    use super::ZeroConfParser;
    use super::*;
    use lazy_static::*;
    use pest::{consumes_to, parses_to};

    const TCP: &str = "tcp";
    const RUST: &str = "rust";
    const DOMAIN: &str = "local";

    lazy_static! {
        static ref KIND: String = format!("_{}._{}", RUST, TCP);
        static ref FULL_KIND: String = format!("kind=\"{}\"", format!("_{}._{}", RUST, TCP));
        static ref FULL_DOMAIN: String = format!("domain=\"{}\"", DOMAIN);
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
    fn test_protcol() {
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
}
