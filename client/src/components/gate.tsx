import React from 'react';
import {useDraggable} from '@dnd-kit/core';
import {CSS} from '@dnd-kit/utilities';

export function Gate(props:any) {
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