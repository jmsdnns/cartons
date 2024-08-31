dn := if os() == "macos" { "lima nerdctl" } else { "nerdctl" }

default:
    just -l

infra-up:
    {{ dn }} compose -f infra/docker-compose.yml up -d

infra-down:
    {{ dn }} compose -f infra/docker-compose.yml down

dbshell:
    {{ dn }} exec -it local-db-1 bash

logs:
    {{ dn }} compose -f infra/docker-compose.yml logs
