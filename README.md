# Sapozhok

Dispatch server implementation for YuanShen / Genshin Impact video game.

Supported game versions: 1.4.5x - 3.1.5x (depends on protocol definitions provided and keys used)

**Note**: Github repo is a mirror of the main repo located at [Invisible Internet Bublik](http://bublik.i2p).
In case Github mirror dies, use I2P to access the main site.

# Building

## Toolkit preparation

You'll need any C/C++ toolkit installed:

- On Windows, MS VS Community or MinGW will do the job;
- On *nix, you just need GCC / Clang

Also you'll need to install Rust.

- On Windows, refer to [official instructions](https://www.rust-lang.org/tools/install)
- On Linux, use system package manager to install `rustc` and `cargo`

## Preparing the workplace

Clone repository with `git clone --recurse-submodules <repo_url>`. This is required to initialize all submodules.

## Retrieving protocol definitions

You'll need several dispatch-specific protobuf befinitions. Look into `dispatch_proto/build.rs` script to figure out their names.

## Supplying encryption keys

There're a few keys you'll need to successfully run dispatch server:

- SSL key and certificate
- Initial traffic encryption key
- RSA keys for encrypting certain messages (since 2.7.5x)

### Generating SSL key and certificate

To generate all stuff required for SSL, you'll need `openssl` tool installed.

- On *nix, use `misc/get_cert.sh` script
- On Windows, TODO

Place generated files into the `keys` subdirectory or the key directory you specified in the config file.

### Generating initial traffic encryption key

To get the traffic encryption key, there're many possible ways, but the easiest one would be to generate them using
[Ec2b tool](https://github.com/Jasuf/Ec2b). Move `Ec2bSeed.bin` into `keys/master.ec2b` and `Ec2bKey.bin` into `keys/master.key`.

### RSA keys

Since 2.7.5x, game is using RSA to encrypt and sign certain pieces of data. Because as of yet it's impossible to recover RSA-2048 
private keys from public ones, if you want to run game off of an unofficial server, you have following options:

- Grabbing official RSA keys and patching the game to make it accept invalid signature. This method allows playing on both official and
  unofficial servers without any further tricks, but might be detected by AC and lead to ban on official servers.
- Generate new RSA keys and create patched `global-metadata.dat` file with new keys. This patched file will allow you to only play 
  on unofficial servers, but with the swap of `global-metadata.dat` file you'll be able to connect to official servers too without 
  any possibility of ban.

Whatever method you choose, place RSA keys inside `keys/RsaConfig.json` file.

## Compiling

Just plain and simple `cargo build`.

## Redirecting the game's traffic to the server

The simplest method is by modifying the `hosts` file. Copy the contents from the provided file into your system-wide one.
Note that you'll need to comment those lines as soon as you'll want to play on the official servers or access official
resources (like web events or daily login rewards).

## Starting the server

Just `cargo run` but with a caveat. By default server listens on privileged ports (80, 443), so it needs permissions for that.

- On Windows, UAC prompt should automatically pop up and ask you to elevate server's priviledges. If it's not happening, run the server's
  executable as admin.
- On *nix, you'll need to grant the server the specific capability. You can do it by running `sudo setcap 'cap_net_bind_service=+ep' ./target/debug/Sapozhok`. **Please don't run the server as root!**

## Configuration

Some of server's properties are configurable inside `dispatch_config.ini` file. Have a look if you want to move your keys directory or
make server to listen on another ports.
