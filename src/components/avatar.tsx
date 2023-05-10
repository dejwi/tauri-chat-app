import { cva, VariantProps } from "class-variance-authority";
import clsx from "clsx";
import { User } from "../utils/data-store";

const avatarCva = cva("bg-neutral-focus text-neutral-content rounded-full", {
  variants: {
    size: {
      small: "text-xs w-8",
      chat: "text-xs w-10",
    },
  },
});

interface Props extends VariantProps<typeof avatarCva> {
  online?: boolean;
  user: User;
  chatImage?: boolean;
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
      <div className={avatarCva({ size })}>
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
