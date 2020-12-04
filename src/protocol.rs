#[derive(Debug, PartialEq)]
pub enum Protocol {
    TCP,
    UDP,
    SCTP,
}
impl Default for Protocol {
    fn default() -> Self {
        Protocol::TCP
    }
}
impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Protocol::TCP => "tcp",
                Protocol::UDP => "udp",
                Protocol::SCTP => "sctp",
            },
        )
    }
}
impl Protocol {
    pub fn parse(p: &str) -> Protocol {
        match p {
            "tcp" => Protocol::TCP,
            "udp" => Protocol::UDP,
            "sctp" => Protocol::SCTP,
            _ => unreachable!(),
        }
    }
}
