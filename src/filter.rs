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
}
