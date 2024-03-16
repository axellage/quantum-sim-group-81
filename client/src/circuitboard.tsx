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

function Circuitboard(circuit: string[][]){
    const [states, setStates] = useState([{"step":0, "state":[]}]);
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
    }, [circuit]); // Circuit dependency array to make it only update when circuit is changed
  
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

    function States() {
        return (
          <section className="states">
            {states.map((timeStep) => (
            <h2>{JSON.stringify(timeStep.state)}</h2>
            ))}
          </section>
        );
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

    return(
    <div>
      <section className="circuit">
        {qubitLines}
      </section>
      {/*<button onClick={addQubit}>+</button>
      <button onClick={removeQubit}>-</button>*/}
      <button onClick={sendCircuit}>send circuit</button>
      <States />
    </div>)
  }
  export default Circuitboard;