import React, { useState, ReactNode, useEffect } from 'react';
import './App.css';
import './circuitboard.css';
import './toolbar.css';
import Toolbar from './toolbar';
import {DndContext} from '@dnd-kit/core';
import {useDraggable, useDroppable} from '@dnd-kit/core';
import {CSS} from '@dnd-kit/utilities';
import axios from 'axios';

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

export default PlacedGate;