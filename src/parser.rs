use pest_derive::*;

#[derive(Parser)]
#[grammar = "zeroconf.pest"]
pub struct ZeroConfParser;
