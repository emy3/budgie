import React from "react";
import "./App.css";
import { Income } from './components/Income/Income';
import { Expense } from './components/Expense/Expense';
import { Totals } from "./components/Totals/Totals";

function App() {

  return (
    <div className="container">
      <h1>Welcome to Budgie!</h1>
        <Income />
        <Expense />
        <Totals />
    </div>
  );
}

export default App;
