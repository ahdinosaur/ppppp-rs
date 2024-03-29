# ppppp-rs

`%/z/HeJwoaU0g/mep39laPu3RVF8Zo5coGNvAWUrnGUA=.sha256`

![](https://i.kym-cdn.com/photos/images/original/002/205/488/707.jpg)

[![ci status](https://img.shields.io/github/actions/workflow/status/ahdinosaur/ppppp-rs/ci.yml?style=flat-square)](https://github.com/ahdinosaur/ppppp-rs/actions/workflows/ci.yml?query=branch%3Amain)

## specs

- [Scuttlebutt Protocol Guide](https://ssbc.github.io/scuttlebutt-protocol-guide/)
- [PPPPP Msg V3](https://github.com/staltz/ppppp-db/blob/master/protospec.md)
- [PPPPP Tangle Sync](https://github.com/staltz/ppppp-tangle-sync/blob/master/protospec.md)
- [PPPPP Tangle Set](https://github.com/staltz/ppppp-set/blob/master/protospec.md)
- [PPPPP Tangle Dict](https://github.com/staltz/ppppp-dict/blob/master/protospec.md)

## modules sketch

- 🔵: done
- 🟢: work in progress
- 🔴: must have for [minimum viable prototype](https://github.com/ahdinosaur/ppppp-rs/issues/1)
- 🟠: expected for a complete platform
- 🟡: nice to have

> see also:
>
> - sunrise sketch: [`%Rkenf2YtjouPfOOzrHpzKPn150t3l8oJq+5dZ9/AXWo=.sha256`](https://viewer.scuttlebot.io/%25Rkenf2YtjouPfOOzrHpzKPn150t3l8oJq%2B5dZ9%2FAXWo%3D.sha256)
> - [pietguerson/ssb-server-system-design](https://github.com/pietgeursen/ssb-server-system-design)

### base

- 🔵 [`ppppp-base58`](./base58) [![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://ahdinosaur.github.io/ppppp-rs/ppppp_base58/index.html) :  base58 with the ppppp alphabet
- 🟢 [`ppppp-bytes`](./bytes) [![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://ahdinosaur.github.io/ppppp-rs/ppppp_bytes/index.html) : traits from and to bytes, with built-in base58 ser/de
- 🟢 [`ppppp-crypto`](./crypto) [![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://ahdinosaur.github.io/ppppp-rs/ppppp_crypto/index.html) : primitive crypto types and operations used by ppppp
  - [sunrise-choir/ssb-crypto](https://github.com/sunrise-choir/ssb-crypto)
- 🟠 `ppppp-service`: rpc service trait (able to be exposed externally over muxrpc or internally in memory)
  - [n0-computer/quic-rpc](https://github.com/n0-computer/quic-rpc)
  - [google/tarpc](https://github.com/google/tarpc)

### formats

- 🟢 [`ppppp-msg`](./msg) [![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://ahdinosaur.github.io/ppppp-rs/ppppp_msg/index.html) : ppppp messages and tangles
  - [staltz/ppppp-db](https://github.com/staltz/ppppp-db)
  - [sunrise-choir/legacy-msg-data](https://github.com/sunrise-choir/legacy-msg-data)
  - [sunrise-choir/ssb-legacy-msg](https://github.com/sunrise-choir/ssb-legacy-msg)
  - [sunrise-choir/ssb-casual-sort](https://github.com/sunrise-choir/ssb-casual-sort)
  - [sunrise-choir/ssb-validate](https://github.com/sunrise-choir/ssb-validate)
  - [sunrise-choir/ssb-verify-signatures](https://github.com/sunrise-choir/ssb-verify-signatures)
- 🟠 `ppppp-threads`: message thread helpers for ppppp
  - [ssbc/ssb-threads](https://github.com/ssbc/ssb-threads)
- 🟠 `ppppp-set`: set data structure for ppppp
  - [staltz/ppppp-set](https://github.com/staltz/ppppp-set)
- 🟠 `ppppp-dict`: dictionary data structure for ppppp
  - [staltz/ppppp-dict](https://github.com/staltz/ppppp-dict)
- 🟠 `ppppp-blob`: binary data format for ppppp

### stores

- 🟠 `ppppp-key-store`: key store for ppppp: read keys from a file, write keys to a file
  - [staltz/ppppp-keypair](https://github.com/staltz/ppppp-keypair)
  - [sunrise-choir/ssb-keyfile](https://github.com/sunrise-choir/ssb-keyfile)
  - [ssbc/ssb-keys](https://github.com/ssbc/ssb-keys)
- 🔴 `ppppp-msg-log`: append-only log storage with pruning for ppppp
  - [ssbc/async-append-only-log](https://github.com/ssbc/async-append-only-log)
  - [sunrise-choir/flumedb-rs](https://github.com/sunrise-choir/flumedb-rs)
- 🔴 `ppppp-msg-store`: materialized data views for ppppp
  - [staltz/ppppp-db](https://github.com/staltz/ppppp-db)
  - [sunrise-choir/ssb-db](https://github.com/sunrise-choir/ssb-db)
  - [ssbc/jitdb](https://github.com/ssbc/jitdb)
- 🟠 `ppppp-blob-store`: binary data stores for ppppp
  - [ssbc/ssb-blobs](https://github.com/ssbc/ssb-blobs)
- 🟠 `ppppp-db`: database for ppppp
  - [ssbc/ssb-db2](https://github.com/ssbc/ssb-db2)

### rpc

- 🔴 `ppppp-packetstream`: multiplexed packet protocol for ppppp
  - [sunrise-choir/ssb-packetstream](https://github.com/sunrise-choir/ssb-packetstream)
  - [sunrise-choir/packet-stream-rs](https://github.com/sunrise-choir/packet-stream-rs)
  - [sunrise-choir/packet-stream-codec-rs](https://github.com/sunrise-choir/packet-stream-codec-rs)
- 🔴 `ppppp-muxrpc`: multiplexed rpc for ppppp
  - [sunrise-choir/muxrpc-rs](https://github.com/sunrise-choir/muxrpc-rs)
  - [ssbc/muxrpc](https://github.com/ssbc/muxrpc)
- 🟠 `ppppp-rpc`: type manifest for ppppp rpc interface
  - [sunrise-choir/ssb-rpc-rs](https://github.com/sunrise-choir/ssb-rpc-rs)
- 🟠 `ppppp-server`: rpc server for pppppp
- 🟠 `ppppp-client`: rpc client for pppppp
  - [sunrise-choir/ssb-client-rs](https://github.com/sunrise-choir/ssb-client-rs)

### handshake

- 🔴 `ppppp-boxstream`: encrypted box stream protocol for ppppp
  - [sunrise-choir/ssb-boxstream](https://github.com/sunrise-choir/ssb-boxstream)
  - [sunrise-choir/box-stream-rs](https://github.com/sunrise-choir/box-stream-rs)
  - [sunrise-choir/box-stream-c](https://github.com/sunrise-choir/box-stream-c)
- 🔴 `ppppp-shse`: secret handshake extended protocol for ppppp
  - [staltz/secret-handshake-ext](https://github.com/staltz/secret-handshake-ext)
  - [sunrise-choir/ssb-handshake](https://github.com/sunrise-choir/ssb-handshake)
  - [sunrise-choir/secret-handshake-rs](https://github.com/sunrise-choir/secret-handshake-rs)
  - [sunrise-choir/shs1-c](https://github.com/sunrise-choir/shs1-c)
- 🔴 `ppppp-caps`
  - [staltz/ppppp-caps](https://github.com/staltz/ppppp-caps)

### discovery

- 🟠 `ppppp-promise`: tokens in ppppp that authorize others to gain something
  - [staltz/ppppp-promise](https://github.com/staltz/ppppp-promise)
- 🟠 `ppppp-invite`: invite code generator and parser for ppppp
  - [staltz/ppppp-invite](https://github.com/staltz/ppppp-invite)
- 🟠 `ppppp-hub`: server to cross-connect ppppp peers via tunnel
  - [staltz/ppppp-hub](https://github.com/staltz/ppppp-hub)
- 🟠 `ppppp-connect`: discover, remember, query, stage, establish, and maintain ppppp connections
  - [ssbc/ssb-conn](https://github.com/ssbc/ssb-conn)
- 🔴 `ppppp-connect-hub`: discover and connect to ppppp peers over hub server
  - [staltz/ppppp-hub-client](https://github.com/staltz/ppppp-hub-client)
- 🟡 `ppppp-connect-lan`: discover and connect to ppppp peers on same LAN
  - [ssbc/ssb-lan](https://github.com/ssbc/ssb-lan)

### replication

- 🔴 `ppppp-sync`: replicate in ppppp using Kleppman's hash graph sync
  - [staltz/ppppp-tangle-sync](https://github.com/staltz/ppppp-tangle-sync)
- 🟡 `ppppp-sync-ebt`: replicate in ppppp using epidemic broadcast trees

### orchestration

- 🟠 `ppppp-gc`: data garbage collector for ppppp
  - [staltz/ppppp-gc](https://github.com/staltz/ppppp-gc)
- 🟠 `ppppp-goals`: track replication goals in ppppp
  - [staltz/ppppp-goals](https://github.com/staltz/ppppp-goals)
- 🟠 `ppppp-conductor`: schedule connections, replication, and pruning

### private messages / groups

🟡

### sdk

- 🟠 `ppppp-sdk`: friendly kit for developers to build ppppp apps
  - [sunrise-choir/ssb-publish](https://github.com/sunrise-choir/ssb-publish)
