# Vibing

This project is a basic example of a gRPC service. The client will ask the user to post a comment and make explicit if they're feeling positive or negative about what they said.

Using it begins with the client asking us `What's going on?` This is a prompt for us to enter a comment. We then respond with either a 'b' or 's' when it asks for our vibe.

```
What's going on?
Hanging out. Watching some of Jon Gjengset's videos. Drinking some tea.
Vibe? (b)ussin or (s)krrt
b
```

A message is then sent to the server which will log it like this:

```
[bussin] Hanging out. Watching some of Jon Gjengset's videos. Drinking some tea.
```

And finally the client confirms the server got this by logging an acknowledgement:

```
[ack] [bussin] Hanging out. Watching some of Jon Gjengset's videos. Drinking some tea.
```

## Running It

Open two terminals, one for the server and one for the client.

In the server terminal:

```shell
$ cd cartons/geerpc/vibing/server
$ cargo run
```

In the client terminal:

```shell
$ cd cartons/geerpc/vibing/client
$ cargo run
```

After the client is built, it will eventually print `What's going on?`

The server will log that it received a message and then the client will log that it received confirmation.

## Proto Definition

There is one rpc function and two message types, the request and the response. The `Vibe` function takes a `VibeRequest` as input and responds with a `VibeResponse`.

A `VibeRequest` consists of a comment and a vibe. The comment is a string and the vibe is an enum for expressing positive or negative vibes with the comment.

_Note: positive vibes are `BUSSIN` and negative vibes are `SKRRT`_


```protobuf
syntax = "proto3";
package vibing;

service Vibing {
    rpc Vibe (VibeRequest) returns (VibeResponse);
}


message VibeRequest {
    string comment = 1;

    enum Vibe {
        BUSSIN = 0;
        SKRRT = 1;
    }
    Vibe vibe = 2;
}

message VibeResponse {
    string confirmation = 1;
}
```

The actual definition is here: [protos/vibing.proto](protos/vibing.proto)

## Implementations

Both the client and the server have a file called `build.rs`. The file is exactly the same in both. This file tells tonic to tell cargo to compile the protobuf definitions as part of the build process. That compilation process translates the proto files into Rust which is then used in both the server and client.

Loading the compiled objects looks unusual if you haven't seen it before. For example, here is how that's done in the server:

```rust
pub mod vibing {
    tonic::include_proto!("vibing");
}
use vibing::{
    vibing_server::{Vibing, VibingServer},
    VibeRequest, VibeResponse,
};
```

The server creates a struct, `VibeService` that implements `Vibing` and defines a function called `vibe` to match the definition in the proto file.

The `VibeService` defines how the service works and is passed to the `VibingServer` to run it as a Tonic server. 

[ [server code](server/src/main.rs) ]

The client only needs `VibeClient` and `VibeRequest`, but it also has the `tonic::include_proto!` line in its imports:

```rust
use vibing::{vibing_client::VibingClient, VibeRequest};

pub mod vibing {
    tonic::include_proto!("vibing");
}
```

The client is generated automatically from the proto definition and we simply pass `VibeRequests` to it.

[ [client code](client/src/main.rs) ]

