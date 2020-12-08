#[cfg(test)]
mod tests;

mod filter;
mod parse_error;
mod parser;

use filter::Filter;
use parse_error::ParseError;

// parser::Rule is generated by Pest but rust-analyzer shows it as an unresolved import
use parser::{Rule, ZeroConfParser};

use pest::Parser;

pub fn parse(filter: &str) -> Result<Filter, ParseError> {
    // Check whether the filter (incorrectly) includes a `host_name` term
    if filter.contains("host_name") {
        return Err(ParseError::new(
            "`host_name` in filter is ignored. Use `name` and `domain` instead",
        ));
    }

    let filter = ZeroConfParser::parse(Rule::filter, filter).unwrap_or_else(|e| panic!("{}", e));

    let mut result: Filter = Filter::new();

    for terms in filter {
        for term in terms.into_inner() {
            match term.as_rule() {
                Rule::name => result.name = Some(term.as_str().to_string()),
                Rule::kind => {
                    for i3p in term.into_inner() {
                        match i3p.as_rule() {
                            Rule::stype => result.stype = Some(i3p.as_str().to_string()),
                            Rule::protocol => result.protocol = Some(i3p.as_str().to_string()),
                            _ => return Err(ParseError::new("Unable to parse filter")),
                        }
                    }
                }
                Rule::domain => result.domain = Some(term.as_str().to_string()),
                Rule::port => {
                    let port = term.as_str().to_string().parse::<u16>();
                    if let Ok(p) = port {
                        result.port = Some(p);
                    } else {
                        return Err(ParseError::new("Unable to parse port"));
                    }
                }
                _ => return Err(ParseError::new("Unable to parse filter")),
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod xtests {
    use super::*;
    #[test]
    fn test_parse() {
        let example = "name=\"freddie\" domain=\"local\" kind=\"_rust._tcp\" port=\"8080\"";
        let f = Filter {
            name: Some("freddie".to_string()),
            domain: Some("local".to_string()),
            stype: Some("rust".to_string()),
            protocol: Some("tcp".to_string()),
            port: Some(8080),
            ..Default::default()
        };
        assert_eq!(Ok(f), parse(example));

        let example ="name=\"freddie\" domain=\"local\" kind=\"_rust._tcp\" port=\"8080\" host_name=\"freddie.local\"";
        assert_eq!(
            Err(ParseError::new(
                "`host_name` in filter is ignored. Use `name` and `domain` instead"
            )),
            parse(example)
        );
    }
}
