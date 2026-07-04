"use client";

import { Check, Copy } from "lucide-react";
import { Children, isValidElement, useMemo, useState } from "react";
import styles from "./code-block.module.css";

function textFromNode(node: React.ReactNode): string {
  if (typeof node === "string" || typeof node === "number") {
    return String(node);
  }
  if (Array.isArray(node)) {
    return node.map(textFromNode).join("");
  }
  if (isValidElement<{ children?: React.ReactNode }>(node)) {
    return textFromNode(node.props.children);
  }
  return "";
}

export function CodeBlock({ children }: Readonly<{ children: React.ReactNode }>) {
  const [copied, setCopied] = useState(false);
  const text = useMemo(() => Children.toArray(children).map(textFromNode).join(""), [children]);

  async function copy() {
    await navigator.clipboard.writeText(text.trimEnd());
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1200);
  }

  return (
    <div className={styles.frame}>
      <button className={styles.copy} type="button" onClick={copy} aria-label="Copy code">
        {copied ? <Check size={16} /> : <Copy size={16} />}
      </button>
      <pre className={styles.pre}>{children}</pre>
    </div>
  );
}
