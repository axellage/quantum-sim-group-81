import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import Circuitboard from './circuitboard';

test('adds qubit line', () => {
  render(<Circuitboard />);
  const addButton = screen.getByText('+');
  
  fireEvent.click(addButton);

  const qubitLines = screen.getAllByTestId('qubit-line');
  expect(qubitLines.length).toBe(4); // Initially rendered 3 + clicked once = 4
});

test('stops at 10 qubit lines', () => {
  render(<Circuitboard />);
  const addButton = screen.getByText('+');
  
  for (let i = 0; i < 10; i++) {
    fireEvent.click(addButton);
  }

  const qubitLines = screen.getAllByTestId('qubit-line');
  expect(qubitLines.length).toBe(10); // 3 initially + 10 clicks = 13, but should stop at 10
});
