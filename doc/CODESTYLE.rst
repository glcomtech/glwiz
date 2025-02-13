.. _codingstyle:

Coding Style
============

This document outlines the preferred coding style for Rust projects. Following a consistent style improves both readability and maintainability of the code.

1. Indentation
--------------

- Use 4 spaces for indentation. Avoid using tabs to ensure consistency across different editors.

.. code-block:: rust

    fn do_something() {
        do_anything();
    }

- Avoid putting multiple statements on a single line unless it enhances clarity.

.. code-block:: rust

    if condition { do_once(); }
    do_something_always();

- Always use braces for control flow statements with multiple lines.

.. code-block:: rust

    if condition {
        first();
        second();
    }

- Don't put multiple assignments on a single line.
- Avoid complex expressions that reduce readability.

2. Breaking Long Lines and Strings
----------------------------------

- The preferred line length limit is 80 characters.
- Long statements should be broken into sensible chunks, unless exceeding 80 characters significantly improves readability.
- The same rules apply to function headers with long argument lists.
- Avoid breaking user-visible strings for better grep-ability.

3. Placing Braces and Spaces
----------------------------

- Place the opening brace on the same line as the control structure (if, match, for, while) and the closing brace on its own line.

.. code-block:: rust

    if condition {
        do_something();
        do_something_else();
    }

This applies to all non-function statement blocks.

- Functions should have the opening brace on the same line.

.. code-block:: rust

    fn function(x: i32) {
        do_something(x);
    }

- The closing brace should be on its own line, except when followed by a continuation of the same statement.

.. code-block:: rust

    loop {
        do_something();
    }

and

.. code-block:: rust

    if condition {
        // ...
    } else if condition {
        // ...
    } else {
        // ...
    }

This style minimizes empty lines while maximizing space for comments.

3.1 Spaces
**********

- Use a space after keywords like if, match, for, while, but not with sizeof or similar.

.. code-block:: rust

    if condition {
        do_something();
    }

- No spaces around (inside) parenthesized expressions.

.. code-block:: rust

    let size = std::mem::size_of::<Something>();

- When declaring pointer types or functions returning pointers, place the `*` adjacent to the variable name, not the type.

.. code-block:: rust

    let something: *const Something;
    fn do_something(ptr: *const i32) -> *const i32;

- Use one space around most binary and ternary operators:

    =  +  -  <  >  *  /  %  |  &  ^  <=  >=  ==  !=  ?  :

- No space after unary operators, before postfix increment/decrement operators, or around . and -> structure member operators:

    &  *  +  -  ~  !  ++  --  .  ->

- Don't leave trailing whitespace at the end of lines.

4. Naming
---------

- Local variable names should be short and descriptive, reflecting their purpose.
- Avoid generic names like `tmp` or `i`.
- Function names should be descriptive and indicate what the function does, using snake_case.

5. Type Aliases
---------------

- Avoid using type aliases for structures and pointers. Their type should be clear directly.
- Type aliases are useful for:
    - Opaque types accessed through accessor functions.
    - Clear integer types to avoid confusion (e.g., u8, u16).
    - Creating new types in specific contexts.

- Generally, avoid type aliases for pointers or directly accessible struct types.

6. Functions
------------

- Functions should be short and focused, ideally fitting on one or two screens.
- More complex functions may be longer as long as they remain understandable.
- Use helper functions with descriptive names for large functions.
- Aim for 5-10 local variables per function.
- Separate functions with one blank line.
- If exported, use the `pub` keyword before the function definition.

.. code-block:: rust

    pub fn doing_something() -> bool {
        task_state == TaskState::DoingSomething
    }

7. Commenting
-------------

- Comment on what your code does, not how it works.
- Over-commenting is discouraged. Write clean code that explains itself.
- Place comments at the head of functions to explain their purpose and functionality.
- For long comments, use the preferred styles outlined.

.. code-block:: rust

    // This is the preferred style for single-line comments

.. code-block:: rust

    /*
     * This is the preferred style for multi-line comments
     *
     * something goes here :)
     */

- Comment data declarations for easier understanding.

.. code-block:: rust

    /// Represents a user in the system.
    struct User {
        id: u32,
        name: String,
    }

- Use doc comments (`///`) for public items to generate documentation automatically.

By adhering to these guidelines, we can ensure that our Rust code remains clean, readable, and maintainable, fostering a collaborative and efficient development environment.