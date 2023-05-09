import { invoke } from "@tauri-apps/api/tauri";
import { useDataStore } from "../utils/data-store";

const ConnectOrHostPage = () => {
  const {
    avatarUrl,
    port,
    setAvatarUrl,
    setPort,
    setUsername,
    username,
    host,
    test_connect,
  } = useDataStore(
    ({
      avatarUrl,
      setAvatarUrl,
      username,
      setUsername,
      port,
      setPort,
      host,
      test_connect,
    }) => ({
      avatarUrl,
      setAvatarUrl,
      username,
      setUsername,
      port,
      setPort,
      host,
      test_connect,
    })
  );

  return (
    <div className="flex flex-col justify-center h-screen w-[17rem] m-[0_auto]">
      <div className="flex flex-col gap-1">
        <input
          type="text"
          placeholder="Name"
          className="input input-bordered"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
        <input
          type="text"
          placeholder="Avatar Url"
          className="input input-bordered"
          value={avatarUrl}
          onChange={(e) => setAvatarUrl(e.target.value)}
        />
        <input
          type="number"
          min="1"
          max="65535"
          placeholder="Port"
          value={port}
          onChange={(e) => {
            const val = +e.target.value;
            if (!isNaN(val)) setPort(val);
          }}
          className="input input-bordered"
        />
      </div>

      <div className="flex mt-4">
        <button
          onClick={test_connect}
          className="btn btn-active btn-primary flex-1"
        >
          Join
        </button>
        <div className="divider divider-horizontal">OR</div>
        <button onClick={host} className="btn btn-active">
          Host
        </button>
      </div>
    </div>
  );
};

export default ConnectOrHostPage;
