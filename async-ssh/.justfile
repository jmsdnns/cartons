default:
    just -l

lsvm:
  limactl ls

ipvm id:
   @limactl shell {{id}} ip address show dev lima0 \
   | grep 'inet ' \
   | sed -e 's/^[ \t]* //' \
   | cut -f 2 -d' ' \
   | awk '{split($0,a,"/"); print a[1]}'

ipswarm name:
  @for id in $(limactl list --format '{{{{.Name}}' | grep {{name}}); do \
    just ipvm $id; \
  done

iplist name:
  @just ipswarm {{name}} \
  | sed 's/.*/"&"/' \
  | paste -sd, - \
  | sed 's/.*/\[&\]/'

mkvm id:
  limactl create \
    --name="{{id}}" \
    --vm-type=vz \
    --rosetta \
    --mount-type=virtiofs \
    --mount-writable \
    --network=vzNAT \
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

