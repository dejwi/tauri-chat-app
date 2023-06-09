import { appWindow } from "@tauri-apps/api/window";
import { useEffect } from "react";
import { ChatLogEntry, Message, User, useDataStore } from "../utils/data-store";

export const useTauriListeners = () => {
  const {
    userConnectedAction,
    addMessage,
    setOnlineUsers,
    userDisconnectedAction,
    setChatLog,
    setId
}= useDataStore();

  useEffect(() => {
    const unlisten1 = appWindow.listen("user-connected", (event) => {
      const user = event.payload as User;
      if (user) userConnectedAction(user);
    });

    const unlisten2 = appWindow.listen("received-message", (event) => {
      const mess = event.payload as Message;
      if (mess) addMessage(mess);
    });

    const unlisten3 = appWindow.listen("set-online-users", (event) => {
      const users = event.payload as User[];
      if (users) setOnlineUsers(users);
    });

    const unlisten4 = appWindow.listen("user-disconnected", (event) => {
      const user = event.payload as User;
      if (user) userDisconnectedAction(user);
    });

    const unlisten5 = appWindow.listen("set-chat-log", (event) => {
      const chatLog = event.payload as ChatLogEntry[];
      if (chatLog) setChatLog(chatLog);
    });

    const unlisten6 = appWindow.listen("client-id", (event) => {
      const id = event.payload as string;
      if (id) setId(id);
    });

    return () => {
      unlisten1.then((f) => f());
      unlisten2.then((f) => f());
      unlisten3.then((f) => f());
      unlisten4.then((f) => f());
      unlisten5.then((f) => f());
      unlisten6.then((f) => f());
    };
  }, []);
}