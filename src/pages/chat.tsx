import { useState } from "react";
import { useDataStore } from "../utils/data-store";

const ChatPage = () => {
  const { onlineUsers, sendMessage, messages } = useDataStore();
  const [message, setMessage] = useState("");

  return (
    <div className="flex flex-col justify-center h-screen w-[17rem] m-[0_auto]">
      <div className="flex flex-col gap-1">
        <h2>Chat</h2>
        {onlineUsers.map((usr) => (
          <span key={"usr-list" + usr.id}>
            {usr.username} {usr.id}
          </span>
        ))}
        <div className="divider" />
        {messages.map((mess, i) => (
          <div key={"mess" + mess.user.id + i}>
            {mess.user.username}: {mess.content}
          </div>
        ))}
        <div className="divider" />
        <input
          type="text"
          placeholder="Message"
          className="input input-bordered"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
        />
        <button
          onClick={() => {
            setMessage("");
            sendMessage(message);
          }}
          className="btn btn-active btn-primary flex-1"
        >
          Send
        </button>
      </div>
    </div>
  );
};

export default ChatPage;
