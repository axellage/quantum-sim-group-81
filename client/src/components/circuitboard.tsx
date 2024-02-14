import React, { useState, ReactNode, useEffect } from 'react';
import './circuitboard.css';
import axios from 'axios';

function Circuitboard() {
  const [qubitLines, setQubitLines] = useState<ReactNode[]>([]);
  const [circuit, setCircuit] = useState([["H"]]);
  const [states, setStates] = useState("");

  useEffect(() => {
    // Initialize ketLines with three elements when the component mounts
    setQubitLines([
      <div className="qubit-line" data-testid="qubit-line" key={0}>
        <p>|0⟩</p><hr/>
      </div>,
      <div className="qubit-line" data-testid="qubit-line" key={1}>
        <p>|0⟩</p><hr/>
      </div>,
      <div className="qubit-line" data-testid="qubit-line" key={2}>
        <p>|0⟩</p><hr/>
      </div>
    ]);
  }, []); // Empty dependency array to ensure this effect runs only once, on mount

  const addQubit = () => {
    if (qubitLines.length < 10) {
      setQubitLines(prevQubitLines => [
        ...prevQubitLines,
        <div className="qubit-line" data-testid="qubit-line" key={prevQubitLines.length}>
          <p>|0⟩</p><hr/>
        </div>
      ]);
    }
    else {
      alert("No more qubits can be added");
      console.log("No more qubits can be added");
    }
  };

  const removeQubit = () => {
    if (qubitLines.length > 0) {
      setQubitLines(prevQubitLines => prevQubitLines.slice(0, -1));
    }
    else {
      //TODO make this a visible error
      console.log("Already 0 qubits");
    }
  };

  //const axios = require('axios');

  async function ping() {
    const response = await axios.post('http://localhost:8000/ping', {
          message: "ping"
  })
  .then(function(response: any){
    console.log(response);
  })}

  async function sendCircuit() {
    const response = await axios.post('http://localhost:8000/simulate',
        {circuit_matrix: circuit})
  .then(function(response: any){
    console.log(response);
    setStates(JSON.stringify(response.data.state_list));
  })}

  return (
    <div className="Circuitboard">
      <section className="circuit">
        {qubitLines}
      </section>
      <button onClick={addQubit}>+</button>
      <button onClick={removeQubit}>-</button>
      <button onClick={ping}>ping</button>
      <button onClick={sendCircuit}>send example circuit</button>
      <section className="states"><h1 style={{ color: 'white' }}>{states}</h1></section>
    </div>
  );
}

export default Circuitboard;