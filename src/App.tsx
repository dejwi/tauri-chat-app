import { appWindow } from "@tauri-apps/api/window";
import ConnectOrHostPage from "./pages/connect-or-host";
import { Message, User, useDataStore } from "./utils/data-store";
import { useEffect } from "react";
import ChatPage from "./pages/chat";

function App() {
  const [isConnected, addOnlineUser, addMessage] = useDataStore((state) => [
    state.isConnected,
    state.addOnlineUser,
    state.addMessage,
  ]);

  useEffect(() => {
    const unlisten1 = appWindow.listen("user-connected", (event) => {
      const user = event.payload as User;
      console.log("user connected: ", user);
      if (user) addOnlineUser(user);
    });

    const unlisten2 = appWindow.listen("received-message", (event) => {
      const mess = event.payload as Message;
      console.log("received message: ", mess);
      if (mess) addMessage(mess);
    });

    return () => {
      unlisten1.then((f) => f());
      unlisten2.then((f) => f());
    };
  }, []);

  if (!isConnected) return <ConnectOrHostPage />;

  return <ChatPage />;
}

export default App;
