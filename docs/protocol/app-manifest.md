# WASM app manifest and distribution package

This document defines the signed app contract, sandbox request, pricing data,
and byte-exact distribution container accepted by Coblox runtimes.

## Overview

A Coblox app is one WebAssembly module plus a signed canonical manifest in a
deterministic `.cobloxapp` container. Installing a package does not grant its
requested capabilities: the host validates the package and then applies one of
two consent paths — an explicit operator grant for a voluntary installation, or
evaluation against a declared machine-readable policy for an assignment the
protocol made. Neither path ever grants by default.

App identity is content-addressed and chain-bound:

```text
app_id = SHA-256("coblox-app-id-v0\0" || chain_id_32 || JCS(unsigned_manifest))
```

`unsigned_manifest` is the manifest without `publisher_signature`. Changing
code, capabilities, deployment requirements, limits, pricing, or metadata
creates a new app ID.

## Manifest schema

The manifest is UTF-8 JCS JSON and MUST be at most 64 KiB.

```text
AppManifest = {
  "schema_version":"0.1",
  "name":string,
  "version":semver-string,
  "description":string,
  "publisher":{
    "node_id":string,
    "public_key":base64url(32 bytes)
  },
  "runtime":{
    "format":"wasm",
    "abi":"wasi_snapshot_preview1",
    "entrypoint":string,
    "deterministic":boolean
  },
  "module":{
    "sha256":sha256-string,
    "size_bytes":u64-string
  },
  "deployment":{
    "desired_replicas":u64-string
  },
  "capabilities":[Capability],
  "resources":ResourceLimits,
  "pricing":Pricing,
  "distribution":{
    "media_type":"application/vnd.coblox.app-v0",
    "package_format":"cobloxapp-v0"
  },
  "publisher_signature":base64url(64 bytes)
}
```

Strings are UTF-8 and bounded: name 1–80 bytes, semantic version 1–64,
description 0–1024, and entrypoint 1–128. The publisher key MUST derive the
publisher node ID and have a finalized, unrevoked enrollment certificate.
Publisher signature domain is `coblox-app-manifest-v0` and follows the global
chain-bound procedure.

The module MUST be a valid core WebAssembly module, contain no start function,
export the declared entrypoint, and import only functions allowed by its ABI and
granted capabilities. The host MUST validate this before compilation.

`deployment.desired_replicas` is REQUIRED and ranges from 1 through 1,024. It
states the publisher's desired concurrently assigned replicas; resource limits
apply independently to every replica. The protocol—not the publisher—selects
eligible hosts using the active assignment policy. A publication is pending
until the desired count can be assigned; it MUST NOT silently weaken the signed
replication request. Host refusal and network block policy may cause later
reassignment without changing the manifest.

## Capabilities

Capabilities are unique and sorted by `name`. Absence means denial.

```text
Capability =
  {"name":"clock_monotonic"}
| {"name":"random_deterministic"}
| {"name":"storage_app", "max_bytes":u64-string}
| {"name":"http_fetch", "https_origins":[string],
   "max_request_bytes":u64-string, "max_response_bytes":u64-string}
```

- `clock_monotonic` exposes an invocation-relative monotonic counter, never
  wall-clock or timezone data.
- `random_deterministic` exposes a host seed committed in the task input. The
  same input and seed MUST reproduce the same bytes.
- `storage_app` exposes only a per-app virtual directory; no host path, device,
  symlink escape, or other app's storage is reachable.
- `http_fetch` allows HTTPS only, exact normalized origins only, no raw sockets,
  DNS rebinding to loopback/private/link-local ranges, redirects outside the
  allowlist, ambient credentials, or host proxy inheritance.

For every initial URL and every redirect hop, the host normalizes and checks the
origin, resolves DNS exactly once, validates **every** returned address against
the forbidden loopback, private, link-local, multicast, unspecified, and
metadata-service ranges for IPv4 and IPv6, then connects to one of those pinned
addresses while retaining the original hostname for TLS SNI and certificate
verification. The connector MUST NOT perform another name resolution. A DNS
answer containing any forbidden address rejects the whole request; redirects
repeat the complete resolve/validate/pin procedure and are capped by the host
policy. IPv4-mapped IPv6 is normalized before classification.

DNS time, connection time, TLS time, redirect bodies, request bytes, and
response bytes all consume the invocation wall-time and byte budgets; exhaustion
aborts the host call. Conformance fixtures: (1) a name resolving to one public
and one `127.0.0.1` address rejects; (2) a public first hop redirecting to a name
that resolves to `169.254.169.254` rejects before connection; (3) changing DNS
after the accepted answer cannot change the pinned destination; (4) a redirect
loop exhausts the redirect/time budget and traps deterministically.

An app with `runtime.deterministic: true` MUST NOT request `http_fetch` or
persistent `storage_app`. Its output may depend only on module bytes, canonical
input, deterministic random seed, and declared resource limits. Such apps are
eligible for sampled compute re-execution. Non-deterministic apps may be hosted
but compute results cannot generate `work_kind: "compute"` rewards in v0.

**Choice rationale.** Capability allowlists were selected instead of ambient
WASI access. Raw network and host filesystem access are excluded because they
break sandbox isolation and deterministic verification.

## Host acceptance policy

`deployment.desired_replicas` means the protocol, not the publisher, picks the
hosts. Those hosts are headless daemons and backgrounded Android nodes: there is
no operator in front of a screen to approve anything. An interactive grant is
therefore not merely inconvenient in that case, it is unimplementable, and the
only thing an implementation could do with the rule "ask the operator" is grant
whatever the manifest asks. That would hand a publisher N third-party machines
making HTTPS requests to origins it controls, inside a sandbox that is
technically intact while the legal and reputational exposure sits with whoever
lent the machine.

Every host that accepts protocol assignments MUST therefore declare a policy
that a machine can evaluate without a human:

```text
HostAcceptancePolicy = {
  "schema_version":"0.1",
  "accept_protocol_assignments":boolean,
  "allowed_capabilities":[string],
  "http_fetch_allowed_origins":[string],
  "require_deterministic":boolean,
  "max_resources":ResourceLimits,
  "max_persistent_storage_bytes":u64-string,
  "max_assigned_apps":u64-string
}
```

Evaluation is total, deterministic, and closed: absence denies. An assignment is
inside the policy only when `accept_protocol_assignments` is true, every
requested capability name appears in `allowed_capabilities`, every `http_fetch`
origin appears in `http_fetch_allowed_origins` after the same normalization the
runtime uses, `require_deterministic` is satisfied, every field of the manifest's
`ResourceLimits` is at most the corresponding field of `max_resources`, declared
storage fits the storage ceiling, and the host is below `max_assigned_apps`.

Inside the policy the assignment is accepted. Outside it — in any single
respect — the assignment is **refused**, and the refusal is the outcome the
protocol reassignment path and the per-node refusal list of [ADR-006] consume.
There is no third outcome: a host MUST NOT grant a capability, raise a ceiling,
partially accept, or defer to an operator prompt that nobody will answer. A
missing or unparseable policy means `accept_protocol_assignments` is false.

The interactive grant of the installation steps below applies only to a
**voluntary installation by a user on their own device**, never to a protocol
assignment. The two paths are disjoint and an implementation MUST NOT
substitute one for the other.

Independently of which path granted it, an operator MUST be able to see at any
time which `app_id`s the node currently hosts, with which capabilities and
ceilings, and to withdraw consent — which produces a refusal and a
reassignment, never a silent continuation.

## Resource limits

```text
ResourceLimits = {
  "memory_bytes":u64-string,
  "table_elements":u64-string,
  "fuel_per_invocation":u64-string,
  "wall_time_ms":u64-string,
  "output_bytes":u64-string,
  "concurrent_invocations":u64-string,
  "persistent_storage_bytes":u64-string
}
```

All fields are required, positive except persistent storage which may be zero,
and are maxima rather than reservations. v0 hosts MUST reject—not silently
clamp—a manifest above their advertised capacity. Hard protocol ceilings are:
4 GiB memory, 1,000,000 table elements, 10^12 fuel, 300,000 ms wall time,
64 MiB output, 1,024 concurrent invocations, 1 TiB persistent storage, 512 MiB
module, and 513 MiB package. Deployment policy SHOULD set much lower defaults.

The runtime interrupts on exhausted fuel or deadline and discards output beyond
the cap. Memory growth, table growth, host-call buffers, and storage writes are
charged before allocation. A trap never earns compute compensation.

## Pricing

Pricing is denominated only in integer microtokens and can express both ADR-005
burn flows.

```text
Pricing = {
  "currency":"coblox_microtoken",
  "hosting":{
    "rate_source":"protocol"
  },
  "subscription":{
    "period_ms":u64-string,
    "microtokens_per_period":u64-string
  },
  "invocation":{
    "base_microtokens":u64-string,
    "microtokens_per_million_fuel":u64-string
  }
}
```

Zero is allowed for publisher-controlled subscription and invocation components
and means that component is free. Hosting is paid from the app escrow created by
`fund_app` and creates an `app_hosting` burn authorized by consensus;
subscription creates a node-funded `app_subscription` burn.
`hosting.rate_source` MUST be the
literal `protocol`: hosting unit prices and minimum billing periods come only
from the signed validator-governed network rate card. The publisher declares
replicas and per-replica resource ceilings but cannot declare a hosting price.
The hosting burn's `pricing_hash` is the active rate-card hash; the subscription
burn's `pricing_hash` is the hash of the manifest's JCS `subscription` and
`invocation` pricing object. Invocation charges are included in the app's
subscription quote for v0 and MUST NOT create an unrecognized ledger
transaction. Validators calculate with checked integer arithmetic and round
fractional billing units upward.

Burned tokens never go directly to the publisher or host. Hosts earn separate
protocol mints only after challenge evidence proves storage/availability/compute
work. The publisher earns a separate `publisher_reward` mint whose amount is
derived from the finalized active-subscriber commitment for the reward epoch,
as specified in [ledger.md](ledger.md#mint-existence-income-work-compensation-and-publisher-reward).

## Canonical serialized manifest example

The following line is the actual serialized manifest form. Fixture signatures
and digests are structural examples, not production credentials.

```json
{"capabilities":[{"name":"clock_monotonic"},{"name":"random_deterministic"}],"deployment":{"desired_replicas":"3"},"description":"Deterministic image thumbnail worker","distribution":{"media_type":"application/vnd.coblox.app-v0","package_format":"cobloxapp-v0"},"module":{"sha256":"sha256:a5b6042890092482e58c2fb7b4d7f78eb5bc546b2258f7f831b9412e8c4175a6","size_bytes":"184320"},"name":"coblox-thumbnailer","pricing":{"currency":"coblox_microtoken","hosting":{"rate_source":"protocol"},"invocation":{"base_microtokens":"10","microtokens_per_million_fuel":"25"},"subscription":{"microtokens_per_period":"300000","period_ms":"2592000000"}},"publisher":{"node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","public_key":"11qYAYdk9J0L5Z-6hB4qMTPBSAE5nK1G0IU2n6z1V9g"},"publisher_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","resources":{"concurrent_invocations":"2","fuel_per_invocation":"50000000","memory_bytes":"134217728","output_bytes":"4194304","persistent_storage_bytes":"0","table_elements":"10000","wall_time_ms":"30000"},"runtime":{"abi":"wasi_snapshot_preview1","deterministic":true,"entrypoint":"coblox_run","format":"wasm"},"schema_version":"0.1","version":"1.2.0"}
```

## Deterministic `.cobloxapp` container

The package is a binary concatenation. There is no ZIP/TAR metadata and no
path parser.

```text
offset  size       value
0       8          ASCII "CBLXAPP" followed by 0x00
8       2          format major, u16be, value 0
10      2          format minor, u16be, value 1
12      4          manifest length, u32be
16      M          exact JCS manifest bytes
16+M    8          module length, u64be
24+M    W          exact WebAssembly module bytes
```

No trailing bytes are allowed. The manifest and module lengths MUST match the
container and manifest, and the module hash MUST match before compilation.
Package hash is SHA-256 of all package bytes. Hosts parse lengths with checked
arithmetic and enforce limits before allocating.

Serialized header example for a 1,024-byte manifest and 184,320-byte module:

```text
43 42 4c 58 41 50 50 00  00 00 00 01  00 00 04 00
[1024 manifest bytes]
00 00 00 00 00 02 d0 00
[184320 module bytes]
```

Distribution uses a content-addressed provider record keyed by package hash.
Providers MAY serve the same bytes through Coblox/libp2p fetch or HTTPS; the
receiver trusts the hash and publisher signature, not the transport. Partial
downloads MUST be stored outside the executable cache and atomically promoted
only after complete verification.

## Installation and execution verification

An implementation performs these steps in order:

1. enforce package and component size limits while streaming;
2. validate magic, version, exact lengths, and absence of trailing bytes;
3. validate canonical manifest schema and recompute the publisher node ID;
4. verify publisher enrollment at a finalized height and the manifest signature;
5. hash the module, validate WASM structure, imports, export, and ABI;
6. reject conflicting/duplicate capabilities and resource values beyond host or
   protocol limits;
7. decide consent by path: for a **voluntary user installation**, present the
   exact capability, resource, and pricing request and record an explicit
   operator grant bounded by the manifest; for a **protocol assignment**,
   evaluate the request against the declared
   [host acceptance policy](#host-acceptance-policy) and either accept it
   because it falls inside, or refuse it. Never grant by absence of an answer;
8. compile in a fresh sandbox configuration keyed by app ID and runtime version;
9. at each invocation enforce fuel, memory, time, output, storage, concurrency,
   and capability boundaries independently of guest behavior.

Updates repeat every step and never inherit additional grants. A rollback is a
separate previously verified app ID.

## Failure behavior

Invalid canonical JSON, signature, enrollment, module digest, length, WASM,
import, limit, origin, or price rejects the whole package. Unknown capability,
ABI, package version, or manifest field is `unsupported_version`, not an
ignored hint. Hosts SHOULD quarantine repeated malicious packages by hash while
retaining minimal audit metadata.

## DRAFT: WASI generation for post-v0 runtime

The v0 wire value is fixed to `wasi_snapshot_preview1` because it is the narrow
core-module ABI with mature Wasmtime support. A later negotiated manifest minor
may add either (a) WASI Preview 2 components, improving typed composition and
resource handles, or (b) a Coblox-only minimal host ABI, reducing ambient WASI
surface but increasing SDK burden. AGENT-003 owns the runtime compatibility
prototype and recommendation; the Project Lead decides the manifest-version
change. Implementations MUST NOT accept either alternative under schema 0.1.
