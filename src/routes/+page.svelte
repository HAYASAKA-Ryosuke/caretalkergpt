<script>
  import { appWindow, PhysicalPosition } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/tauri";
  import { getClient, Body, ResponseType } from '@tauri-apps/api/http';

  let text = '';
  let responseText = '';
  /**
   * @type {{ role: string; content: string; }[]}
   */
  let messages = [];
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
    const message = {
        role: "user",
        content: text
    };
    messages.push(message);
    const client = await getClient();
    const body = Body.json({
      model: "gpt-3.5-turbo",
      messages: messages,
    });
    const headers = {
      headers: {Authorization: `Bearer ${API_KEY}`},
      responseType: ResponseType.JSON,
    };
    const response = await client.post('https://api.openai.com/v1/chat/completions',
      body,
      headers
    );
    if (response.ok) {
      responseText = response.data.choices[0].message.content;
      messages.push(response.data.choices[0].message);
    }
    console.log(messages)
  }
</script>

<div data-tauri-drag-region class="container">
  <div data-tauri-drag-region class="characterMessageBox">
    <img src="/charactor.png" height="100px" width="100px" />
    <p class="talk">{responseText}</p>
  </div>
  <div data-tauri-drag-region class="sendMessageBox">
    <input bind:value={text}>
    <button on:click={send}>send</button>
  </div>
</div>

<style>

.container {
  flex-direction: column;
  display: flex;
  background-size: 100%
}

.sendMessageBox {
  align-items: stretch;
  margin: 10px;
  padding: 10px;
}

.characterMessageBox {
  display: flex;
  align-items: center;
}

.talk {
  position: relative;
  display: inline-block;
  margin: 1.5em 0 1.5em 15px;
  padding: 15px 10px;
  min-width: 120px;
  min-height: 50px;
  max-width: 100%;
  color: #555;
  font-size: 16px;
  background: #e0edff;
  box-sizing: border-box;
  border-radius: 15px;
}

.talk:before {
  content: "";
  position: absolute;
  top: 50%;
  left: -29px;
  margin-top: -15px;
  border: 15px solid transparent;
  border-right: 15px solid #e0edff;
}
.talk p {
  margin: 0;
  padding: 0;
}
</style>