# headers

Generate perfect code headers every time, [Sol-DAO](https://github.com/Sol-DAO) edition.

## Installation

You need Rust and Cargo installed on your machine. See the installation guide
[here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```
git clone https://github.com/0xClandestine/headers/
cd headers
cargo install --path .
```

## Usage

```sh
headers "business logic"
```
This will copy the header to your clipboard automatically.

```
/// -----------------------------------------------------------------------
/// Business Logic
/// -----------------------------------------------------------------------
```



### With VSCode

Set your global [`tasks.json`](https://stackoverflow.com/questions/41046494/making-global-tasks-in-vs-code) like so to add the command as task:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Generate Header",
      "type": "shell",
      "command": "headers ${input:header}",
      "presentation": {
        "reveal": "never"
      }
    }
  ],
  "inputs": [
    {
      "id": "header",
      "description": "Header",
      "type": "promptString"
    }
  ]
}
```

To really speed-up your workflow, you can even add a keybind for the task in [`keybindings.json`](https://code.visualstudio.com/docs/getstarted/keybindings):

```json
[
  {
    "key": "CMD+h",
    "command": "workbench.action.tasks.runTask",
    "args": "Generate Header"
  }
]
```

This will copy the generated header to your clipboard.

## Credits

Inspired by virtualjpeg's [`blocky`](https://github.com/virtualjpeg/blocky).
