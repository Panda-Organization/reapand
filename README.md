# Reapand 
## Author: [Raphael Dray](https://www.linkedin.com/in/raphaeldray/)
Reapand is a fast command line tool written in Rust, built to reap data from 
LFI/[PHP-Filters](https://www.php.net/manual/en/filters.php) source for example.

For now the tool supports principals encoding of php-filters:
* **Zlib**
* **Bzip2**
* **Base64**

## Command Line Usage
```
@@@@@@@   @@@@@@@@   @@@@@@   @@@@@@@    @@@@@@   @@@  @@@  @@@@@@@
@@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@@@@@  @@@@ @@@  @@@@@@@@
@@!  @@@  @@!       @@!  @@@  @@!  @@@  @@!  @@@  @@!@!@@@  @@!  @@@
!@!  @!@  !@!       !@!  @!@  !@!  @!@  !@!  @!@  !@!!@!@!  !@!  @!@
@!@!!@!   @!!!:!    @!@!@!@!  @!@@!@!   @!@!@!@!  @!@ !!@!  @!@  !@!
!!@!@!    !!!!!:    !!!@!!!!  !!@!!!    !!!@!!!!  !@!  !!!  !@!  !!!
!!: :!!   !!:       !!:  !!!  !!:       !!:  !!!  !!:  !!!  !!:  !!!
:!:  !:!  :!:       :!:  !:!  :!:       :!:  !:!  :!:  !:!  :!:  !:!
::   :::   :: ::::  ::   :::   ::       ::   :::   ::   ::   :::: ::
 :   : :  : :: ::    :   : :   :         :   : :  ::    :   :: :  :

reapand 1.2.0
Raphael Dray <dray.raph@gmail.com>
Create files by decoding client input from specified encodings

USAGE:
    reapand.exe [OPTIONS] --encode <encoding>... [--] [ip|hostname]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

ENCODING:
    -e, --encode <encoding>...    Order of encoding applied to the content [possible values: b64,
                                  zlib, bzip2, rot13]

LISTENER:
    -p, --port <port>    Port Listener [default: 4444]
    <ip|hostname>    Host Listener [default: 0.0.0.0]

OUTPUT:
    -d, --dir <dir>    Directory in which will be saved files [default: .]
```