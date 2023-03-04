<script>
  import Greet from "$lib/Greet.svelte";
  import Menu from "$lib/menu.svelte";
  import MenuOption from "$lib/menuOption.svelte";
  import { move_window, Position } from "tauri-plugin-positioner-api";
  import { appWindow, currentMonitor, LogicalPosition, PhysicalPosition } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/tauri";

  let showMenu = false;
  let pos = { x: 0, y: 0 };

  async function handleMousemove(event) {
    let pos = await invoke('get_cursor_position')
    if (pos) {
      let x = pos[0]
      let y = pos[1]
      appWindow.setPosition(new PhysicalPosition(x, y))
    }
	}
  function closeMenu() {
		showMenu = false;
	}
</script>

<div on:dragend={handleMousemove}>
  <img src="/vite.svg" width="100px" />
</div>

{#if showMenu}
	<Menu {...pos} on:click={closeMenu} on:clickoutside={closeMenu}>
		<MenuOption 
			on:click={console.log} 
			text="close" />
	</Menu>
{/if}

<!--<svelte:body on:contextmenu|preventDefault={onRightClick} />-->
<style>
</style>