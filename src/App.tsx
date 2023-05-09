import ConnectOrHostPage from "./pages/connect-or-host";
import { useDataStore } from "./utils/data-store";

function App() {
  const isConnected = useDataStore((state) => state.isConnected);

  if (!isConnected) return <ConnectOrHostPage />;

  return null;
}

export default App;
