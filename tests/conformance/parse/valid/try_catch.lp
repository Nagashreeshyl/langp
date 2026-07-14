@ valid: try/catch/finally
try,
    risky_function().
catch error: NetworkError,
    print error.message.
catch error,
    print "Unknown error".
finally,
    cleanup().
..

try,
    resume = input file "Choose file".
    print resume.
catch error: InputCancelledError,
    print "Cancelled".
..
