"use client";

type Bookmark = {
  id: string;
  url: string; // 本物のURL
  realTitle: string; // 本物のタイトル
  realFavicon?: string; // 本物のFavicon
  fakeTitle: string; // 偽装タイトル
  fakeType: "pdf" | "excel" | "folder" | "image"; // 偽装アイコンの種類
};

const DUMMY_BOOKMARKS: Bookmark[] = [
  {
    id: "1",
    url: "https://developer.mozilla.org/ja/",
    realTitle: "MDN Web Docs (JavaScript)",
    fakeTitle: "2025年度_システム要件定義書.xlsx",
    fakeType: "excel",
  },
  {
    id: "2",
    url: "https://doc.rust-lang.org/book/",
    realTitle: "The Rust Programming Language",
    fakeTitle: "Q1_進捗報告資料_v2.pdf",
    fakeType: "pdf",
  },
  {
    id: "3",
    url: "https://github.com/",
    realTitle: "GitHub: Let's build from here",
    fakeTitle: "社内共有フォルダ_2024",
    fakeType: "folder",
  },
];

import { useState } from "react";

export default function Home() {
  const [isGlobalStealth, setIsGlobalStealth] = useState(true);

  const handleLinkClick = (bookmark: Bookmark) => {
    if (isGlobalStealth) {
      alert(
        `エラー: '${bookmark.fakeTitle}' を開けません。\n権限が不足しています。`
      );
      return;
    }
    window.open(bookmark.url, "_blank");
  };

  return (
    <main className="p-8 font-sans max-w-2xl mx-auto">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-xl font-bold text-gray-800">Document Manager</h1>
        <button
          onClick={() => setIsGlobalStealth(!isGlobalStealth)}
          className={`text-xs px-3 py-1 rounded border ${
            isGlobalStealth
              ? "bg-gray-100 text-gray-500 border-gray-300"
              : "bg-red-50 text-red-500 border-red-200"
          }`}
        >
          {isGlobalStealth ? "Debug Mode: OFF" : "Debug Mode: ON"}
        </button>
      </div>

      <ul className="space-y-3">
        {DUMMY_BOOKMARKS.map((bookmark) => (
          <li
            key={bookmark.id}
            onClick={() => handleLinkClick(bookmark)}
            className="p-4 border border-gray-200 rounded bg-white shadow-sm hover:bg-gray-50 cursor-pointer transition-colors"
          >
            <div className="flex items-center gap-3">
              {/* アイコン部分（今は文字で代用） */}
              <span className="text-xl">
                {isGlobalStealth
                  ? bookmark.fakeType === "pdf"
                    ? "📄"
                    : "📊"
                  : "🔗"}
              </span>

              <div className="flex flex-col">
                <span
                  className={`font-medium ${
                    isGlobalStealth ? "text-gray-700" : "text-blue-600"
                  }`}
                >
                  {isGlobalStealth ? bookmark.fakeTitle : bookmark.realTitle}
                </span>

                {/* リアルモードの時だけURLを表示（デバッグ用） */}
                {!isGlobalStealth && (
                  <span className="text-xs text-gray-400">{bookmark.url}</span>
                )}
              </div>
            </div>
          </li>
        ))}
      </ul>
    </main>
  );
}
