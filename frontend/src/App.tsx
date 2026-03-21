import { invoke } from "@tauri-apps/api/core";

function App() {
  const handleLogin = async () => {
    try {
      const token = await invoke<string>("login");
      console.log("Access token:", token);
    } catch (e) {
      console.error("Login failed:", e);
    }
  };

  return (
    <div>
      <button onClick={handleLogin}>Login with Spotify</button>
    </div>
  );
}

export default App;