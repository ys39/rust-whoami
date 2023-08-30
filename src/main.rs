use std::ptr::null;

use anyhow::Result;
use clap::Parser;
/* use std::io::{stdout, BufWriter, Write};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
 */
use libc::getpwuid;
use nix::unistd::geteuid;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Print the user name associated with the current effective user ID.\nSame as id -un.",
    long_about = None)]
struct Cli {}

fn main() -> Result<()> {
    let _args = Cli::parse();

    const NO_UID: u32 = u32::MAX;
    let euid = geteuid();

    let pw = if euid.as_raw() == NO_UID {
        u32::MAX
    } else {
        euid.as_raw()
    };

    //println!("{}", pw);
    Ok(())
}
