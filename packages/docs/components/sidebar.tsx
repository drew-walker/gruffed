import * as NavigationMenu from "@radix-ui/react-navigation-menu";
import { Braces } from "lucide-react";
import Link from "next/link";
import { navGroups } from "../lib/nav";
import { Search } from "./search";
import styles from "./sidebar.module.css";

export function Sidebar() {
  return (
    <div className={styles.wrap}>
      <Link className={styles.brand} href="/docs/cli">
        <span className={styles.mark} aria-hidden="true">
          <Braces size={16} strokeWidth={2.4} />
        </span>
        gruffed docs
      </Link>
      <Search />
      <NavigationMenu.Root orientation="vertical" className={styles.navRoot}>
        <NavigationMenu.List className={styles.list}>
          {navGroups.map((group) => (
            <NavigationMenu.Item key={group.title} className={styles.group}>
              <div className={styles.groupTitle}>{group.title}</div>
              {group.items.map((item) => (
                <NavigationMenu.Link asChild key={item.href}>
                  <Link className={styles.link} href={item.href}>
                    <span className={styles.linkTitle}>{item.title}</span>
                    <span className={styles.summary}>{item.summary}</span>
                  </Link>
                </NavigationMenu.Link>
              ))}
            </NavigationMenu.Item>
          ))}
        </NavigationMenu.List>
      </NavigationMenu.Root>
    </div>
  );
}
