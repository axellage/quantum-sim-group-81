import React from 'react';
import './toolbar.css';
import Gate from './gate';

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