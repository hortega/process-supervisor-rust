# Process supervisor
Daemon build in Rust that controls the state of different applications according to the configuration received via REST JSON commands.

## How to run
1. Clone the repository and `cd` into it.
1. Run `cargo run`.
1. Send __PUT__ requests to __http://localhost:8000/commands__ with the body in the next format:
  ```json
  {
    command: ["/path/to/executable", "argument1", "argument2"],
    cwd: "/path/to/workdir",
    state: "running"
  }
  ```
  __state__ can be either __running__ or __stopped__.

The supervisor will start a new process for any command in __running__ state, or for a command in __running__ state that was killed outside of the application. If a command state is changed from __running__ to __stopped__, the process will be killed.


## Possible improvements
1. Better error handling. Right now errors starting or killing processes will result in `panic`.
1. Refactor supervisor functionality outside of the `main` module.
1. Write unit and integration tests for the application.
