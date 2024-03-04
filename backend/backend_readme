# Backend
This document explains the structure and functionality of the backend.

## Methods
### handle_simulate_circuit
Handles  _/simulate_ endpoint using `simulate_circuit`.

### simulate_circuit
Takes the input from frontend and parses it into a circuit using `build_circuit_from_data`. The type of the circuit is `Vec<Vec<(Vec<i32>, QuantumGate)>>`. Each entry in the list corresponds to a time step, and each entry in that time step is a tuple with a gate matrix and the qubits that gate affects. In the case that the qubits aren't entangled from previous gates, a one qubit gate the list will consist of one qubit, for CNOT two qubits etc. In the other case where qubits have been entangled and a gate is acting upon only a subset on them, for example a circuit with a CNOT and then a Hadamard gate, the gate will be expanded using Kronecker with the identity gate so all qubits are included in it. The reason for this is so that the gates don't have to be modified after this method which simplifies the state vector calculations.

After the circuit has been parsed the method calculates the state vectors for each time step. First six qubits with state `[1 0]^(-1)` are initialized and added to the first time step. Then, for every entry in the circuit, the gates at that time step are applied to the state vectors with the same corresponding qubits. In some cases these qubits can be in separate groups before the gate, and in that case they need to be combined into one large state so the gate can be applied.

## Examples
 TODO

