import React from 'react';
import {useDraggable} from '@dnd-kit/core';
import {CSS} from '@dnd-kit/utilities';

export function Gate(props:any) {
    const {attributes, listeners, setNodeRef, transform} = useDraggable({
        id: 'unique-id',
      });
      const style = {
        transform: CSS.Translate.toString(transform),
      };
      
      return (
        <button ref={setNodeRef} style={style} {...listeners} {...attributes}>
          <h1>{props.name}</h1>
        </button>
      );
}