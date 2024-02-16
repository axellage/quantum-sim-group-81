import React from 'react';
import './App.css';
import {DndContext} from '@dnd-kit/core';
import {useDraggable, useDroppable} from '@dnd-kit/core';
import {CSS} from '@dnd-kit/utilities';

function App() {
  return (
    <div className="App">
      <DndContext onDragEnd={handleDragEnd}>
        <Toolbar />
        <Circuitboard />
      </DndContext>
    </div>
  );
}

function Toolbar(){
  return (<Gate name="X"/>);
}

function Circuitboard(){
  return(
  <div>
    <QubitLine id="0"/>
    <QubitLine id="1"/>
    <QubitLine id="2"/>
  </div>)
}

function QubitLine(props:any){
  return (
  <div className='qubitLine'>
    <h2>|0⟩</h2>
    <Slot id = {props.id + "0"}/>
    <Slot id = {props.id + "1"}/>
    <Slot id = {props.id + "2"}/>
    <Slot id = {props.id + "3"}/>
  </div>);
}

function Gate(props:any) {
  const {attributes, listeners, setNodeRef, transform} = useDraggable({
      id: props.name,
    });
    const style = {
      transform: CSS.Translate.toString(transform),
      width: 60,
      height: 60
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
    opacity: isOver ? 0.2 : 0.1,
    backgroundColor: 'white',
    width: 70,
    height: 70
  };

  return (
    <div ref={setNodeRef} style={style}>
      {props.children}
    </div>
  );
}

function handleDragEnd(){
  return null;
}

export default App;
