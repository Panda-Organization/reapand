use clap::ArgMatches;
use lazy_static::lazy_static;
use crate::generate_app;

pub mod args;
pub mod groups;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CRATE_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const CRATE_ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

pub const CRATE_HEADER: &str = "@@@@@@@   @@@@@@@@   @@@@@@   @@@@@@@    @@@@@@   @@@  @@@  @@@@@@@
@@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@ @@@  @@@@@@@@
@@!  @@@  @@!       @@!  @@@  @@!  @@@  @@!  @@@  @@!@!@@@  @@!  @@@
!@!  @!@  !@!       !@!  @!@  !@!  @!@  !@!  @!@  !@!!@!@!  !@!  @!@
@!@!!@!   @!!!:!    @!@!@!@!  @!@@!@!   @!@!@!@!  @!@ !!@!  @!@  !@!
!!@!@!    !!!!!:    !!!@!!!!  !!@!!!    !!!@!!!!  !@!  !!!  !@!  !!!
!!: :!!   !!:       !!:  !!!  !!:       !!:  !!!  !!:  !!!  !!:  !!!
:!:  !:!  :!:       :!:  !:!  :!:       :!:  !:!  :!:  !:!  :!:  !:!
::   :::   :: ::::  ::   :::   ::       ::   :::   ::   ::   :::: ::
 :   : :  : :: ::    :   : :   :         :   : :  ::    :   :: :  : ";
pub const CRATE_AFTER_HELP: &str = "Command Line Usage Examples:\n\
                                    reapand -e b64,zlib -p 20000 -d files/\n\
                                    reapand -e b64,bzip2 -- 127.0.0.1";

lazy_static!{
    pub static ref APP_MATCHES: ArgMatches = generate_app().get_matches();
}

