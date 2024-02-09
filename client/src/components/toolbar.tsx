import React, {useState} from "react";
import './toolbar.css';
import {Gate} from './gate';
import {DndContext} from '@dnd-kit/core';

function Toolbar() {
    const [parent, setParent] = useState(null);
    return (
      <div className="toolbar">
        <section className="gate-container">
          <DndContext onDragEnd={handleDragEnd}>
            <Gate name="X"/>
          </DndContext>
          <DndContext onDragEnd={handleDragEnd}>
            <Gate name="Y"/>
          </DndContext>
          <DndContext onDragEnd={handleDragEnd}>
            <Gate name="Z"/>
          </DndContext>
          <DndContext onDragEnd={handleDragEnd}>
            <Gate name="H"/>
          </DndContext>
        </section>
      </div>
    );

    function handleDragEnd({over}:any) {
      setParent(over ? over.id : null);
    }
  }

  
  export default Toolbar;