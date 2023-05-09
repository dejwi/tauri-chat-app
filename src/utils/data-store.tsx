import { invoke } from "@tauri-apps/api/tauri";
import { create } from "zustand";
import { combine } from "zustand/middleware";

export const useDataStore = create(
  combine(
    {
      username: "",
      avatarUrl: "",
      port: 8080,
      isConnected: false,
      isHosting: false,
    },
    (set) => ({
      setUsername: (username: string) => set({ username }),
      setAvatarUrl: (url: string) => set({ avatarUrl: url }),
      setPort: (port: number) => set({ port }),
      connect: () => set({ isConnected: true }),
      test_connect: () =>
        set(({ username, port, avatarUrl }) => {
          invoke("test_client_connect", {
            username,
            port,
            avatarUrl,
          }).catch((e) => console.error(`Error connect to server: ${e}`));

          return {};
        }),
      host: () =>
        set((state) => {
          if (state.isHosting || !state.port) return {};
          invoke("host_server", { port: state.port });

          return { isHosting: true };
        }),
    })
  )
);
