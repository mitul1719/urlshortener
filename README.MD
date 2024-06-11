# Project Name

URL SHORTENER

#### Usage
```bash
cargo watch -x run
```

## Description

#### Endpoints:

```
/shorten

JSON PAYLOAD : {"url":"http://www.example.com",validity:3000}

validity in seconds

RETURNS : SHORT URL APPENDED WITH ID
```

```
/redirect/:key 

:key -> id returned by /shorten endpoint

REDIRECTS THE USER TO THE ORIGINAL URL
```

