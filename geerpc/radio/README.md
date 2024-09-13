# Radio

_If you haven't used Tonic before, please check out the [vibing project](../vibing) first. This doc assumes you have knowledge that can be gained there._

This project is keeps things simple while demonstrating multiple types of message passing. One call receives a request and responds, another receives a request and streams responses back, and the last call uses full-duplex streaming so the client can stream requests to the server which streams responses back.

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

[ [server code](server/src/main.rs) ]

[ [client code](client/src/main.rs) ]
