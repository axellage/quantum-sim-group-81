import React, { useState, ReactNode, useEffect } from 'react';
import './App.css';
import './circuitboard.css';
import './toolbar.css';
import Toolbar from './toolbar';
import PlacedGate from './placedGate';
import {DndContext} from '@dnd-kit/core';
import {useDraggable, useDroppable} from '@dnd-kit/core';
import {CSS} from '@dnd-kit/utilities';
import axios from 'axios';

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

export default Slot;