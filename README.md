# tarpc tahini
This branch explores a prototype for 
a Tahini wrapper around a tarpc service.


# Examples

## Simple example

### Description

This example contains two RPC calls :
- one to test the overall infrastructure (jsut increments a counter)
- One to check various types for serializability

### Run
Server-side : `cargo run --bin simple_server`
Client-side : `cargo run --bin simple_client`

## LLM example

### Description

This example is made of three binaries:
- `llm_server.rs` : Contains an LLM inference server that takes a prompt as an input, does inference on it. In case of success, store the conversation to a database. Return to a user the infered tokens or an error, with the associated UUID of the DB entry.

- `database_server.rs`: Contains a two-level database: One table per user, which contains UUID: chats pairs. Returns the optional entry if it exists for this (user, UUID) pair.

- `llm_client.rs`: Requires inference to the LLM server, then tries to access the database entry associated with the UUID. Also tries to access a nonexsistent UUID for testing.

### Run
LLM server: `cargo run --bin llm_server [--release]`
Run as release for better performance

Database server: `cargo run --bin llm_db`

Client: `cargo run --bin llm_client`

# Code changes

## Changes to Sesame
A lot: TODO
