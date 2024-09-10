# Asynchronous Password Hashing

Password hashes are best when they hog the CPU for a brief period of time because that slows hackers down from brute force attacks. Therefore, they are also good examples of how to put CPU intensive tasks on a different thread from the main event loop in async systems.

Running it produces the following output:

```
Password:  meowmeowbeans
Pass Hash: "$argon2id$v=19$m=19456,t=2,p=1$V+fcv+3NAMKPl94SdHcZKg$rgx0oTvDhjAhoOsVwki/h2RU1nEC2fzYFxqilWQX1jY"
Trying: meowmeowbeans
- verified
Trying: woofwoof
- nope
```
