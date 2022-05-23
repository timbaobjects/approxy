approxy
=======

approxy is a custom proxy service that presents a webhook for the NGtrend 
SMS aggregator and then translates the request into one that Apollo understands.

NGtrend usually makes a HTTP POST request to the chosen endpoint with a form 
with the parameters: `msg` for the contents of the text message and `tel` for 
the sender phone number. They also expect an xml-encoded response as the reply.

Apollo on the other hand would take HTTP GET parameters: `sender` for the 
sender, `text` for the contents of the text message and `secret` for the custom 
messaging secret used to authenticate valid requests.

```
Usage: approxy [-p <port>] -u <url> -s <secret>

Options:
  -p, --port        listening port (default: 3030)
  -u, --url         base url for Apollo service
  -s, --secret      messaging secret
  --help            display usage information
```