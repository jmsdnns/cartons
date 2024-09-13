# Asynchronous gRPC

Uses [Tokio](https://tokio.rs) for async and [Tonic](https://docs.rs/tonic/) for gRPC.

## Projects

* [Vibing](vibing/): Very simple example of a client sending requests and getting responses back from a server.
* [Radio](radio/): A more elaborate setup where the server has multiple RPC functions available. Different types of streaming is shown here, including full duplex.
