The idea behind this mock is simple
There's a server mock
and there are multiple client "test programs"
that can run against the server.
The idea is this is to be used when testing proxy implementations
You place your proxy between the server and the test programs and
fire away
If said programs exit 0, your proxy implementation is correct
