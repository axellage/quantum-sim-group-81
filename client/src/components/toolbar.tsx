import React from "react";
import './toolbar.css';

function Toolbar() {
    return (
      <div className="toolbar">
        <section className="gate-container">
            <h1>Gates</h1>
            <div className="gates">
                <div className="gate-object">X</div>
                <div className="gate-object">Y</div>
                <div className="gate-object">Z</div>
                <div className="gate-object">H</div>
            </div>
        </section>
      </div>
    );
  }
  
  export default Toolbar;