import clsx from "clsx";
import { User } from "../utils/data-store";

interface Props {
  online?: boolean;
  user: User;
  chatImage?: boolean;
  size: "small" | "chat";
}

const Avatar = ({ online, size, user, chatImage }: Props) => {
  return (
    <div
      className={clsx(
        "avatar  ",
        online && "online",
        !user.avatar_url && "placeholder",
        chatImage && "chat-image"
      )}
    >
      <div
        className={clsx(
          "bg-neutral-focus text-neutral-content rounded-full text-xs",
          size === "small" && "w-8",
          size === "chat" && "w-10"
        )}
      >
        {user.avatar_url ? (
          <img src={user.avatar_url} />
        ) : (
          <span>{user.username?.slice(0, 2)}</span>
        )}
      </div>
    </div>
  );
};

export default Avatar;
