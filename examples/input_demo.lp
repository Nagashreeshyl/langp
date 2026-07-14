@ input_demo.lp — Demonstrates the built-in input expression.
@
@ Run interactively: langc input_demo.lp --mode interpret
@ Requires a TTY for text input; picker inputs need a graphical environment.

@ Generic text input (returns String)
name = input "Enter your name : ".
print "Hello " with name with "!".

@ Typed inputs with validation
age = input number "Enter your age : ".
salary = input decimal "Enter your salary : ".
online = input boolean "Are you online? ".

@ Masked password input
password = input password "Enter your password : ".
print "Password length: " with len(password).

@ Summary
print "--- Summary ---".
print "Name: " with name.
print "Age: " with age.
print "Salary: " with salary.
print "Online: " with online.

@ Picker-based input (native dialogs — uncomment to try in a GUI environment)
@ resume = input file "Choose your resume".
@ folder = input folder "Select a folder".
@ birth_date = input date "Select your birth date".
@ theme_color = input color "Choose a theme color".

@ Error handling for cancelled pickers
@ try,
@     resume = input file "Choose your resume".
@     print "Selected: " with resume.
@ catch error: InputCancelledError,
@     print "No file selected.".
@ ..
