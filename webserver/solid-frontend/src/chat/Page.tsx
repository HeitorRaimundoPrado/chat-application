import { For, createSignal, onMount, onCleanup, type Component } from "solid-js";

const Chat: Component = () => {
  const [message, setMessage] = createSignal<string>("")
  const [messages, setMessages] = createSignal<string[]>([])

  const handleSendMessage = (e: SubmitEvent) => {
    e.preventDefault();

    fetch("/api/message/send", {
      method: "POST",
      body: JSON.stringify({
        content: message(),
      }),
      headers: {
        "Content-Type": "application/json"
      }
    })
  }

  const events = new EventSource("/api/message/listen");

  const handleMessage = (ev: MessageEvent) => {
    console.log(ev.data)
    setMessages([...messages(), ev.data])
  }

  onMount(() => {
    events.addEventListener("message", handleMessage);
  })

  onCleanup(() => {
    events.removeEventListener("message", handleMessage)
  })

  return (
    <div>

      <form onSubmit={handleSendMessage}>

        <ul>
          <For each={messages()}>{
            (message, _idx) => (
              <li><p>{message}</p></li>
            )
          }</For>
        </ul>

        <input type="text" value={message()} onChange={e => setMessage(e.target.value)} />
        <button>Send</button>
      </form>
    </div>
  )
}

export default Chat;
