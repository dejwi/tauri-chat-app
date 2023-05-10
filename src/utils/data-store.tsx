import { invoke } from "@tauri-apps/api/tauri";
import { create } from "zustand";
import { combine } from "zustand/middleware";
import { appWindow } from "@tauri-apps/api/window";

export interface User {
  id: string;
  username: string;
  avatar_url: string;
}

export interface Message {
  user: User;
  content: string;
}

export interface ChatLogEntry {
  message?: Message;
  connected?: User;
  disconnected?: User;
}

export const useDataStore = create(
  combine(
    {
      id: "",
      username: "",
      avatarUrl: "",
      port: 8080,
      isConnected: false,
      isHosting: false,
      chatLog: [] as ChatLogEntry[],
      onlineUsers: [] as User[],
      newMessageCallback: () => {},
    },
    (set, get) => ({
      setUsername: (username: string) => set({ username }),
      setAvatarUrl: (url: string) => set({ avatarUrl: url }),
      setPort: (port: number) => set({ port }),
      connect: async () => {
        const state = get();
        if (!state.username || !state.port) return;

        set({ isConnected: true });
        invoke("client_connect", {
          username: state.username,
          port: state.port,
          avatarUrl: state.avatarUrl,
        })
          .catch((e) => {
            console.error(`Error connecting to the server: ${e}`);
            set({ isConnected: false });
          })
          .then(() => set({ isConnected: false }));
      },
      host: () => {
        const state = get();
        if (state.isHosting || !state.port) return;

        set({ isHosting: true });
        invoke("host_server", { port: state.port })
          .catch((e) => {
            console.error(`Error hosting server: ${e}`);
            set({ isHosting: false });
          })
          .then(() => set({ isHosting: false }));
      },
      userConnectedAction: (user: User) => {
        set((state) => ({
          onlineUsers: [...state.onlineUsers, user],
          chatLog: [...state.chatLog, { connected: user }],
        }));
        get().newMessageCallback();
      },
      setOnlineUsers: (users: User[]) => set({ onlineUsers: users }),
      userDisconnectedAction: (user: User) => {
        set((state) => ({
          onlineUsers: state.onlineUsers.filter((usr) => usr.id !== user.id),
          chatLog: [...state.chatLog, { disconnected: user }],
        }));
        get().newMessageCallback();
      },
      sendMessage: (content: string) => {
        appWindow.emit("send-message", content).catch((e) => {
          console.error("Failed to pass message ", e);
        });
      },
      addMessage: (message: Message) => {
        set((state) => ({ chatLog: [...state.chatLog, { message }] }));
        get().newMessageCallback();
      },
      setChatLog: (chatLog: ChatLogEntry[]) => {
        set({ chatLog });
        get().newMessageCallback();
      },
      setNewMessageCallback: (cb: () => void) =>
        set({ newMessageCallback: cb }),
      setId: (id: string) => set({ id }),
    })
  )
);
