// https://vike.dev/Head

import logoUrl from "../assets/logo.svg";

export default function HeadDefault() {
  return (
    <>
      <meta charset="utf-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <meta name="theme-color" content="#000000" />
      <link rel="icon" href={logoUrl} />
      <script innerHTML={`
      document.documentElement.classList.toggle("dark", localStorage.theme === "dark" || (!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark)").matches),);
      `} />
    </>
  );
}
