# Vibing

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

After the client is built, it will eventually print `What's going on?` Write some text and hit enter. Then press b or s, to associated a vibe with how you answered the question, and hit enter.

The server will log that it received a message and then the client will log that it received confirmation.

