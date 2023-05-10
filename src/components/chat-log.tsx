import { Message, User } from "../utils/data-store";
import Avatar from "./avatar";
import clsx from "clsx";
import { motion } from "framer-motion";

interface UserConnectionProps {
  user: User;
  type: "connected" | "disconnected";
}
export const ChatUserConnection = ({ type, user }: UserConnectionProps) => (
  <motion.div
    className={clsx(
      "self-center py-1 px-4 rounded-sm flex items-center",
      type === "connected" && "bg-green-600",
      type === "disconnected" && "bg-red-400"
    )}
    initial={{ scale: 0.6, y: 30 }}
    animate={{ scale: 1, y: 0 }}
  >
    <Avatar user={user} size="small" />
    <span className="ml-1 text-white/75 ">{user.username}</span>
    <span className="ml-2 text-black/70 font-semibold">
      {type?.toUpperCase()}
    </span>
  </motion.div>
);

interface MessageProps {
  own: boolean;
  message: Message;
}
export const ChatMessage = ({
  message: { content, user },
  own,
}: MessageProps) => (
  <motion.div
    className={clsx("chat", own && "chat-end", !own && "chat-start")}
    initial={{ scale: 0.6, y: -30, x: own ? 150 : -150 }}
    animate={{ scale: 1, y: 0, x: 0 }}
    transition={{ duration: 0.17, bounce: 0.15 }}
  >
    <Avatar user={user} chatImage size="chat" />
    <div className="chat-header">{user.username}</div>
    <div className="chat-bubble">{content}</div>
  </motion.div>
);
