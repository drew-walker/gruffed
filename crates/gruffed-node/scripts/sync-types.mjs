import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(__dirname, "..");

fs.copyFileSync(path.join(packageRoot, "types/index.d.ts"), path.join(packageRoot, "index.d.ts"));
