-> TITLE

Neopolitan

-> SUBTITLE 

A plain-text format to build my websites

-> H2

Test Notes

-> LIST

- GOAL: To processor works with multiple
passes. The first pass converts everything
to `<<...>>`` tags. The second pass turns
those things into an AST which is then 
used for the output. 






-> H2

Testing

The in progress tests point to `output_dev``
and `parse_dev``. That lets them be worked
on independently of the rests of the tests. 

Finished tests use `output_switch`` and
`parse_switch``. The swithc files are genearlly
pointed to the production `output`` and `parse``
files. When a `_dev`` file is ready for testing 
the `_switch`` files are updated to point to 
`_dev``. That lets everything be tested with 
the new code at one time. 

When everthing is passing the content of the
`_dev`` file is copied to the productio file
and the function renamed to remove `_dev``. 
The new test file gets changed from `_dev`` 
to `_switch`` and all the test files are 
update to point to `_switch``. 

