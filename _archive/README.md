This is built from the other project:

neopolitan-py/src/test_neopolitan.py

The source files are directly in the
`site/whatever/whatever.neo` locations

They get read by the builder and then
output as index.html

---

Run the neopolitan-py/src/.. build_watch_command.py
to get the command to watch for changes. then paste that
to run it.

Then run the start-brower-sync to auto refresh the local
page to see changes.


---

TODO: Add a ---> EOF header that goes to the next ---> EOF
and ignores any `--->` in it and just outputs. Set that
up so that it's a secondary header that goes under
`---> CONTENT` or `---> CODE` or whatever. 


---

Maybe setup so that it's a template based thing for the 
output. So that folks can define it however they want.
Make a page level teamplte that get identified by the 
filename that corresponds to the `---> TYPE` then define
any components in it. TBD on adding like a `---> RAW`
flag that would turn off any processing and just output
the raw area without translating the marddown style 
stuff. 

Though, there would be tricks with the content areas
in terms of how you put them together for the output

Maybe define somethings that are in the flow of the
document and somethings that are outside of it
in terms of meta data and titles and the like

Maybe anything it doesn't understand becomes metadata?
But that would only be output if there as 
a tag for it. 





