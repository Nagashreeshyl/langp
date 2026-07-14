@ valid: function declarations and calls
function greet(name),
    print "Hello " with name.
..

function add(a: Int, b: Int) -> Int,
    return a + b.
..

async function fetch(url),
    response = wait for get url.
    return response.body.
..

greet("Naga").
result = add(3, 4).
data = fetch("https://api.example.com").
