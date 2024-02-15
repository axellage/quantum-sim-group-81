import './App.css';
import React, { useState, ReactNode, useEffect, useMemo} from 'react';
import './components/circuitboard.css';
import axios from 'axios';
import './components/toolbar.css';
import {Gate} from './components/gate';
import {DndContext, useDroppable} from '@dnd-kit/core';
import {
  createSnapModifier,
  restrictToHorizontalAxis,
} from '@dnd-kit/modifiers';

function Toolbar() {
    const gateX = <Gate name="X"/>;
    const gateY = <Gate name="Y"/>;
    const gateZ = <Gate name="Z"/>;
    const gateH = <Gate name="H"/>;

    return (
      <div className="toolbar">
        <section className="gate-container">
          {gateH}
          {gateX}
          {gateY}
          {gateZ}
        </section>
      </div>
    );

    
  }

function Circuitboard() {
  const [qubitLines, setQubitLines] = useState<ReactNode[]>([]);
  const [circuit, setCircuit] = useState([["H"]]);
  const [states, setStates] = useState("");
  const {isOver, setNodeRef} = useDroppable({
    id: "droppable",
  });
  const style = {
    opacity: isOver ? 1 : 0.5,
  };

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
  })
  
  }

  async function sendCircuit() {
    const response = await axios.post('http://localhost:8000/simulate',
        {circuit_matrix: circuit})
  .then(function(response: any){
    console.log(response);
    setStates(JSON.stringify(response.data.state_list));
  })}

  return (
    <div className="Circuitboard" ref={setNodeRef} style={style}>
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

function App() {
  const [parentH, setParentH] = useState(null);
  const [parentX, setParentX] = useState(null);
  const [parentY, setParentY] = useState(null);
  const [parentZ, setParentZ] = useState(null);
  const [gridSize, setGridSize] = React.useState(60);
  const snapToGrid = useMemo(() => createSnapModifier(gridSize), [gridSize]);

  return (
    <div className="App">
      <DndContext onDragEnd={handleDragEnd}>
        <Toolbar />
        <Circuitboard />
      </DndContext>
    </div>
  );

  function handleDragEnd(event:any) {
    // TODO limit number of draggables in droppable to 1.
    // probably by creating an isEmpty constant
    const {active, over} = event;
    if (active.id == 'X'){
      setParentX(over ? over.id : null);
    } else if(active.id == 'Y'){
      setParentY(over ? over.id : null);
    } else if(active.id == 'Z'){
      setParentZ(over ? over.id : null);
    } else if(active.id == 'H'){
      setParentH(over ? over.id : null);
    }
  }
}

export default App;
