# Persistent caching Rust key-value storage

## Features
- Multithreaded TCP server
- Persistency
- Caching least recently used values with memcached
- Transactions support
- Sharding (in progress)

## Building
1. Configure the environmental variables in `config.toml`.
2. Use `cargo build` to build the executable.

## API

> GET:
> ```
> get <key>
> ```
Gets the value by key.

Returns `value` if key exists, `None` else.

Possible errors: memcached connection/storage errors, input-output error.

> SET:
> ```
> set <key> <value>
> ```
Sets the value by the key.

Returns `OK` if value has been set, describes an `Error` else.

Possible errors: memcached connection/storage errors, input-output error.

> DELETE:
> ```
> del <key>
> ```
Deletes the key and its' value if exists.

Returns `OK` if value has been set, describes an `Error` else.

Possible errors: memcached connection/storage errors, input-output error.

> INCREMENT:
> ```
> incr <key> <value>
> ```
Adds value to the entry with the given key.

Returns `OK` if value has been updated, describes an `Error` else.

Possible errors: memcached connection/storage errors, input-output error.

> DECREMENT:
> ```
> decr <key> <value>
> ```
Subtracts value from the entry with the given key.

Returns `OK` if value has been updated, describes an `Error` else.

Possible errors: memcached connection/storage errors, input-output error.

> CLEAR:
> ```
> clear
> ```

Clears the storage **and flushes the cache.**

Returns `OK` if value has been updated, describes an `Error` else.

Possible errors: memcached connection/storage errors, input-output error.

> TRANSACTIONS:
> ```
> transact:
> <op_1>
> <op_2>
> ...
> <op_n>
> ```
Transacts all the operations. If any error occures, **flushes the cache** and **rolls back** to previous commit.

Returns each operations' response one by one.

Possible errors: memcached connection/storage errors, input-output error.

> ROLLBACK:
> ```
> rolback <key> <commit>
> ```
Lets you see the value with `key` at `n`'th commit starting with `0` from the current. This being said that commits are enumerated starting from the last one.

Returns `value`.

Possible errors: memcached connection/storage errors, input-output error.

> HELP:
> ```
> help
> ```
Lets you see the list of available commands with subtle descriptions.