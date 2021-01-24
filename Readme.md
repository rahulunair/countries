## countries API in RUST

A simple client to query countries API

```bash
countries -c Bharat
fetching country details...
========================================
code :: IND
name :: India
native name :: भारत
capital :: New Delhi
population :: 1295210000
area :: 3287590
boders :: ["AFG","BGD","BTN","MMR","CHN","NPL","PAK","LKA"]
region :: Asia
demonym :: Indian
domain :: [".in"]
flag :: https://restcountries.eu/data/ind.svg
========================================
```

## TODO
- fix ugly unwrap
- take list of countries, process async
- score countries based on pop density, show list
- cache using lru

