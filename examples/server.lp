@ server.lp — Minimal HTTP server example.

@ Import the network module for HTTP server capabilities.
use network.

@ Handle an incoming HTTP request and return a response.
function handle_request(request),
    name = request.query.get("name", default = "World").
    body = "Hello " with name with "!".
    return response(200, body = body, content_type = "text/plain").
..

@ Create an HTTP server listening on port 8080.
server = Server(port = 8080).

@ Process each incoming request.
on server.request,
    reply = handle_request(server.request).
    server.respond(reply).
..

@ Start the server and print a status message.
server.start().
print "Server running on http://localhost:8080".
