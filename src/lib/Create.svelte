<script lang="ts">
    import { Input } from "./components/input";
    import { invoke } from "@tauri-apps/api/tauri"
  
    let name = "";
    let greetMsg = ""
  
    async function greet(){
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      greetMsg = await invoke("greet", { name })
    }
  </script>
  
  <div>
    <Input class="text-wrap"/>
  </div>