import React from 'react';
import logo from './logo.svg';
import './App.css';
import Wallet from './components/Wallet';

function App() {
  const heading = 'Simple BDK wallet using WASM bindings';

  return (
    <div className="App bg-orange-100">
      <div className="flex text-center">
        <div className="flex-1">
          <h1 className='text-3xl font-bold text-orange-900'>{heading}</h1>
        </div>
      </div>
      <div className="flex p-8 mb-8 h-screen">
        <div className="flex-none w-32 bg-orange-200">
        </div>
        <div className="flex-1 w-64">
          <Wallet />
        </div>
        <div className="flex-initial w-32 bg-orange-200">
        </div>
      </div>
    </div>
  );
}

export default App;
