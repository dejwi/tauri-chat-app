import { useEffect, useRef, useState } from "react";
import { User, useDataStore } from "../utils/data-store";
import Avatar from "../components/avatar";
import { ChatMessage, ChatUserConnection } from "../components/chat-log";
import { motion, AnimatePresence } from "framer-motion";

const ChatPage = () => {
  const {
    onlineUsers,
    sendMessage,
    chatLog,
    setNewMessageCallback,
    userConnectedAction,
    userDisconnectedAction,
    id: clientId,
  } = useDataStore();
  const [message, setMessage] = useState("");
  const chatLogRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setNewMessageCallback(() => {
      if (!chatLogRef.current) return;

      setTimeout(
        () =>
          chatLogRef.current?.scrollTo({
            left: 0,
            top: chatLogRef.current.scrollHeight,
            behavior: "smooth",
          }),
        100
      );
    });
  }, []);

  const onSendMessage = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!message) return;
    setMessage("");
    sendMessage(message);
  };

  return (
    <div className="h-screen grid grid-cols-[1fr_max-content]">
      <div className="flex flex-col overflow-hidden">
        <div
          className="flex-1 m-2 flex flex-col gap-2 overflow-y-auto !mb-0 overflow-x-hidden"
          ref={chatLogRef}
        >
          {chatLog.map((entry, i) =>
            entry.message ? (
              <ChatMessage
                key={"chat-mess" + entry.message.user.id + i}
                own={entry.message.user.id === clientId}
                message={entry.message}
              />
            ) : (
              <ChatUserConnection
                key={
                  "chat-usr-conn" +
                  entry.connected?.id +
                  entry.disconnected?.id +
                  i
                }
                user={
                  (entry?.connected as User) || (entry?.disconnected as User)
                }
                type={entry?.connected ? "connected" : "disconnected"}
              />
            )
          )}
        </div>

        <form className="flex gap-1 m-2" onSubmit={onSendMessage}>
          <input
            type="text"
            placeholder="Message"
            className="input input-bordered flex-1"
            value={message}
            onChange={(e) => setMessage(e.target.value)}
          />
          <button
            onClick={() => {}}
            className="btn btn-active btn-primary"
            type="submit"
          >
            Send
          </button>
        </form>
      </div>

      <div className="border-l border-base-300 px-6 pt-3 max-w-sm flex flex-col gap-1">
        <AnimatePresence mode="popLayout">
          {onlineUsers.map((usr) => (
            <motion.div
              key={"onlineusr" + usr.id}
              className="flex gap-2  items-center"
              initial={{ scale: 0.6, y: -30, opacity: 0.5 }}
              animate={{ scale: 1, y: 0, opacity: 1 }}
              exit={{ scale: 0.6, opacity: 0, x: 40 }}
            >
              <Avatar size="small" online user={usr} />
              <span>{usr.username}</span>
            </motion.div>
          ))}
        </AnimatePresence>
      </div>
    </div>
  );
};

export default ChatPage;
