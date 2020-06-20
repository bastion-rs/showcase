# Bastion AWS Lambda

In this example we are using Bastion to send a get request to given list of URLs inside an AWS Lambda.

# Requirements

Docker is required in order to run serverless in local mode

## Usage
Install dependencies with:
```
$ npm install
```

And trigger lambda locally
```
$ ./node_modules/.bin/serverless invoke local -f page_fetcher -d \
    '{
      "sites": [
        "https://bastion.rs",
        "https://blog.bastion.rs",
        "http://google.com",
        "https://docs.rs/",
        "https://crates.io/",
        "https://twitter.com/",
        "https://news.ycombinator.com/",
        "http://play.rust-lang.org/",
        "http://catb.org/jargon/html/hates.html"
      ]
    }'
```
