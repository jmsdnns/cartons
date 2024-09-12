# Signed Tokens

Uses [JsonWebToken](https://docs.rs/jsonwebtoken/) for auth tokens, [Serde](https://docs.rs/serde/latest/serde/) for serialization / deserialization of token, and [Chrono](https://docs.rs/chrono/) for the token fields based on datetimes.

## Running It

The output for this example has two parts. A valid token is shown first and an expired token is shown after.

The JsonWebToken library automatically checks if tokens are expired as part of the default validation process, but there is a `leeway` field that allows a token to be valid for up to 60 seconds after expiry. The author claims this is helpful for time drift between computers. The example code here sets it 0 for the expired token example to avoid making anyone stare at the screen for 60 seconds to see expiration work.

```shell
VALID TOKEN
-----------
TOKEN: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6Im15ZW1haWxAd2VlZWUuY29tIiwiZXhwIjoxNzI2MjY0MDAyLCJpYXQiOjE3MjYxNzc2MDJ9.jLxsVAZrktTOQxww5sm1wYs43_tGEGdIX9bopWx3uNY"
CLAIMS: Ok(Claims { email: "myemail@weeee.com", exp: 1726264002, iat: 1726177602 })

EXPIRED TOKEN
-------------
TOKEN: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6Im15ZW1haWxAd2VlZWUuY29tIiwiZXhwIjoxNzI2MTc3NjAzLCJpYXQiOjE3MjYxNzc2MDJ9.1AvDRSj00_R46HS-8ElfrpOZNH-LbgpHw3zgyU0Jlsc"
SLEEP 2 SECONDS
CLAIMS: Err(Error(ExpiredSignature))
```
