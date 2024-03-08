import React, { useState, ReactNode, useEffect } from 'react';
import './App.css';
import './circuitboard.css';
import './toolbar.css';
import Gate from './gate';
import {DndContext} from '@dnd-kit/core';
import {useDraggable, useDroppable} from '@dnd-kit/core';
import {CSS} from '@dnd-kit/utilities';
import axios from 'axios';

function Toolbar(){
    return (
    <div className='Toolbar'>
      <Gate name="X" id = "X"/>
      <Gate name="Y" id = "Y"/>
      <Gate name="." id = "C_down"/>
      <Gate name="Z" id = "Z"/>
      <Gate name="H" id = "H"/>
    </div>
    );
  }

export default Toolbar;