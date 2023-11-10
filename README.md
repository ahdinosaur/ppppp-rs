# ppppp-rs

`%/z/HeJwoaU0g/mep39laPu3RVF8Zo5coGNvAWUrnGUA=.sha256`

![](https://i.kym-cdn.com/photos/images/original/002/205/488/707.jpg)

## specs

- [Scuttlebutt Protocol Guide](https://ssbc.github.io/scuttlebutt-protocol-guide/)
- [PPPPP Msg V3](https://github.com/staltz/ppppp-db/blob/master/protospec.md)
- [PPPPP Tangle Sync](https://github.com/staltz/ppppp-tangle-sync/blob/master/protospec.md)
- [PPPPP Tangle Set](https://github.com/staltz/ppppp-set/blob/master/protospec.md)
- [PPPPP Tangle Dict](https://github.com/staltz/ppppp-dict/blob/master/protospec.md)

## modules sketch

\* [MVP](https://github.com/ahdinosaur/ppppp-rs/issues/1)

(see also [pietguerson/ssb-server-system-design](https://github.com/pietgeursen/ssb-server-system-design))

### base

- \*ppppp-base58
- \*ppppp-crypto
  - [sunrise-choir/ssb-crypto](https://github.com/sunrise-choir/ssb-crypto)

### data

- \*ppppp-msg
  - [staltz/ppppp-db](https://github.com/staltz/ppppp-db)
  - [sunrise-choir/legacy-msg-data](https://github.com/sunrise-choir/legacy-msg-data)
  - [sunrise-choir/ssb-legacy-msg](https://github.com/sunrise-choir/ssb-legacy-msg)
- \*ppppp-validate
  - [staltz/ppppp-db](https://github.com/staltz/ppppp-db)
  - [sunrise-choir/ssb-validate](https://github.com/sunrise-choir/ssb-validate)
  - [sunrise-choir/ssb-verify-signatures](https://github.com/sunrise-choir/ssb-verify-signatures)
- \*ppppp-tangle
  - [staltz/ppppp-db](https://github.com/staltz/ppppp-db)
  - [sunrise-choir/ssb-casual-sort](https://github.com/sunrise-choir/ssb-casual-sort)
- ppppp-threads
  - [ssbc/ssb-threads](https://github.com/ssbc/ssb-threads)
- \*ppppp-account
- ppppp-set
  - [staltz/ppppp-set](https://github.com/staltz/ppppp-set)
- ppppp-dict
  - [staltz/ppppp-dict](https://github.com/staltz/ppppp-dict)
- ppppp-blobs

### data store

- \*ppppp-msg-log
  - [ssbc/async-append-only-log](https://github.com/ssbc/async-append-only-log)
  - [sunrise-choir/flumedb-rs](https://github.com/sunrise-choir/flumedb-rs)
- \*ppppp-msg-store
  - [staltz/ppppp-db](https://github.com/staltz/ppppp-db)
  - [sunrise-choir/ssb-db](https://github.com/sunrise-choir/ssb-db)
- ppppp-key-store
  - [staltz/ppppp-keypair](https://github.com/staltz/ppppp-keypair)
  - [sunrise-choir/ssb-keyfile](https://github.com/sunrise-choir/ssb-keyfile)
- ppppp-blobs-store

### rpc

- \*ppppp-packetstream
  - [sunrise-choir/ssb-packetstream](https://github.com/sunrise-choir/ssb-packetstream)
  - [sunrise-choir/packet-stream-rs](https://github.com/sunrise-choir/packet-stream-rs)
  - [sunrise-choir/packet-stream-codec-rs](https://github.com/sunrise-choir/packet-stream-codec-rs)
- \*ppppp-muxrpc
  - [sunrise-choir/muxrpc-rs](https://github.com/sunrise-choir/muxrpc-rs)
- ppppp-rpc
  - [sunrise-choir/ssb-rpc-rs](https://github.com/sunrise-choir/ssb-rpc-rs)
- ppppp-server
- ppppp-client
  - [sunrise-choir/ssb-client-rs](https://github.com/sunrise-choir/ssb-client-rs)

### handshake

- \*ppppp-boxstream
  - [sunrise-choir/ssb-boxstream](https://github.com/sunrise-choir/ssb-boxstream)
  - [sunrise-choir/box-stream-rs](https://github.com/sunrise-choir/box-stream-rs)
  - [sunrise-choir/box-stream-c](https://github.com/sunrise-choir/box-stream-c)
- \*ppppp-shse
  - [staltz/secret-handshake-ext](https://github.com/staltz/secret-handshake-ext)
  - [sunrise-choir/ssb-handshake](https://github.com/sunrise-choir/ssb-handshake)
  - [sunrise-choir/secret-handshake-rs](https://github.com/sunrise-choir/secret-handshake-rs)
  - [sunrise-choir/shs1-c](https://github.com/sunrise-choir/shs1-c)
- \*ppppp-caps
  - [staltz/ppppp-caps](https://github.com/staltz/ppppp-caps)

### discovery

- ppppp-promise
  - [staltz/ppppp-promise](https://github.com/staltz/ppppp-promise)
- ppppp-invite
  - [staltz/ppppp-invite](https://github.com/staltz/ppppp-invite)
- \*ppppp-hub-client
  - [staltz/ppppp-hub-client](https://github.com/staltz/ppppp-hub-client)
- ppppp-hub
  - [staltz/ppppp-hub](https://github.com/staltz/ppppp-hub)

### replication

- \*ppppp-tangle-sync
  - [staltz/ppppp-tangle-sync](https://github.com/staltz/ppppp-tangle-sync)
- ppppp-garbage
  - [staltz/ppppp-gc](https://github.com/staltz/ppppp-gc)
- ppppp-goals
  - [staltz/ppppp-goals](https://github.com/staltz/ppppp-goals)
- ppppp-connections
  - [ssbc/ssb-conn](https://github.com/ssbc/ssb-conn)
- ppppp-conductor

### private messages / groups

### sdk

- ppppp-sdk
  - [sunrise-choir/ssb-publish](https://github.com/sunrise-choir/ssb-publish)
