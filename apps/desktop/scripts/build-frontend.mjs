import { cp, mkdir } from "node:fs/promises";
import { build } from "esbuild";

await mkdir("dist", { recursive: true });
await cp("src/index.html", "dist/index.html");
await build({
  bundle: true,
  entryPoints: ["src/main.js"],
  format: "esm",
  outfile: "dist/main.js"
});
