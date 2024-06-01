import { type Component } from 'solid-js';
import { A } from '@solidjs/router';
import { createClient } from '@supabase/supabase-js';

const supabase = createClient(import.meta.env.VITE_SUPABASE_URL!, import.meta.env.VITE_SUPABASE_KEY!)

const App: Component = () => {
  const handleSignUp = () => {

  }
  const handleLogin = () => {

  }

  const handleSSO = async () => {
    const { data, error } = await supabase.auth.signInWithOAuth({
      provider: "google"
    })

    console.log(data)
  }

  const handleGetUser = async () => {
    console.log(await supabase.auth.getSession())
    fetch("/api/test-token", {
      headers: {
        "Authorization": `Bearer ${(await supabase.auth.getSession()).data.session?.access_token!}`
      }
    });
  }

  return (
    <div>
      <button onClick={handleSignUp}>Sign Up</button>
      <button onClick={handleLogin}>Login</button>
      <button onClick={handleSSO}>SignIn with Google</button>
      <button onClick={handleGetUser}>get user</button>
      <A href="/chat">Chat</A>
    </div>
  );
};

export default App;
