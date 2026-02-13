# isac
> a modular and hackable system configurator for selfhosted infrastructure

`isac` (infrastructure setup as config) is a somewhat simpler [ansible](https://github.com/ansible/ansible): it allows to automatically configure fresh machines from a snapshot, generated from a single toml file

`isac` aims to setup software directly, without using containers or virtualization. to make this possible with all weird different possible configurations possible, `isac` is very hackable in its design.

`isac` is born mostly to generate "module" snapshots of all services installed on a single machine, and be able to selectively restore them on multiple different machines, effectively subdividing a monolithic system. it's suitable also for automatically configuring throwaway vps-es without going into more complex solutions

> [!IMPORTANT]
> `isac` is currently very tailored to my needs, feel free to open issues and request features

## usage
> [!CAUTION]
> `isac` currently relies on `pacman`, `systemd` and `usermod`, as no other providers are implemented

`isac` works around a config file, usually `isac.toml`, which defines a snapshot root

it's possible to point to an arbitrary config with `-c` flag, but by default current directory will be used as snapshot root

the config file defines "modules", which are individual services:

```toml
[system]
services = "/etc/systemd/system"         # where service units will be placed, and snapshotted from
configs = "/etc"                         # default path for configs

[mod.email]
dependencies = ["dovecot", "postfix"]    # install packages with system package manager
services = ["postfix"]                   # copy postfix.service if exists, and start services
configs = ["dovecot"]                    # snapshot and restore the whole /etc/dovecot directory

[mod.db]
dependencies = ["postgresql"]
data.dumper = "pg_dumpall > pg-dump.sql"         # arbitrary script for dumping data
data.loader = """
psql -f pg-dump.sql -d postgres
"""                                              # and for loading, can be multiline!

[mod.my-service]
dependencies = ["rustup", "git"]
services = ["my-service"]
user.name = "myservice"                 # create new user
user.system = true                      # make it a system user
user.groups = ["wheel"]                 # add it to groups
compile = """
rustup update
git clone https://github.com/alemidev/isac
cd isac
cargo build --release
mv target/release/isac /bin
"""                                     # compile your software with custom script
```

once you have a config file, you can `isac snapshot` and `isac restore` your system
