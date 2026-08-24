# Coblox P2P wire protocol

This document defines connectivity and the complete Coblox v0 application
message catalog carried over authenticated libp2p connections.

## Network stack

Coblox nodes use libp2p. The v0 interoperability baseline is:

- QUIC-v1 over UDP as the preferred encrypted, multiplexed transport;
- TCP with Noise and Yamux as the required fallback;
- multistream-select for application protocol negotiation;
- Identify (`/ipfs/id/1.0.0` and push), Ping, and Kademlia DHT for WAN peer
  routing; mDNS for optional same-LAN discovery;
- AutoNAT v1 (`/libp2p/autonat/1.0.0`) for reachability classification,
  Circuit Relay v2 for the relayed fallback, and DCUtR (`/libp2p/dcutr`) for
  attempts to upgrade a relayed connection to a direct TCP/QUIC connection;
- GossipSub 1.1 or later for authenticated block and evidence announcements.

AutoNAT v2 remains an active libp2p working draft and is an optional negotiated
optimization, not the v0 interoperability baseline. A node MUST retain a relay
connection when hole punching fails: NAT traversal improves reachability but
does not guarantee a direct path. Relay service is opt-in for publicly reachable
nodes and MUST apply reservation, byte, duration, and connection limits.

**Choice rationale.** QUIC was preferred over TCP for connection migration and
fewer handshakes on mobile networks. WebSocket was not selected for native v0
nodes because it adds HTTP/TLS deployment constraints; it can be added under a
new negotiated transport profile. The behavior above follows the current
[libp2p specifications](https://github.com/libp2p/specs), including
[Circuit Relay v2](https://github.com/libp2p/specs/blob/master/relay/circuit-v2.md)
and [DCUtR](https://github.com/libp2p/specs/blob/master/relay/DCUtR.md).

## Discovery

Bootstrap distributions provide at least three seed multiaddresses from
independent failure domains. Nodes:

1. discover LAN peers with mDNS service `_p2p._udp.local` when enabled;
2. connect to seeds and run Identify;
3. join the Kademlia DHT namespace whose genesis-derived key is
   `SHA-256("coblox-dht-v0\0" || genesis_block_id)`;
4. advertise the rendezvous namespace `/coblox/<network_id>/v0` when a
   rendezvous service is configured;
5. discard addresses whose authenticated Peer ID does not match Identify.

Private, loopback, and link-local addresses learned from untrusted WAN peers
MUST NOT be dialed unless they are valid for an existing local connection
scope. Nodes SHOULD randomize reconnect backoff and bound address-book entries.

## Coblox protocol IDs and topics

| Purpose | Identifier | Pattern |
| --- | --- | --- |
| identity enrollment | `/coblox/enrollment/0.1.0` | request/response |
| availability/storage/compute challenge | `/coblox/challenge/0.1.0` | request/response |
| ledger status and synchronization | `/coblox/ledger-sync/0.1.0` | request/response |
| block announcements | `/coblox/<network_id>/blocks/0.1` | GossipSub |
| challenge-evidence announcements | `/coblox/<network_id>/evidence/0.1` | GossipSub |

The enrollment stream accepts unauthenticated transport peers; all other
Coblox protocols require a valid enrollment certificate. Topic names are exact
UTF-8 strings. A node MUST NOT bridge messages between network IDs.

## Framing

Request/response streams carry repeated frames:

```text
unsigned-varint payload_length || payload_length bytes of JCS JSON
```

The varint is the minimal unsigned LEB128 encoding and is limited to five
bytes. A zero-length frame, non-minimal varint, oversized length, truncated
payload, invalid UTF-8, or non-canonical JSON closes the stream. The limits in
[README.md](README.md#security-and-resource-limits) apply before allocation.
GossipSub message data is exactly one `SignedEnvelope` without this length
prefix because GossipSub already frames messages.

## Signed envelope

Domain: `coblox-wire-envelope-v0`. The envelope signature covers the object
with `signature` removed using the global chain-bound signature procedure.
`payload` is an object, never an embedded JSON string.

```text
SignedEnvelope = {
  "schema_version": "0.1",
  "network_id": string,
  "message_type": enum,
  "message_id": sha256-string,
  "sender_node_id": string,
  "created_at_ms": u64-string,
  "expires_at_ms": u64-string,
  "nonce": base64url(16 bytes),
  "payload": object,
  "signature": base64url(64 bytes)
}
```

`message_id` is SHA-256 of `"coblox-message-id-v0\0" || chain_id_32` plus JCS
of the envelope without `message_id` and `signature`. Receivers recompute it,
reject an expiry before
creation, and require `expires_at_ms - created_at_ms <= max_envelope_validity_ms`
from the active signed consensus parameters. They cache message IDs and
`(sender_node_id, nonce)` until expiry. The cache has protocol caps
`replay_cache_entries_global` and `replay_cache_entries_per_peer`; an insertion
that would exceed either cap rejects the new envelope as `rate_limited` and
MUST NOT evict a still-live entry. Clock rollback, unavailable durable cache,
or loss of cache integrity fails closed for protected protocols. Either
duplicate is a replay. GossipSub's message-ID function MUST use this verified ID.

The complete v0 enum is `enrollment_request`, `enrollment_response`,
`challenge_request`, `challenge_response`, `ledger_status_request`,
`ledger_status_response`, `ledger_range_request`, `ledger_range_response`,
`balance_proof_request`, `balance_proof_response`, `block_announcement`, and
`challenge_evidence_announcement`. Any other value is `unsupported_version`.

Canonical serialized example:

```json
{"created_at_ms":"1787654410000","expires_at_ms":"1787654470000","message_id":"sha256:56d2aa0cd4c2ff0b06c47b478b6bfc2dff88b2c162c6cff1e33f9bf3284c7308","message_type":"ledger_status_request","network_id":"coblox-devnet-0","nonce":"AAECAwQFBgcICQoLDA0ODw","payload":{"finalized_height":"41","want_validator_set":true},"schema_version":"0.1","sender_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}
```

## Message catalog

The `message_type` value determines the exact payload schema. No other custom
message types exist in v0.

### `enrollment_request` / `enrollment_response`

`enrollment_request` payload is the `EnrollmentRequest` defined in
[identity.md](identity.md#enrollment-request). Because the sender is not yet
enrolled, its envelope `sender_node_id` and envelope signature are checked
against the request key, not a certificate.

```text
EnrollmentResponse = {
  "enrollment_request_hash": sha256-string,
  "status": "accepted" | "pending" | "rejected",
  "certificate": EnrollmentCertificate,   // required only when accepted
  "error_code": enum                      // required only when rejected
}
```

Allowed error codes are `invalid_request`, `invalid_pow`, `stale_parameters`,
`duplicate_identity`, `rate_limited`, and `internal_unavailable`. The response
hash MUST equal the registry `enrollment_request_hash` of the received request.
The response MUST NOT echo the proof or expose validation internals.

### `challenge_request`

```text
{
  "challenge_id": sha256-string,
  "kind": "availability" | "storage" | "compute",
  "subject_node_id": string,
  "issued_at_ms": u64-string,
  "deadline_ms": u64-string,
  "randomness": base64url(32 bytes),
  "assignment": AvailabilityAssignment | StorageAssignment | ComputeAssignment
}
```

```text
AvailabilityAssignment = {"response_bytes": u64-string}
StorageAssignment = {
  "object_id": sha256-string,
  "chunk_index": u64-string,
  "chunk_length": u64-string,
  "expected_root": sha256-string
}
ComputeAssignment = {
  "app_id": sha256-string,
  "module_hash": sha256-string,
  "input_hash": sha256-string,
  "input": base64url(bytes),
  "fuel_limit": u64-string
}
```

The `challenge_id` is `request_hash` from the hash registry over the JCS request
without `challenge_id`. Only
an assigned validator or ledger-selected auditor may issue a challenge.
`randomness` MUST derive from finalized consensus randomness plus an issuer
secret committed before subject selection; a subject-chosen nonce is invalid.

Canonical availability example:

```json
{"assignment":{"response_bytes":"32"},"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","deadline_ms":"1787654420000","issued_at_ms":"1787654415000","kind":"availability","randomness":"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8","subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"}
```

### `challenge_response`

```text
{
  "challenge_id": sha256-string,
  "subject_node_id": string,
  "completed_at_ms": u64-string,
  "result": AvailabilityResult | StorageResult | ComputeResult,
  "subject_signature": base64url(64 bytes)
}
AvailabilityResult = {"kind":"availability", "response":base64url(bytes)}
StorageResult = {
  "kind":"storage", "chunk":base64url(bytes),
  "leaf_index":u64-string, "merkle_siblings":[sha256-string]
}
ComputeResult = {
  "kind":"compute", "output":base64url(bytes), "output_hash":sha256-string,
  "fuel_consumed":u64-string
}
```

The subject signature domain is `coblox-challenge-response-v0` and uses the
global chain binding. `response_hash` is the registry hash of the response with
`subject_signature` removed. Storage proof
verification reconstructs `expected_root`; compute verification hashes the
exact output and MUST reject fuel above the assignment limit. Responses after
the deadline remain auditable but are not reward-eligible.

Canonical response example:

```json
{"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","completed_at_ms":"1787654416000","result":{"kind":"availability","response":"MzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzM"},"subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","subject_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}
```

### `ledger_status_request` / `ledger_status_response`

```text
LedgerStatusRequest = {
  "finalized_height":u64-string,
  "want_validator_set":boolean,
  "document_hashes":[sha256-string]
}
LedgerStatusResponse = {
  "genesis_block_id":sha256-string,
  "finalized_header":BlockHeader,
  "quorum_certificate":QuorumCertificate,
  "validator_set":ValidatorSet,            // omitted unless requested/needed
  "protocol_documents":[SignedProtocolDocument]
}
```

`document_hashes` are unique, bytewise sorted, and capped at 32. The response
includes each requested active or historical signed document, capped at 2 MiB;
unknown hashes return `not_found`. This is the canonical retrievability path for
enrollment parameters, reward policies, hosting rate cards, and consensus
parameters committed by ledger hashes.

### `ledger_range_request` / `ledger_range_response`

```text
LedgerRangeRequest = {
  "from_height":u64-string, "max_blocks":u64-string,
  "expected_previous_block_id":sha256-string
}
LedgerRangeResponse = {
  "blocks":[Block], "more":boolean,
  "next_height":u64-string
}
```

`max_blocks` is 1–128. Blocks are contiguous, finalized, and capped by the 8 MiB
response limit. The requester validates every link and certificate before
advancing its trusted height; a response is never trusted because it arrived
over an authenticated peer connection.

Canonical range request example:

```json
{"expected_previous_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af","from_height":"42","max_blocks":"64"}
```

### `balance_proof_request` / `balance_proof_response`

```text
BalanceProofRequest = {
  "account_kind":"node"|"app", "subject_id":string,
  "at_height":u64-string
}
BalanceProofResponse = {
  "header":BlockHeader,
  "quorum_certificate":QuorumCertificate,
  "validator_set":ValidatorSet,
  "proof":AccountProof
}
```

Servers MUST return only finalized heights and MUST return a header whose
height equals `at_height` exactly; substituting an older or merely available
height is invalid. A client rejects any height below its persisted trusted
height, requires the tip/checkpoint freshness bounds from active consensus
parameters, and queries independently operated enrolled peers for corroboration.
Proof validation is defined step by
step in [ledger.md](ledger.md#light-client-balance-verification).

### `block_announcement`

Gossip topic: `blocks`. Payload:

```text
{"block_id":sha256-string,"height":u64-string,"header":BlockHeader,
 "quorum_certificate":QuorumCertificate}
```

The announcement is a hint. Receivers validate it and fetch the full block via
ledger sync; gossip is not a finality mechanism.

### `challenge_evidence_announcement`

Gossip topic: `evidence`. Payload:

```text
{"evidence_tx_id":sha256-string,"challenge_id":sha256-string,
 "subject_node_id":string,"result":"passed"|"failed"|"late"}
```

The full evidence transaction is fetched from a finalized block. Announcements
do not independently create rewards or penalties.

## Error response

Any request/response stream may return:

```json
{"error_code":"invalid_request","message_id":"sha256:56d2aa0cd4c2ff0b06c47b478b6bfc2dff88b2c162c6cff1e33f9bf3284c7308","retry_after_ms":"0"}
```

Codes are `invalid_request`, `unauthorized`, `not_found`, `conflict`,
`rate_limited`, `unsupported_version`, `too_large`, and
`internal_unavailable`. Human text is intentionally absent from the wire.

## Gossip validation and backpressure

Nodes validate topic/network, canonical envelope, size, ID, signature,
certificate, expiry, replay cache, and payload schema before accepting gossip.
Block announcements additionally require a valid quorum certificate. Evidence
announcements require an enrolled validator sender but are treated as hints.
Application objects MUST NOT use libp2p's anonymous author mode.

Per-peer queues are bounded. When full, nodes drop duplicate/low-priority hints
before finalized headers or direct responses. Challenge request and ledger sync
streams use timeouts and explicit concurrency limits; no untrusted peer can
cause an unbounded task, allocation, or retained response.
