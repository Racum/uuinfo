


Type / Version | Random<br>or Opaque | Date / Time | Node or<br>Namespace | Sequence | Bits | Length
:--|:-:|:-:|:-:|:-:|--:|--:
**UUID-like:**        |
UUIDv1                |❌|✅|✅|✅|128|36 or 32
UUIDv2                |✅|❌|❌|❌|128|36 or 32
UUIDv3                |✅|❌|❌|❌|128|36 or 32
UUIDv4                |✅|❌|❌|❌|128|36 or 32
UUIDv5                |✅|❌|❌|❌|128|36 or 32
UUIDv6                |❌|✅|✅|✅|128|36 or 32
UUIDv7                |✅|✅|❌|❌|128|36 or 32
UUIDv8                |✅|❌|❌|❌|128|36 or 32
Nil UUID              |❌|❌|❌|❌|128|36 or 32
Max UUID              |❌|❌|❌|❌|128|36 or 32
NCS UUID              |✅|❌|❌|❌|128|36 or 32
Microsoft GUID        |✅|❌|❌|❌|128|36 or 32
ULID                  |✅|✅|❌|❌|128|26
UPID                  |✅|✅|✅|❌|128|27
Wrapped in Short UUID |-|-|-|-|128|22
Wrapped in Base64     |-|-|-|-|128|24 or 22
**Snowflake Variants:**|
Twitter               |❌|✅|✅|✅|64|Variable
Mastodon              |❌|✅|❌|✅|64|Variable
Discord               |❌|✅|✅|✅|64|Variable
Instagram             |❌|✅|✅|✅|64|Variable
LinkedIn              |❌|✅|✅|✅|64|Variable
Sony                  |❌|✅|✅|✅|64|Variable
Spaceflake            |❌|✅|✅|✅|64|Variable
**Other:**            |
MongoDB ObjectId      |✅|✅|❌|✅|96|24
Xid                   |❌|✅|✅|✅|96|20
KSUID                 |✅|✅|❌|❌|160|27
Timeflake             |✅|✅|❌|❌|128|22
Flake (Boundary)      |❌|✅|✅|✅|128|18
SCRU128               |✅|✅|❌|❌|128|25
SCRU64                |✅|✅|❌|❌|64|12
CUID 1                |✅|✅|✅|✅|-|25
CUID 2                |✅|❌|❌|❌|-|Variable
Nano ID               |✅|❌|❌|❌|-|Variable
Hashid                |?|?|?|?|-|Variable
Sqid                  |?|?|?|?|-|Variable
**Non-IDs:**          |
IPv4                  |✅|❌|❌|❌|32|7 to 15
IPv6                  |✅|❌|❌|❌|128|up to 39
MAC address           |❌|❌|✅|✅|48|17
Jira ticket           |❌|❌|✅|✅|-|Variable
Unix timestamp        |❌|✅|❌|❌|-|Variable









