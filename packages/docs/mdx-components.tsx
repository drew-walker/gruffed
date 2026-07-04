import type { ComponentType, ReactNode } from "react";
import { CodeBlock } from "./components/code-block";
import styles from "./mdx.module.css";

type MdxComponentProps = {
  children?: ReactNode;
};

type MdxComponents = Record<string, ComponentType<MdxComponentProps>>;

export function useMDXComponents(components: MdxComponents): MdxComponents {
  return {
    wrapper: ({ children }) => <article className={styles.article}>{children}</article>,
    pre: ({ children }) => <CodeBlock>{children}</CodeBlock>,
    ...components,
  };
}
