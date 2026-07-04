import type { Metadata } from "next";
import styles from "./layout.module.css";

export const metadata: Metadata = {
  title: {
    default: "gruffed docs",
    template: "%s | gruffed docs",
  },
  description: "API reference and usage documentation for gruffed.",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en" className={styles.document}>
      <body className={styles.body}>{children}</body>
    </html>
  );
}
