.. _codingstyle:

Coding Style
============

This document outlines the preferred coding style for the Linux kernel. Following a consistent style improves both readability and maintainability of the code.

1. Indentation
--------------
  
- Tabs are considered 8 characters and so are indentations. This increases readability and warns of overly nested functions.

.. code-block:: rust

  do_something() {
    do_anything();
  }

- Avoid putting multiple statements on a single line unless for hiding something obscure.

.. code-block:: c

	if (condition) do_once;
	        do_something_always;
  
- Avoid using commas to avoid braces.

.. code-block:: c

	if (condition)
	        first(), second();

- Always use braces for multiple statements.
   
.. code-block:: c

	if (condition) {
	        first();
	        second();
	}

- Don't put multiple assignments on a single line.
- Avoid tricky expressions.

2. Breaking Long Lines and Strings
----------------------------------

- The preferred line length limit is 80 characters.
- Long statements should be broken into sensible chunks, \
  unless exceeding 80 characters significantly improves readability and doesn't hide information.
- The same rules apply to function headers with long argument lists.
- However, don't break user-visible strings like printf messages for grep-ability.

3. Placing Braces and Spaces
----------------------------

- Put the opening brace on the last line of a control structure (if, switch, for, while, do) \
  and the closing brace on the first line of the following block.

.. code-block:: c
  
  if (condition) {
          do_something();
          do_something_else();
  }

This applies to all non-function statement blocks.

- Functions have the opening brace at the beginning of the next line.

.. code-block:: c

	int function(int x)
	{
	        do_something(x);
	}

- The closing brace is empty on a line of its own, except when followed \
  by continuation of the same statement (like a "while" in a do-statement).

.. code-block:: c

	do {
	        do_something();
	} while (condition);

and

.. code-block:: c

	if (condition) {
		..
	} else if (condition) {
		...
	} else {
		....
	}

This style minimizes empty lines while maximizing space for comments.

- Don't unnecessarily use braces for single statements.

.. code-block:: c

	if (condition)
	        do_something();

and

.. code-block:: c

	if (condition)
	        do_something();
	else
	        do_anything();

- Use braces when a loop contains more than one simple statement.
  
.. code-block:: c

	while (condition) {
	        if (condition_2)
	                do_something();
	}

3.1 Spaces
**********

- Use a space after keywords like if, switch, case, for, do, while, *but not with* sizeof, typeof, alignof, or __attribute__.
  
.. code-block:: c
  
  if (condition)
          do_something();

- No spaces around (inside) parenthesized expressions.

.. code-block:: c

  /* don't do like that */
  variable = sizeof( struct something );

- When declaring pointer data or a function that returns a pointer type, the preferred placement of * is adjacent to the data name/function name, not to the type name.

.. code-block:: c


	char *something;
	unsigned int do_something(char *ptr, char **retptr);
	char *do_anything(int *i);

- Use one space around most binary and ternary operators::

    =  +  -  <  >  *  /  %  |  &  ^  <=  >=  ==  !=  ?  :
  
- But no space after unary operators, before postfix increment/decrement operators, or around . and -> structure member operators::

    &  *  +  -  ~  !  ++  --  .  ->
    
- Don't leave trailing whitespace at the end of lines.

4. Naming
---------

- Local variable names should be short and descriptive, reflecting their purpose.
- Avoid generic names like tmp or i.
- Function names should be descriptive and indicate what the function does.

5. Typedefs
-----------

- Avoid using typedef for structures and pointers. Their type should be clear directly.
- typedef is useful for
    - Opaque objects accessed through accessor functions (e.g., pte_t).
    - Clear integer types to avoid confusion (e.g., u8, u16).
    - Creating new types in sparse.
  
- Generally, don't use typedef for pointers or directly accessible struct types.

6. Functions
------------

- Functions should be short and focused, ideally fitting on one or two screens.
- More complex functions may be longer as long as they remain understandable.
- Use helper functions with descriptive names for large functions.
- Aim for 5-10 local variables per function.
- Separate functions with one blank line.
- If exported, add the EXPORT macro after the closing brace in the source file.

.. code-block:: c

	int doing_something(void)
	{
	        return task_state == DOING SOMETHING;
	}

6.1 Function Prototypes
***********************

- Include parameter names with their data types in function prototypes for clarity.
- Don't use the extern keyword with function declarations.
- Maintain a specific order for prototype elements: storage class, storage class attributes, return type, return type attributes, function name, function parameters, function parameter attributes, function behavior attributes.

7. Commenting
-------------

- C++ style comments are unacceptable.

.. code-block:: c

  int i; //this comment is unaccaptable

- Comment on what your code does, not how it works.
- Over-commenting is discouraged. Write clean code that explains itself.
- Place comments at the head of functions to explain their purpose and functionality.
- For long comments, use the preferred styles outlined.

.. code-block:: c

	/*
	 * This is the preferred style
	 *
	 * something goes here :)
	 */

- Comment data declarations for easier understanding.
