@ valid: event handlers
on button.clicked,
    print "Clicked".
.

on Browser.Start,
    print "Browser started".
.

on server.request,
    response = handle(server.request).
    server.respond(response).
.

on server.request where server.request.path == "/",
    server.respond(response(200, body = "Home")).
.
