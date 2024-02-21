import React, { useState } from 'react';
import './App.css';
import './circuitboard.css';
import './toolbar.css';
import {DndContext} from '@dnd-kit/core';
import {useDraggable, useDroppable} from '@dnd-kit/core';
import {CSS} from '@dnd-kit/utilities';
import axios from 'axios';

function App() {
  // This matrix doesn't contain actual elements, just information about what the circuit looks like.
  const [circuit, setCircuit] = useState([["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"]]);
  // Initializing this because it complains about type otherwise, there is probably a better way to do it.
  // TODO have it show something before user has modified circuit
  const [states, setStates] = useState([{"step":0, "state":[]},{"step":1, "state":[]},{"step":2, "state":[]}]);
  const [timeStep, setTimeStep] = useState(0);

  // TODO implement setCircuit (aka add + and - buttons).

  return (
    <div className="App">
      <DndContext onDragEnd={handleDragEnd}>
        <Toolbar />
        <Circuitboard />
        <ProbDiagram />
      </DndContext>
    </div>
  );

  function Toolbar(){
    return (
    <div className='Toolbar'>
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
      <TimeStepButtons />
    </div>)
  }
  
  function ProbDiagram(){
    return (
      <div>
        {states[timeStep].state.map(thing => <p>{JSON.stringify(thing)}</p>)}
      </div>
    );
  }

  function TimeStepButtons(){
    return (
    <div className='timeStepButtons'>
      <button onClick = {() => setTimeStep(0)}>ᴪ_0</button>
      <button onClick = {() => setTimeStep(1)}>ᴪ_1</button>
      <button onClick = {() => setTimeStep(2)}>ᴪ_2</button>
    </div>);
  }

  function QubitLine(props:any){
    return (
    <div className='qubitLine'>
      <h2>|0⟩</h2>
      <hr/>
      {// This generates a qubit line element from the 'circuit' matrix.
      }
      <div className='slot-container'>
        {circuit[Number(props.id)].map((gate, index) => <Slot gateType = {gate} id = {props.id + index.toString()}/>)}
      </div>
    </div>);
  }
  
  function Gate(props:any) {
    const {attributes, listeners, setNodeRef, transform} = useDraggable({
        id: props.name,
      });
      const style = {
        transform: CSS.Translate.toString(transform),
        width: 70,
        height: 70
        
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
    if(over === null){
      return;
    }
    
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
    sendCircuit(newCircuit);
  }

  async function sendCircuit(circuitToSend:any) {
    const response = await axios.post('http://localhost:8000/simulate',
        {circuit_matrix: circuitToSend})
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
