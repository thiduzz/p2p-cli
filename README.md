
# p2p-cli

A CLI tool to communicate with a P2P network with other nodes with the concept of rooms and membership

## Run Locally

Clone the project

```bash
  git clone git@github.com:thiduzz/p2p-cli.git
```

Go to the project directory

```bash
  cd p2p-cli
```

Run the node

```bash
  cargo run
```

## Flow Diagrams

```mermaid
flowchart TD
    Build --> A["Load Address from Env"]
    A --> B["Load Username from Args"]
    B --> C["Connect P2P Client"]
    C --> D["List rooms and flag currently joined"]
    D --> I["Create Room"]
    I --> G
    D --> E["Select room"]
    E --> G["Enable polling of messages"]
    G --> H["Load current messages of room"]
```
```mermaid
flowchart LR
    A("Room Actions") --> B("Send Message")
    A --> C("List users")
    A --> D("Edit message by ID")
    A --> E("Delete message by ID")
    A --> F("Leave room")
```
```mermaid
classDiagram
  direction RL
  class User {
    -id : int32
    -username : string
  }
  class Room{
    -id : int32
    -name : string
    -members : vec<int32>
  }
  class Message{
    -id : int
    -text : string
    -created_at : chrono:DateTime
  }
  Room "*" o-- "*" Message : belongs
  User "1" --o "*" Room : participates
```

## Authors

- [@thiduzz](https://www.github.com/thiduzz)


## License

[MIT](https://choosealicense.com/licenses/mit/)