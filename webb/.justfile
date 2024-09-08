dn := if os() == "macos" { "lima nerdctl" } else { "nerdctl" }
appwdb := "app-w-db.yml"
justdb := "just-db.yml"
compose_file := justdb

default:
    just -l

infra-up:
    {{ dn }} compose -f infra/local/{{compose_file}} up -d

infra-down:
    {{ dn }} compose -f infra/local/{{compose_file}} down

appshell:
    {{ dn }} exec -it local-shell-1 bash

dbshell:
    {{ dn }} exec -it local-db-1 bash

logs:
    {{ dn }} compose -f infra/local/{{compose_file}} logs

build:
    cd app/ && cargo build

app:
    cd app/ && cargo run
