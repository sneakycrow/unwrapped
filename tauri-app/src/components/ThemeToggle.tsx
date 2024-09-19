import React, { useState, useEffect } from "react";

const ThemeToggle = () => {
  const [isDarkMode, setIsDarkMode] = useState(false);

  useEffect(() => {
    // @ts-expect-error
    document.body.classList.toggle("dark");
  }, [isDarkMode]);

  const toggleTheme = () => {
    setIsDarkMode(!isDarkMode);
  };

  return (
    <button onClick={toggleTheme} className="min-w-8 flex items-center">
      <span className="mr-2">{isDarkMode ? "Dark" : "Light"} Mode</span>
      <div className="inline-flex items-center mr-2 grow-0">
        <div
          className={`w-4 h-4 rounded-full border-2 ${isDarkMode ? "bg-white border-white" : "bg-black border-black"}`}
        >
          <div
            className={`w-2 h-2 rounded-full m-0.5 ${isDarkMode ? "bg-black" : "bg-white"}`}
          ></div>
        </div>
      </div>
    </button>
  );
};

export default ThemeToggle;
