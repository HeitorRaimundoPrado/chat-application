import { type Component } from 'solid-js';
import { A } from '@solidjs/router';



const App: Component = () => {
  const handleSignUp = () => {

  }
  const handleLogin = () => {

  }

  return (
    <div>
      <button onClick={handleSignUp}>Sign Up</button>
      <button onClick={handleLogin}>Login</button>
      <A href="/chat">Chat</A>
    </div>
  );
};

export default App;
