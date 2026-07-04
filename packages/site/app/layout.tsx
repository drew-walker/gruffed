import type { Metadata } from "next";
import { Analytics } from "@vercel/analytics/next";
import { Footer } from "../components/footer";
import { Nav } from "../components/nav";
import styles from "./layout.module.css";

export const metadata: Metadata = {
  title: {
    default: "gruffed",
    template: "%s | gruffed",
  },
  description: "Fast module-graph analysis for JavaScript and TypeScript codebases.",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en" className={styles.document}>
      <body className={styles.body}>
        <div className={styles.page}>
          <Nav />
          {children}
          <Footer />
        </div>
        <Analytics />
      </body>
    </html>
  );
}
