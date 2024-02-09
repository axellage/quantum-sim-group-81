# Quantum simulation project
A education quantum computer simulation developed as part of a candidate thesis in Chalmers University of Technology in Spring 2024.
## Prerequisites
- Ensure you have Rust and Cargo installed. You can get them from the [official Rust website](https://www.rust-lang.org/tools/install).
- Rocket requires the latest stable version of Rust. Make sure you have it by running `rustup update`.
## Installation
### Backend
1. **Clone the Repository:**
  Start by cloning the repository to your local machine. Use the command:

```sh
git clone https://github.com/axellage/quantum-sim-group-81.git
cd quantum-sim-group-81/backend
```

2. **Install Dependencies:**
    Inside the backend directory, install the Rust project dependencies by running:

```sh
cargo build
```

This command will download and compile all the necessary dependencies.

### Frontend
## Running the Application
### Backend

  After setting up, you can start the backend server by running:

```sh
cargo run
```

  This command will compile the project (if not already compiled) and start the Rocket server. By default, the server will be available at http://localhost:8000, unless configured otherwise.

### Frontend
## API Endpoints
### Simulate
This endpoint simulates the quantum circuit specified in the request body. It takes a matrix representation of the grid and returns the list of states the quantum circuits will go through.
### Http Request
### `POST /simulate`
### Request Body
The request should contain a JSON object with a key `circuit_matrix`:
```json
{
  "circuit_matrix": [["Gate", "..."], ["Gate", "..."], "..."]
}
```
The circuit_matrix is a 2-dimensional list of strings, where each row represents a qubit and each column represents a concurrent step in the circuit. Each string in the matrix represents a quantum gate or a wire (identity operation). The quantum circuit initializes all qubits to the state |0>.

**Possible gates**

| Gate         | Key          | Notes        |
| -----------  | ------------ | ------------ |
| Pauli-X      | X        |         |
| Identity gate or wire| I         |         |
| Hadamard gate | H        |         |
| CNOT gate    | CNOT-1 & CNOT-2         | CNOT-1 is control and CNOT-2 is target*        |
| SWAP gate       | SWAP-1 & SWAP-2        |         |
| Toffoli gate    | CCNOT-1 & CCNOT-2 & CCNOT-3         | CCNOT-1 and CCNOT-2 is control and CCNOT-3 is target* |

\* In current version the keys follwing gate-1 has to be directly below the first one

### Response Body
The response is a JSON object with a key state_list, which is a list of objects each representing the state of the quantum circuit at a particular step:
```json
{
    "state_list": [
        {
            "step": "Integer",
            "state": [
                ["Real", "Imaginary"],
                "..."
            ]
        },
        "..."
    ]
}
```
Each object in the state_list has a step indicating the current step, and a state representing the state vector after that step. The state is a list of complex numbers, where each complex number is represented as a list [Real, Imaginary] with real and imaginary parts (both float64).

### Example
Request:
```json 
{
    "circuit_matrix": [["H", "CNOT-1"], ["I", "CNOT-2"]]
}
```
Response:
```json
{
  "state_list": [
    {"step":0,"state":[[1.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]]},
    {"step":1,"state":[[0.7071067811865475,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.7071067811865475,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]]},
    {"step":2,"state":[[0.7071067811865475,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.7071067811865475,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]]}
  ]
}
```

In this example, the first qubits goes through a Hadamard gate and then used as control bit in a CNOT operation where the second qubit is the target. This results is in a equal superposition between the states |00> and |11>.
## Authors
Axel Bergman, Chiara Cesarini, Lucas Möller and Alexander Persson
## License

