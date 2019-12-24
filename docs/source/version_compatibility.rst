****************************************
Smart and Sawtooth Version Compatibility
****************************************

The following table shows the compatible versions of the Smart transaction
processor, Smart SDK, and Sawtooth Rust SDK. It also shows the Docker tag for
the Smart transaction processor image.

 - The Smart transaction processor versions are
   the versions used as part of the transaction processor’s registration.

 - The Smart SDK versions are the Crate versions of the Rust library that should
   be set in the Cargo.toml file.

 - The Docker tag is the tag that should be used for the
   hyperledger/sawtooth-dgc-contract-tp image if including in a docker-compose yaml
   file.

 - The Sawtooth Rust SDK versions are the Crate versions of the Rust library
   that should be set in the Cargo.toml file.

+------------+----------+-----------+---------+--------------------------------+
| Smart      | Smart SDK| Docker Tag| Sawtooth| Changes                        |
| Transaction|          |           | Rust SDK|                                |
| Processor  |          |           |         |                                |
+============+==========+===========+=========+================================+
| 0.0        | 0.1      | 0.1       |  0.2    |                                |
+------------+----------+-----------+---------+--------------------------------+
| 0.2        | 0.2      | 0.2       |  0.3    | - Transaction context is a     |
|            |          |           |         |   trait                        |
|            |          |           |         | - API has new get_state_entry  |
|            |          |           |         |   to get one entry and         |
|            |          |           |         |   get_state_entries to get     |
|            |          |           |         |   multiple entries (plus       |
|            |          |           |         |   corresponding functions for  |
|            |          |           |         |   set and delete)              |
+------------+----------+-----------+---------+--------------------------------+
| 0.3        | 0.3      | 0.3       |  0.3    | - Adds native rust             |
|            |          |           |         |   implementation of the proto  |
|            |          |           |         |   messages to the Smart SDK and|
|            |          |           |         |   is used by the Smart         |
|            |          |           |         |   Transaction Processor.       |
|            |          |           |         | - Adds no-op logging macros to |
|            |          |           |         |   the Smart SDK                |
+------------+----------+-----------+---------+--------------------------------+
| 0.4        | 0.4      | 0.4       |  0.3    | - Replaces the no-op log macros|
|            |          |           |         |   with macros that will        |
|            |          |           |         |   marshal the log back to the  |
|            |          |           |         |   Smart Transaction Processor  |
|            |          |           |         |   where it will be logged.     |
+------------+----------+-----------+---------+--------------------------------+
