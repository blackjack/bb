extern crate toml;
extern crate rustc_serialize;

mod config; 

fn main()
{
    let b = config::Config::parse("/etc/fstab");
    println!("{:?}",b);
}
