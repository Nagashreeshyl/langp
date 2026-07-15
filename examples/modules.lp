@ Module example — standard library imports

use filesystem.
use math.

print filesystem.exists("/tmp").
print math.abs(-5).

text = read "examples/hello.lp".
print len(text).
