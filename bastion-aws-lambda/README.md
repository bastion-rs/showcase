# Bastion AWS Lambda

In this example we are using Bastion to send a get request to given list of URLs inside an AWS Lambda.

## Usage
Install serverless with:
```
$ npm install -g serverless
$ npm install --save-dev serverless-rust
```

And trigger lambda locally
```
$ serverless invoke local -f page_fetcher -d \
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
