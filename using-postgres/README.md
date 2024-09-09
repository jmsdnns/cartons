# Using Postgres

This project demonstrates asynchronous use of Postgres via SQLx.

## Containers

A `.justfile` is provided to simplify the CLI. A working install of containerd is required for using it.

On Linux, containers run directly on the host machine without a VM.

On macs, containers are run inside a Linux VM, provided by LimaVM. Start one before attempting to use the commands provided by Just.

```shell
$ limactl create default
$ limactl start default
```

## CLI Commands

The main things here are just turning on a postgres container (infra-up), opening a shell on the database server (dbshell), and shutting postgres down (infra-down).

```shell
$ cd using-postgres
$ just
Available recipes:
    dbshell
    default
    infra-down
    infra-up
    logs
```

Start the infra with: `just infra-up`. One that's up, open a shell in the postgres container with: `just dbshell`. Then connect to postgres like this: `psql -U postgres mydb`. 

```
> just dbshell
root@db:/# psql -U postgres mydb
psql (16.4 (Debian 16.4-1.pgdg120+1))
Type "help" for help.
mydb=#
```

Type `\dt` an confirm PG says `Did not find any relations`.

```sql
mydb=# \dt
Did not find any relations.
mydb=#
```

Open another terminal in the same directory and run the app with: `cargo run`. This will run the SQL migrations, creating the database tables used by the app. It then runs a few queries on the database to put data in it.

```
Creating a user
SUCCESS: 1
Loading the user
SUCCESS: User { id: 1, username: "jmsdnns", password_hash: "foo" }
Updating the password
SUCCESS
Back in the terminal with postgres, run the following SQL to confirm data is now there.
```

Back in the postgres terminal, confirm a scheme now exists. 

```sql
 Schema |       Name       | Type  |  Owner
--------+------------------+-------+----------
 public | _sqlx_migrations | table | postgres
 public | users            | table | postgres
(2 rows)
```

Select all rows from the users table and confirm some dummy data is in there now.

```sql
mydb=# select * from users;
 id | username | password_hash |          created_at           |          updated_at        
----+----------+---------------+-------------------------------+-------------------------------
  1 | jmsdnns  | foofighters   | 2024-09-09 03:23:34.894341+00 | 2024-09-09 03:23:34.909165+00
(1 row)
```

That's it, so exit the postgres instance and shut everything down.

```
mydb=# \q
root@db:/# exit
$ just infra-down
```

If you're on a mac, you probalby want to shut the VM down too.

```shell
$ limactl stop default
```
