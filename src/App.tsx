import ConnectOrHostPage from "./pages/connect-or-host";
import { useDataStore } from "./utils/data-store";
import ChatPage from "./pages/chat";
import { useTauriListeners } from "./hooks/use-tauri-listeners";

function App() {
  useTauriListeners();
  const isConnected = useDataStore((state) => state.isConnected);

  if (!isConnected) return <ConnectOrHostPage />;

  return <ChatPage />;
}

export default App;
