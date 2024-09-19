import "./App.css";
import Header from "./components/Header";
import Footer from "./components/Footer";
import ActionList from "./components/ActionList";

function App() {
  return (
    <main className="min-h-screen min-w-full bg-white text-black dark:text-white dark:bg-black grid grid-rows-main p-4">
      <Header title="UNWRAPPED" />
      <ActionList />
      <Footer />
    </main>
  );
}

export default App;
