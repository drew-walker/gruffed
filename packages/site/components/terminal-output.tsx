"use client";

import { Check, Copy, Terminal } from "lucide-react";
import { useState } from "react";
import styles from "./terminal-output.module.css";

export function TerminalOutput({
  label,
  output,
}: Readonly<{
  label: string;
  output: string;
}>) {
  const [copied, setCopied] = useState(false);

  async function copy() {
    await navigator.clipboard.writeText(output.trimEnd());
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1200);
  }

  return (
    <section className={styles.terminal}>
      <div className={styles.bar}>
        <span className={styles.label}>
          <Terminal size={16} aria-hidden="true" />
          {label}
        </span>
        <button className={styles.copy} type="button" onClick={copy} aria-label="Copy terminal output">
          {copied ? <Check size={16} /> : <Copy size={16} />}
        </button>
      </div>
      <pre className={styles.pre}>{output}</pre>
    </section>
  );
}
