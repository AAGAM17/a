import React from 'react';
import ReactDOM from 'react-dom/client';

function App() {
  return (
    <div className="app">
      <header>
        <h1>Hello from A Package Manager</h1>
        <p>This app was set up using the blazingly fast A package manager</p>
      </header>
      <main>
        <p>Start editing this file to see your changes</p>
      </main>
    </div>
  );
}

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
