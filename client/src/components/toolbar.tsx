import React, {useMemo, useState} from "react";
import './toolbar.css';
import {Gate} from './gate';
import {DndContext, useDroppable} from '@dnd-kit/core';
import {
  createSnapModifier,
  restrictToHorizontalAxis,
} from '@dnd-kit/modifiers';

function Droppable(props:any) {
  const {isOver, setNodeRef} = useDroppable({
    id: props.id,
  });
  const style = {
    opacity: isOver ? 1 : 0.5,
  };

  return (
    <div ref={setNodeRef} style={style}>
      {props.children}
    </div>
  );
}

function Toolbar() {
    const [parentH, setParentH] = useState(null);
    const [parentX, setParentX] = useState(null);
    const [parentY, setParentY] = useState(null);
    const [parentZ, setParentZ] = useState(null);
    const [gridSize, setGridSize] = React.useState(60);
    const snapToGrid = useMemo(() => createSnapModifier(gridSize), [gridSize]);
    const gateX = <Gate name="X"/>;
    const gateY = <Gate name="Y"/>;
    const gateZ = <Gate name="Z"/>;
    const gateH = <Gate name="H"/>;

    return (
      <div className="toolbar">
        <section className="gate-container">
          <DndContext onDragEnd={handleDragEnd}>
            {!parentX ? gateX : null}
            {!parentY ? gateY : null}
            {!parentZ ? gateZ : null}
            {!parentH ? gateH : null}
            <Droppable id="droppable1">
              {// TODO: If statement for if it's empty, in that case display "[]". 
              // Aka parentX != droppable1 && parentY != droppable1 etc
}
              {parentX === "droppable1" ? gateX : '.'}
              {parentY === "droppable1" ? gateY : '.'}
              {parentZ === "droppable1" ? gateZ : '.'}
              {parentH === "droppable1" ? gateH : '.'}
            </Droppable>
          </DndContext>
        </section>
      </div>
    );

    function handleDragEnd(event:any) {
      // TODO limit number of draggables in droppable to 1.
      // probably by creating an isEmpty constant
      const {active, over} = event;
      if (active.id == 'X'){
        setParentX(over ? over.id : null);
      } else if(active.id == 'Y'){
        setParentY(over ? over.id : null);
      } else if(active.id == 'Z'){
        setParentZ(over ? over.id : null);
      } else if(active.id == 'H'){
        setParentH(over ? over.id : null);
      }
    }
  }

  
  export default Toolbar;