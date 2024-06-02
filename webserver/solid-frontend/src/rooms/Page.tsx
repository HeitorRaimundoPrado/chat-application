import { For, Switch, Match, createResource, type Component, createSignal } from "solid-js";
import { A } from "@solidjs/router";
import { createClient } from "@supabase/supabase-js"

const supabase = createClient(import.meta.env.VITE_SUPABASE_URL!, import.meta.env.VITE_SUPABASE_KEY!)

const fetchRooms = async () => {
  const res = await fetch("/api/rooms/get", {
    headers: {
      "Authorization": `Bearer ${(await supabase.auth.getSession()).data.session?.access_token!}`
    }
  });
  return await res.json();
}

interface IRoom {
  id: number,
  title: string
}


const Page: Component = () => {
  const [rooms] = createResource<IRoom[]>(fetchRooms);
  const [newRoomTitle, setNewRoomTitle] = createSignal("");

  const handleNewRoom = async () => {
    fetch("/api/room/create", {
      method: "POST",
      body: JSON.stringify({
        title: newRoomTitle(),
        owner_id: (await supabase.auth.getUser())?.data?.user?.id
      }),
      headers: {
        "Authorization": `Bearer ${(await supabase.auth.getSession()).data.session?.access_token!}`,
        "Content-Type": "application/json"
      }
    })
  }

  return (
    <div>
      <Switch>
        <Match when={rooms.loading}>
          <h1>loading...</h1>
        </Match>
        <Match when={rooms.error}>
          <div>
            <h1>Error fetching rooms!</h1>
          </div>
        </Match>
        <Match when={rooms() !== undefined}>
          <ul>
            <For each={rooms()}>{
              (room, _idx) => {
                return <li><A href={`/room/${room.id}`}>{room.title}</A></li>
              }
            }</For>
          </ul>
        </Match>
      </Switch>
      <input type="text" onChange={e => setNewRoomTitle(e.target.value)} />
      <button onClick={handleNewRoom}>Create new room</button>
    </div>
  )
}
export default Page;
