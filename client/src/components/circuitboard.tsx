import React, { useState, ReactNode, useEffect } from 'react';
import './circuitboard.css';

function Circuitboard() {
  const [qubitLines, setQubitLines] = useState<ReactNode[]>([]);

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
      //TODO make this a visible error
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

  return (
    <div className="Circuitboard">
      <section className="circuit">
        {qubitLines}
      </section>
      <button onClick={addQubit}>+</button>
      <button onClick={removeQubit}>-</button>
    </div>
  );
}

export default Circuitboard;