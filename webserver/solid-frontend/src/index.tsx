/* @refresh reload */
import { render } from 'solid-js/web';

import './index.css';
import { Router } from '@solidjs/router';

import Home from "./Home"
import Chat from './chat/Page'
import Rooms from './rooms/Page'

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

const routes = [{
  path: "/",
  component: Home,
},
{
  path: "/chat",
  component: Chat
},
{
  path: "/rooms",
  component: Rooms
}]

render(() => <Router>{routes}</Router >, root!);
