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
use std::ffi::CStr;
use std::io::{self, Write};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Print the user name associated with the current effective user ID.\nSame as id -un.",
    long_about = None)]
struct Cli {}

fn main() -> Result<()> {
    let _args = Cli::parse();

    const NO_UID: i32 = -1;
    let euid = geteuid();

    let pw = if euid.as_raw() as i32 == NO_UID {
        null()
    } else {
        unsafe { getpwuid(euid.as_raw()) }
    };

    let err_msg = "cannot find name for user ID %lu";
    if !pw.is_null() {
        let username_cstr = unsafe { CStr::from_ptr((*pw).pw_name) };
        let username_str = username_cstr.to_str().unwrap_or(err_msg);
        writeln!(io::stdout(), "{}", username_str)?;
    } else {
        writeln!(io::stdout(), "{}", err_msg)?;
    }
    Ok(())
}
