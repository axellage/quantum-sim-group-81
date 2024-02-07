import React, { useState, ReactNode, useEffect } from 'react';
import './circuitboard.css';

function Circuitboard() {
  const [ketLines, setKetLines] = useState<ReactNode[]>([]);

  useEffect(() => {
    // Initialize ketLines with three elements when the component mounts
    setKetLines([
      <div className="ket-line" data-testid="qubit-line" key={0}>
        <p>|0⟩</p><hr/>
      </div>,
      <div className="ket-line" data-testid="qubit-line" key={1}>
        <p>|0⟩</p><hr/>
      </div>,
      <div className="ket-line" data-testid="qubit-line" key={2}>
        <p>|0⟩</p><hr/>
      </div>
    ]);
  }, []); // Empty dependency array to ensure this effect runs only once, on mount

  const handleClick = () => {
    if (ketLines.length < 10) {
      setKetLines(prevKetLines => [
        ...prevKetLines,
        <div className="ket-line" data-testid="qubit-line" key={prevKetLines.length}>
          <p>|0⟩</p><hr/>
        </div>
      ]);
    }
    else {
      alert("No more qubits can be added");
      console.log("No more qubits can be added");
    }
  };

  return (
    <div className="Circuitboard">
      <section className="circuit">
        {ketLines}
      </section>
      <button onClick={handleClick}>+</button>
    </div>
  );
}

export default Circuitboard;