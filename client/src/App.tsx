import React, { useState } from 'react';
import './App.css';
import './circuitboard.css';
import {DndContext} from '@dnd-kit/core';
import {useDraggable, useDroppable} from '@dnd-kit/core';
import {CSS} from '@dnd-kit/utilities';
import axios from 'axios';

function App() {
  // This matrix doesn't contain actual elements, just information about what the circuit looks like.
  const [circuit, setCircuit] = useState([["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"]]);
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
    <div>
      <Gate name="X"/>
      <Gate name="Y"/>
      <Gate name="Z"/>
      <Gate name="H"/>
    </div>
    );
  }
  
  function Circuitboard(){
    return(
    <div>
      <QubitLine id="0"/>
      <QubitLine id="1"/>
      <QubitLine id="2"/>
      <button onClick={sendCircuit}>send circuit</button>
      <section className="states">
        {states.map((timeStep) => (
        <h2>{JSON.stringify(timeStep.state)}</h2>
        ))}
      </section>
    </div>)
  }
  
  function QubitLine(props:any){
    return (
    <div className='qubitLine'>
      <h2>|0⟩</h2>
      <hr/>
      {// This generates a qubit line element from the 'circuit' matrix.
      }
      {circuit[Number(props.id)].map((gate, index) => <Slot gateType = {gate} id = {props.id + index.toString()}/>)}
    </div>);
  }
  
  function Gate(props:any) {
    const {attributes, listeners, setNodeRef, transform} = useDraggable({
        id: props.name,
      });
      const style = {
        transform: CSS.Translate.toString(transform),
        
      };
      
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
      color: 'black',
      backgroundColor: 'white',
      width: 70,
      height: 70
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
  const style = {
    width: 60,
    height: 60,
    backgroundColor: "cyan",
    color: "white"
  };
  
  // Display nothing if there is no placed gate (which is the same as the identity gate).
  if(props.name != "I"){
    return (
      <button style={style} className = "placedGate">
        <h1>{props.name}</h1>
      </button>
    );
  } else return null;
  
  
}



export default App;
