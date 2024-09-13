# Radio

_If you haven't used Tonic before, please check out the [vibing project](../vibing) first. This doc assumes you have knowledge that can be gained there._

This project is keeps things simple while demonstrating multiple types of message passing. One call receives a request and responds, another receives a request and streams responses back, and the last call has requests streaming in with responses streaming out.

The service itself is implemented in the form of a simple music service. No user input is necessary.

The client first asks the server for info about the band Converge. It gets info back that has the band's name and a list of band members.

```
*** GET INFO ABOUT CONVERGE ***
Band = BandInfo { name: "Converge", bio: "New England metalcore perfection", members: [BandMember { name: "Ben Koller", role: "Drums" }, BandMember { name: "Jacob Bannon", role: "Vocals" }, BandMember { name: "Nate Newton", role: "Bass" }, BandMember { name: "Kurt Ballou", role: "Guitar" }] }
```

The client then asks for a list of songs by Converge and the server streams back songs.

```
*** GET LIST OF CONVERGE SONGS ***
Song = Song { name: "Dark Horse", band: "Converge", album: "Axe To Fall" }
Song = Song { name: "Reap What You Sew", band: "Converge", album: "Axe To Fall" }
```

The client then calls the radio function where it streams a request for a song, to which the server streams a song back. The client then waits two seconds and does that again. And again. And again a fourth time, before quitting.

```
*** TURN ON THE RADIO ***
Next = RadioNext { index: 0 }
Song = Song { name: "Drunkenship of Lanterns", band: "The Mars Volta", album: "Deloused In The Comatorium" }
Next = RadioNext { index: 1 }
Song = Song { name: "Cygnus....Vismund Cygnus", band: "The Mars Volta", album: "Frances The Mute" }
Next = RadioNext { index: 2 }
Song = Song { name: "Dark Horse", band: "Converge", album: "Axe To Fall" }
Next = RadioNext { index: 3 }
Song = Song { name: "Reap What You Sew", band: "Converge", album: "Axe To Fall" }
```

## Running It

Open two terminals, one for the server and one for the client.

In the server terminal:

```shell
$ cd cartons/geerpc/radio/server
$ cargo run
```

In the client terminal:

```shell
$ cd cartons/geerpc/radio/client
$ cargo run
```

## Proto Definition

The radio service has three functions and five message types.

The `BandQuery` message is simply a way for clients to send a band name to the server. The `RadioNext` message is how the client tells the server which song it wants from the playlist. By default, the client simply counts from 0 up.

The other three message types, `Song`, `BandInfo`, and `BandMember` are used to pass data from the radio's artist database.

```protobuf
syntax = "proto3";

package radio;

service Radio {
    rpc GetBand(BandQuery) returns (BandInfo) {}
    rpc ListSongs(BandQuery) returns (stream Song) {}
    rpc Radio(stream RadioNext) returns (stream Song) {}
}

message BandQuery {
    string name = 1;
}

message BandMember {
    string name = 1;
    string role = 2;
}

message BandInfo {
    string name = 1;
    string bio = 2;
    repeated BandMember members = 3;
}

message Song {
    string name = 1;
    string band = 2;
    string album = 3;
}

message RadioNext {
    int32 index = 1; 
}
```

The actual definition is here: [protos/radio.proto](protos/radio.proto)

## Implementations

The server has three main files: `src/main.rs`, `src/data.rs`, and `radiodb.json`. 

The json file stores all of the information about bands and songs and `data.rs` contains functions for transforming that json into rust structs. The main file implements the functions defined in the proto file: `get_band`, `list_songs`, and `radio`. 

The `list_songs` function is the first time some new ideas are shown, relative to [vibing](../vibing). The first new idea is that we use a background thread to do the work of determining what data should be streamed to the client. The other new idea is that the background thread uses [Channels](https://doc.rust-lang.org/std/sync/mpsc/index.html) to communicate data to the main thread when it is ready to be streamed out to the client.

The `radio` function gets even more interesting. It uses Tonic's builtin streaming to receive requests from the client and it uses Tokio's `async_stream` for streaming responses to the client. The strangest part of the code, imo, is the way the output from `async_stream::try_stream!` is [pinned](https://doc.rust-lang.org/std/pin/struct.Pin.html) and then returned from the function as a RadioStream instance. Tonic can call `output.next()` to wait for an incoming request from the client to produce a response which is then streamed out to the client.

[ [server code](server/src/main.rs) ]

The client is essentially the opposite of the server, but it is less complicated. The `run_radio` function is similar to the `radio` function in the server, so I'll leave understanding it as an exercise to you.

[ [client code](client/src/main.rs) ]
