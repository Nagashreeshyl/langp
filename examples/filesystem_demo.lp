@ Filesystem example — read, write, copy, list

use filesystem.

write "Hello Lang.P filesystem" to "notes.txt".
text = read "notes.txt".
write text to "backup.txt".

print filesystem.exists("notes.txt").
print filesystem.exists("missing.txt").

copy "notes.txt" to "notes-copy.txt".
delete "notes-copy.txt".

filesystem.create_folder("Project").
write "data" to "Project/data.txt".
print filesystem.list("Project").

filesystem.remove_folder("Project").
delete "backup.txt".
delete "notes.txt".

print "Filesystem demo done.".
