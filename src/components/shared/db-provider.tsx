"use client";
import { FC, ReactNode, useEffect, useState } from "react";
import { useLocalStorage } from "usehooks-ts";
import { invoke } from "@tauri-apps/api/tauri";

interface DbProviderProps {
  children: ReactNode;
}

const DbProvider: FC<DbProviderProps> = ({ children }) => {
  const [dbReady, setDbReady] = useLocalStorage("db-ready", false);

  useEffect(() => {
    if (!dbReady) {
      invoke<string>("create_tables", {})
        .then(() => {
          console.log("tables created");
          setDbReady(true);
        })
        .catch(console.error);
    }
  }, []);

  return <>{children}</>;
};

export default DbProvider;
