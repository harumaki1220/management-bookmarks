"use client";

import { useState } from "react";

export default function Home() {
  const [isGlobalStealth, setIsGlobalStealth] = useState(true);
  return (
    <main className="p-8 font-sans">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-xl font-bold">Document Manager</h1>
        <button
          onClick={() => setIsGlobalStealth(!isGlobalStealth)}
          className="text-xs text-gray-400 hover:text-gray-600"
        >
          {isGlobalStealth ? "Enter Debug Mode" : "Exit Debug Mode"}
        </button>
      </div>

      <ul className="space-y-4">
        <li className="p-4 border border-gray-200 rounded shadow-sm hover:bg-gray-50">
          <p className="text-blue-600 font-medium">
            {isGlobalStealth ? "20251225_システム構成図_v1.pdf" : "https://~~~"}
          </p>
        </li>
      </ul>
    </main>
  );
}
