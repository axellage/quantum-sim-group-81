import React, { useState, ReactNode, useEffect } from 'react';
import './App.css';
import './circuitboard.css';
import './toolbar.css';
import Toolbar from './toolbar';
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

  
  
  function Circuitboard(){
    const [qubitLines, setQubitLines] = useState<ReactNode[]>([]);
    console.log(states);
    useEffect(() => {
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
          {//TODO create records for gateTypes and their corresponding names
          }
          {circuitLine.map((gate, index) => <Slot name={gate} gateType={gate} id={`${qubitLineId}${index}`} key={`${qubitLineId}${index}`} />)}
        </div>
      </div>
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

    /*if(props.gateType == "."){
      console.log("Placed control gate");
      return (
        <div ref={setNodeRef} style={style}>
          <PlacedControlGate name = {props.gateType} />
        </div>
      );
    }else {
      console.log("Placed other gate")
      return (
        <div ref={setNodeRef} style={style}>
          <PlacedGate name = {props.gateType} />
        </div>
      );
    }*/
      return (
        <div ref={setNodeRef} style={style}>
          <PlacedGate name = {props.name} gateType = {props.gateType}/>
        </div>
      );
  }

  function handleDragEnd(event:any){
    const {active, over} = event;
    console.log(over.id[0]);
    if(active.id == "C_down"){
      if(over.id[0] == 5){
        alert("No gate to control.");
        return;
      }
      if(circuit[parseInt(over.id[0]) + 1][parseInt(over.id[1])] == "I"){
        alert("No gate to control.");
        return;
      }
    }

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
    console.log("Sending circuit: " + convertToOldVersion(circuit));
    const response = await axios.post('http://localhost:8000/simulate',
        {circuit_matrix: convertToOldVersion(circuit)})
  .then(function(response: any){
    console.log(response);
    setStates(response.data.state_list);
  })}

  function convertToOldVersion(newCircuit:any){
    for(let i = 0; i < newCircuit.length - 1; i++){
      for(let j = 0; j < newCircuit[0].length; j++){
        if(newCircuit[i][j] == "C_down"){
          newCircuit[i][j] = "CNOT-1";
          newCircuit[i + 1][j] = "CNOT-2";
          //newCircuit = swapMatrixItem(newCircuit, i + 1, j, "CNOT-2")
        }
      }
    }
    return newCircuit;
  }

  /*function swapMatrixItem(matrix:string[][], y:number, x:number, newItem:string){
    const newMatrix = matrix.map((line, i) => {
      if(i === y) {
        return (line.map((gate, j) => {
          if(j === x){
            return (newItem);
          } else{
            return (gate);
          }
        }));
      } else {
        return line;
      } 
    });
  }*/
}

function PlacedGate(props:any){
  
  // Display nothing if there is no placed gate (which is the same as the identity gate).
  if(props.gateType != "I"){
    return (
      <button className = "placedGate">
        <h1>{props.name}</h1>
      </button>
    );
  } 
  else return null;
  
  
}

/*function PlacedControlGate(props:any, event:any){
  
  // Display nothing if there is no placed gate (which is the same as the identity gate).
  //const {active, over} = event;

  
  if(props.name == "."){
    return (
      <button className = "placedGate">
        <h1>{props.name}</h1>
      </button>
    );
  } else return null;
  
  
}*/



export default App;
