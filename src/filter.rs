use std::fmt;
#[derive(Default, Debug, PartialEq)]
pub struct Filter {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub stype: Option<String>,
    pub protocol: Option<String>,
    pub port: Option<u16>,
}
impl Filter {
    pub fn new() -> Filter {
        Filter {
            ..Default::default()
        }
    }
    pub fn kind(&self) -> Option<String> {
        // Prefix stype and protocol with underscores
        if let Some(stype) = &self.stype {
            if let Some(protocol) = &self.protocol {
                return Some(format!("_{}._{}", stype, protocol));
            }
        }
        return None;
    }
}
impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: Vec<String> = Vec::new();
        let kind = self.kind();
        if let Some(name) = &self.name {
            result.push(format!("name=\"{}\"", name.to_owned()));
        }
        if let Some(domain) = &self.domain {
            result.push(format!("domain=\"{}\"", domain.to_owned()));
        }
        if let Some(kind) = kind {
            result.push(format!("kind=\"{}\"", kind));
        }
        if let Some(port) = self.port {
            result.push(format!("port=\"{}\"", port.to_string()));
        }
        let result = result.join(" ");
        write!(f, "{}", result)
    }
}
#[cfg(test)]
mod tests {
    use super::Filter;
    #[test]
    fn test_kind() {
        let f = Filter {
            stype: Some("rust".to_string()),
            protocol: Some("tcp".to_string()),
            ..Default::default()
        };
        assert_eq!(Some("_rust._tcp".to_string()), f.kind())
    }
    #[test]
    fn test_fmt() {
        let f = Filter {
            stype: Some("rust".to_string()),
            protocol: Some("tcp".to_string()),
            ..Default::default()
        };
        assert_eq!(format!("{}", f), "kind=\"_rust._tcp\"");
    }
}
