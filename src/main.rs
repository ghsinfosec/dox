#![allow(unused_imports)]
use std::{env, println};
use std::net::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = &args[1];
    
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let mut ips: Vec<std::net::IpAddr> = Vec::new();

    let response = resolver.lookup_ip(source).unwrap(); 
        for res in response {
            ips.push(res);
        }
    println!("{:?}", ips);

}
