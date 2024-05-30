import { createResource, Show, Switch, Match, type Component, createEffect } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

const fetchHelloWorld = async () => {
  const res = await fetch("/api/hello/world");
  return res.text();
}
const App: Component = () => {
  const [data] = createResource(fetchHelloWorld)

  createEffect(() => {
    console.log(data())
  })

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <img src={logo} class={styles.logo} alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          class={styles.link}
          href="https://github.com/solidjs/solid"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn Solid
        </a>
      </header>
      <Show when={data.loading}>
        <p>Loading...</p>
      </Show>
      <Switch>
        <Match when={data.error}>
          <span>Error </span>
        </Match>
        <Match when={data()}>
          <div>{data()}</div>
        </Match>
      </Switch>
    </div>
  );
};

export default App;
