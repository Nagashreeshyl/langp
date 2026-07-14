@ valid: natural language I/O forms
text = read "settings.txt".
bytes = read_bytes "image.png".
lines = read_lines "data.csv".

write text to "backup.txt".
write_bytes data to "output.bin".
append line to "log.txt".

response = get "https://google.com".
response = post url with data.
response = put url with body.
response = delete url.

copy "source.txt" to "dest.txt".
move "old.txt" to "new.txt".
delete "temp.txt".
