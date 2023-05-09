import { invoke } from "@tauri-apps/api/tauri";
import { create } from "zustand";
import { combine } from "zustand/middleware";
import { appWindow } from "@tauri-apps/api/window";

export interface User {
  id: string;
  username: string;
  avatarUrl: string;
}

export interface Message {
  user: User;
  content: string;
}

export const useDataStore = create(
  combine(
    {
      username: "",
      avatarUrl: "",
      port: 8080,
      isConnected: false,
      isHosting: false,
      messages: [] as Message[],
      onlineUsers: [] as User[],
    },
    (set, get) => ({
      setUsername: (username: string) => set({ username }),
      setAvatarUrl: (url: string) => set({ avatarUrl: url }),
      setPort: (port: number) => set({ port }),
      connect: async (user: Omit<User, "id">, port: number) => {
        set({ isConnected: true });
        invoke("client_connect", {
          username: user.username,
          port,
          avatarUrl: user.avatarUrl,
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
        invoke("host_server", { port: state.port }).catch((e) => {
          console.error(`Error hosting server: ${e}`);
          set({ isHosting: false });
        });
      },
      addOnlineUser: (user: User) =>
        set((state) => ({ onlineUsers: [...state.onlineUsers, user] })),
      sendMessage: (content: string) => {
        appWindow.emit("send-message", content).catch((e) => {
          console.error("Failed to pass message ", e);
        });
      },
      addMessage: (data: Message) =>
        set((state) => ({ messages: [...state.messages, data] })),
    })
  )
);
