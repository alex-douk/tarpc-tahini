# tarpc tahini
This branch explores a prototype for 
a Tahini wrapper around a tarpc service.

# Running the example
Server-side : `cargo run --bin server`
Client-side : `cargo run --bin client`


# Code changes

## Changes to Sesame
In crate `alohomora`, a new mod at `src/tarpc/mod.rs`
contains a hardcoded tahini service.

## Application

Found in project's root `src/simple-example` directory

## Current limitations
The end goal here is to have services be defined in application code, but
the internals live in Sesame's crate.

Currently, service declaration the Tahini generated code that result from it
are scoped in Sesame, allowing usage of `remove_bbox`-like methods.

Furthermore, the current implementation only acts on 1-deep PCons, aka
`PCon<T>`, rather than any `SesameType`. A straightforward fix of that 
is to call `unsafe_rpc(x.to_enum().remove_bboxes().from_enum())` to unwrap

and `MyProtectedType::deserialize(rpc_result.serialize(&mut serializer))` to rewrap the resulting 
value. 

This is very low performance if we have to apply 4 transformations for every RPC call.
A better design probably exists
