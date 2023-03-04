<script>
  import { appWindow, PhysicalPosition } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/tauri";
  import { getClient, Body, ResponseType } from '@tauri-apps/api/http';

  let text = '';
  let responseText = '';
  let API_KEY = 'API_KEY'

  async function handleMousemove(event) {
    let pos = await invoke('get_cursor_position');
    if (pos) {
      let x = pos[0];
      let y = pos[1];
      appWindow.setPosition(new PhysicalPosition(x, y));
    }
	}

  async function send() {
    const client = await getClient();
    const body = Body.json({
      model: "gpt-3.5-turbo",
      messages: [{
        role: "user",
        content: text
      }]
    });
    const headers = {
      headers: {Authorization: `Bearer ${API_KEY}`},
      responseType: ResponseType.JSON,
    };
    const response = await client.post('https://api.openai.com/v1/chat/completions',
      body,
      headers
    );
    console.log(response.data.choices[0].message.content);
    responseText = response.data.choices[0].message.content;
  }
</script>

<div on:dragend={handleMousemove}>
  <p>{responseText}</p>
  <img src="/vite.svg" width="100px" />
  <input bind:value={text}>
  <button on:click={send}>send</button>
</div>

<style>
</style>