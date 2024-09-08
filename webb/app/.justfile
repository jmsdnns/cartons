valid_token := "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6Imptc2RubnMiLCJleHAiOjE3MjU5MDk2MzgsImlhdCI6MTcyNTgyMzIzOH0.nlAoGAJ1HotI8-oC8pYAa9KRrd8yDNbKCNFP3T26gmE"
expired_token := "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6Imptc2RubnMiLCJleHAiOjE3MjU4Mjg5MjgsImlhdCI6MTcyNTgyODkyM30.5Or2_SeH_XAhnd5IgaokA4N_SMFcg0rfXxMf91C05rk"
bt := valid_token
#bt := expired_token

default:
    @just -l

mkaccount:
    curl -X POST \
        -H "Content-type: application/json" \
        -d '{"username":"jmsdnns","password":"foo"}' \
        -v \
        http://localhost:5454/account/create

login:
    curl -X POST \
        -H "Content-type: application/json" \
        -d '{"username":"jmsdnns","password":"foo"}' \
        -v \
        http://localhost:5454/account/login

badlogin:
    curl -X POST \
        -H "Content-type: application/json" \
        -d '{"username":"jmsdnns","password":"food"}' \
        -v \
        http://localhost:5454/account/login

landing:
    curl -X GET \
     -H 'Content-Type: application/json' \
     -H 'Authorization: Bearer {{bt}}' \
     -v \
     http://localhost:5454/landing

badlanding:
    curl -X GET \
     -H 'Content-Type: application/json' \
     -H 'Authorization: Bearer meowmeow' \
     -v \
     http://localhost:5454/landing

notfound:
    curl -v http://localhost:5454/account
