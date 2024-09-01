os_vm_args := if os() == "macos" { "--vm-type=vz --rosetta" } else { "" }

default:
    just -l

lsvm:
  limactl ls

hostvm id:
    @cat $(limactl info | jq -r .limaHome)/{{id}}/ssh.config \
    | grep -E "Hostname|Port" \
    | sed 's/^ *//' \
    | cut -f 2 -d ' ' \
    | paste -sd: -

hostswarm name:
  @for id in $(limactl list --format '{{{{.Name}}' | grep {{name}}); do \
    just hostvm $id; \
  done

hostlist name:
  @just hostswarm {{name}} \
  | sed 's/.*/"&"/' \
  | paste -sd, - \
  | sed 's/.*/\[&\]/'

mkvm id:
  limactl create \
    --name="{{id}}" \
    --mount-type=virtiofs \
    --mount-writable \
    {{os_vm_args}} \
    --cpus=1 \
    --memory=1 \
    --tty=false  \
    template://default

mkswarm name count:
  for i in $(seq 1 {{count}}); do \
    just mkvm {{name}}-$i; \
  done

startswarm name:
  for id in $(limactl list --format '{{{{.Name}}' | grep {{name}}); do \
    limactl start $id; \
  done

stopswarm name:
  for id in $(limactl list --format '{{{{.Name}}' | grep {{name}}); do \
    limactl stop $id; \
  done

rmvm id:
  limactl delete {{id}}

rmswarm name:
  for id in $(limactl list --format '{{{{.Name}}' | grep {{name}}); do \
    just rmvm $id; \
  done

shellvm id:
  eval $(limactl show-ssh --format=cmd "{{id}}" 2>/dev/null)

