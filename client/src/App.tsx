import React, { useState, ReactNode, useEffect } from 'react';
import './App.css';
import './circuitboard.css';
import './toolbar.css';
import {DndContext} from '@dnd-kit/core';
import {useDraggable, useDroppable} from '@dnd-kit/core';
import {CSS} from '@dnd-kit/utilities';
import axios from 'axios';

function App() {
  // This matrix doesn't contain actual elements, just information about what the circuit looks like.
  const [circuit, setCircuit] = useState([["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"]]);
  // Initializing this because it complains about type otherwise, there is probably a better way to do it.
  const [states, setStates] = useState([{"step":0, "state":[]}]);

  // TODO implement setCircuit (aka add + and - buttons).

  return (
    <div className="App">
      <DndContext onDragEnd={handleDragEnd}>
        <Toolbar />
        <Circuitboard />
      </DndContext>
    </div>
  );

  function Toolbar(){
    return (
    <div className='Toolbar'>
      <Gate name="X"/>
      <Gate name="Y"/>
      <ControlGate name="."/>
      <Gate name="Z"/>
      <Gate name="H"/>
    </div>
    );
  }
  
  function Circuitboard(){
    const [qubitLines, setQubitLines] = useState<ReactNode[]>([]);
    useEffect(() => {
      // Initialize ketLines with three elements when the component mounts
      setQubitLines([
        <div>
          <QubitLine id="0"/>
        </div>,
        <div>
          <QubitLine id="1"/>
        </div>,
        <div>
          <QubitLine id="2"/>
        </div>,
        <div>
          <QubitLine id="3"/>
        </div>,
        <div>
          <QubitLine id="4"/>
        </div>,
        <div>
          <QubitLine id="5"/>
        </div>
      ]);
    }, []); // Empty dependency array to ensure this effect runs only once, on mount
  
  {/*  const addQubit = () => {
      if (qubitLines.length < 6) {
        setQubitLines(prevQubitLines => [
          ...prevQubitLines,
          <div>
            <QubitLine id={JSON.stringify(qubitLines.length)}/>
          </div>
        ]);
      }
      else {
        alert("No more qubits can be added");
        console.log("No more qubits can be added");
      }
    };
  
    const removeQubit = () => {
      if (qubitLines.length > 1) {
        setQubitLines(prevQubitLines => prevQubitLines.slice(0, -1));
      }
      else {
        //TODO make this a visible error
        console.log("Already 0 qubits");
      }
    };*/}

    return(
    <div>
      <section className="circuit">
        {qubitLines}
      </section>
      {/*<button onClick={addQubit}>+</button>
      <button onClick={removeQubit}>-</button>*/}
      <button onClick={sendCircuit}>send circuit</button>
      <section className="states">
        {states.map((timeStep) => (
        <h2>{JSON.stringify(timeStep.state)}</h2>
        ))}
      </section>
    </div>)
  }
  
  function QubitLine(props:any) {
    const qubitLineId = Number(props.id);
    const circuitLine = circuit[qubitLineId] || []; // Fallback to an empty array if circuit[qubitLineId] is undefined
  
    return (
      <div className='qubitLine'>
        <h2>|0⟩</h2>
        <hr/>
        <div className='slot-container'>
          {circuitLine.map((gate, index) => <Slot gateType={gate} id={`${qubitLineId}${index}`} key={`${qubitLineId}${index}`} />)}
        </div>
      </div>
    );
  }
  
  function Gate(props:any) {
    const {attributes, listeners, setNodeRef, transform} = useDraggable({
        id: props.name,
      });
      const style = {
        transform: CSS.Translate.toString(transform),
        width: 50,
        height: 50
        
      };
      
      return (
        <button ref={setNodeRef} style={style} {...listeners} {...attributes}>
          <h1>{props.name}</h1>
        </button>
      );
  }

  function ControlGate(props:any) {
    //TODO: getTimestepGates()
    const {attributes, listeners, setNodeRef, transform} = useDraggable({
        id: props.name,
      });
      const style = {
        transform: CSS.Translate.toString(transform),
        width: 50,
        height: 50
        
      };

      const controledGates: string[] = [];
      
      return (
        <button ref={setNodeRef} style={style} {...listeners} {...attributes}>
          <h1>{props.name}</h1>
        </button>
      );
  }
  
  function Slot(props:any) {
    const {isOver, setNodeRef} = useDroppable({
      id: props.id,
    });

    // TODO: Move to CSS.
    const style = {
      opacity: (isOver ? .8 : 1),
    };
  
    return (
      <div ref={setNodeRef} style={style}>
        <PlacedGate name = {props.gateType} />
      </div>
    );
  }

  function handleDragEnd(event:any){
    const {active, over} = event;

    console.log("Placed gate on position " + over.id[1] + " on qubit line " + over.id[0]);

    // These nested maps replace the gate at the given position.
    const newCircuit = circuit.map((line, i) => {
      if(i === (Number(over.id[0]))) {
        return (line.map((gate, j) => {
          if(j === (Number(over.id[1]))){
            return (active.id);
          } else{
            return (gate);
          }
        }));
      } else {
        return line;
      } 
    });
    setCircuit(newCircuit);
  }

  async function sendCircuit() {
    const response = await axios.post('http://localhost:8000/simulate',
        {circuit_matrix: circuit})
  .then(function(response: any){
    console.log(response);
    setStates(response.data.state_list);
  })}
}

function PlacedGate(props:any){
  
  // Display nothing if there is no placed gate (which is the same as the identity gate).
  if(props.name != "I"){
    return (
      <button className = "placedGate">
        <h1>{props.name}</h1>
      </button>
    );
  } else return null;
  
  
}



export default App;
