"use client";

import { Check, Copy } from "lucide-react";
import { useState } from "react";
import styles from "./install-command.module.css";

export function InstallCommand({ command }: Readonly<{ command: string }>) {
  const [copied, setCopied] = useState(false);

  async function copy() {
    await navigator.clipboard.writeText(command);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1200);
  }

  return (
    <div className={styles.wrap}>
      <code className={styles.command}>{command}</code>
      <button
        className={styles.button}
        type="button"
        onClick={copy}
        aria-label="Copy install command"
      >
        {copied ? <Check size={17} /> : <Copy size={17} />}
      </button>
    </div>
  );
}
