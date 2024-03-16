import React, { useState, ReactNode, useEffect } from 'react';
import './App.css';
import './circuitboard.css';
import './toolbar.css';
import Toolbar from './toolbar';
import Slot from './slot';
import {DndContext} from '@dnd-kit/core';
import {useDraggable, useDroppable} from '@dnd-kit/core';
import {CSS} from '@dnd-kit/utilities';
import axios from 'axios';
import Circuitboard from './circuitboard';

function App() {
  // This matrix doesn't contain actual elements, just information about what the circuit looks like.
  const [circuit, setCircuit] = useState([["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"]]);
  // Initializing this because it complains about type otherwise, there is probably a better way to do it.

  // TODO implement setCircuit (aka add + and - buttons).

  return (
    <div className="App">
      <DndContext onDragEnd={handleDragEnd}>
        <Toolbar />
        <Circuitboard {...circuit}/> {/*shallow copy of circuit to circuitboard, solve for it to be in circuitboard later*/}
      </DndContext>
    </div>
  );
  

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
