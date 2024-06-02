import { createResource, createEffect, Switch, Match, type Component } from 'solid-js';
import { A } from '@solidjs/router';
import { createClient } from '@supabase/supabase-js';
import { FaBrandsGoogle } from 'solid-icons/fa'

const supabase = createClient(import.meta.env.VITE_SUPABASE_URL!, import.meta.env.VITE_SUPABASE_KEY!)

async function getUser() {
  return await supabase.auth.getUser();
}

const App: Component = () => {
  const [user] = createResource(getUser);


  const handleSSO = async () => {
    const { data, error } = await supabase.auth.signInWithOAuth({
      provider: "google"
    })
  }

  const handleGetUser = async () => {
    fetch("/api/test-token", {
      headers: {
        "Authorization": `Bearer ${(await supabase.auth.getSession()).data.session?.access_token!}`
      }
    });
  }

  const handleLogout = async () => {
    const { error } = await supabase.auth.signOut();
  }

  return (
    <div>
      <Switch>
        <Match when={user.loading}>
          <div>loading...</div>
        </Match>
        <Match when={user()?.data.user === null}>
          <button onClick={handleSSO}>SignIn with Google <FaBrandsGoogle /></button>
        </Match>
        <Match when={user()?.data.user !== null && user()?.data.user !== undefined}>
          <A href="/rooms">Rooms</A>
          <button onClick={handleLogout}>Logout</button>
        </Match>
      </Switch>
    </div >
  );
};

export default App;
